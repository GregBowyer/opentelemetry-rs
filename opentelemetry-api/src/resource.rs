/*
 * Copyright 2019, OpenTelemetry Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//!
//! API for resource information population.
//!
//! The resources library primarily defines a type `Resource` that captures information about the
//! entity for which stats or traces are recorded.
//!
//! For example, metrics exposed by a Kubernetes container can be linked to a resource that
//! specifies the cluster, namespace, pod, and container name.
//!
//! Label keys, and label values *MUST* contain only printable ASCII (codes between 32 and 126,
//! inclusive) and less than 256 characters. Type and label keys MUST have a length greater than
//! zero. They SHOULD start with a domain name and separate hierarchies with / characters, e.g.
//! `k8s.io/namespace/name`.
use std::{
    borrow::Cow,
    collections::{HashMap, hash_map::Entry},
};

use crate::internal::validate_and_convert_str;

#[derive(Default)]
pub struct Resource<'a> {
    pub labels: HashMap<Cow<'a, str>, Cow<'a, str>>
}

impl <'a> Resource<'a> {
    fn new(labels: HashMap<Cow<'a, str>, Cow<'a, str>>) -> Self {
        Resource { labels }
    }

    /// Creates a new Resource out of the collection of labels
    ///
    /// # Panics
    /// If the following hold
    ///
    /// * The length of a key or value is _over_ 256 bytes
    /// * If a key or value contains none ascii chars
    pub fn create<K, V>(labels: HashMap<K, V>) -> Self
        where K: Into<Cow<'a, str>>, V: Into<Cow<'a, str>>
    {
        let labels = labels.into_iter()
            .map(|(k, v)| (validate_and_convert_str(k), validate_and_convert_str(v)))
            .collect();
        Resource::new(labels)
    }

    /// Creates a new Resource that is a combination of labels of two Resources.
    ///
    /// For example, from two Resources - one representing the host and one representing a container,
    /// resulting Resource will describe both.
    ///
    /// Already set labels *WILL NOT* be overwritten unless they are empty string.
    /// Label key name-spacing SHOULD be used to prevent collisions across different resource
    /// detection steps.
    pub fn merge(&mut self, other: Self) {
        other.labels
            .into_iter()
            .for_each(|(key, value)| {
                match self.labels.entry(key) {
                    Entry::Vacant(e) => { e.insert(value); },
                    Entry::Occupied(mut e) => {
                        if e.get() == "" {
                            e.insert(value);
                        }
                    }
                };
            });
    }

    pub fn labels(&self) -> HashMap<&str, &str> {
        self.labels
            .iter()
            .map(|(k, v)| (k.as_ref(), v.as_ref()))
            .collect()
    }

    /// Helper method to get values for given labels
    pub fn get(&self, label: &str) -> Option<&str> {
        self.labels.get(label).map(|x| x.as_ref())
    }

    pub fn empty() -> Self {
        Resource::default()
    }
}

/// Utility to make resource creation slightly simpler
///
/// Can be used in place of `Resource::new`
///
/// ## Example
///
/// ```
/// use opentelemetry_api::{Resource, resource};
///
/// let resource = resource!{
///     "label" => "value",
///     "other_label" => "other_value",
/// };
///
/// assert_eq!(resource.get("label").unwrap(), "value");
/// assert_eq!(resource.get("other_label").unwrap(), "other_value");
/// ```
#[macro_export]
macro_rules! resource {
    ($($key:expr => $value:expr,)+) => { resource!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            use ::std::borrow::Cow;
            let mut _map: ::std::collections::HashMap
                <
                    ::std::borrow::Cow<'_, str>,
                    ::std::borrow::Cow<'_, str>
                > = ::std::collections::HashMap::new();

            $(
                let _ = _map.insert(Cow::from($key), Cow::from($value));
            )*

            Resource::create(_map)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_create() {
        let mut map = HashMap::new();
        map.insert(Cow::Borrowed("test"), Cow::Borrowed("label"));
        let r = Resource::create(map);
        assert_eq!(r.labels.get("test"), Some(&Cow::Borrowed("label")));

        let mut map = HashMap::new();
        map.insert("test", "label");
        let r = Resource::create(map);
        assert_eq!(r.labels.get("test"), Some(&Cow::Borrowed("label")));

        let mut map = HashMap::new();
        map.insert("test", "label".to_string());
        let r = Resource::create(map);
        assert_eq!(r.labels.get("test"), Some(&Cow::Borrowed("label")));
    }

    #[test]
    fn test_create_macro() {
        let mut label2 = String::new();
        label2.push_str("label2");

        let r = resource! {
            "test" => "label",
            "test2" => label2,
        };

        assert_eq!(r.labels.get("test"), Some(&Cow::Borrowed("label")));
        assert_eq!(r.labels.get("test2"), Some(&Cow::Borrowed("label2")));
    }

    #[test]
    fn test_merge_resources() {
        let mut r1 = resource! {
            "test_1" => "val_1",
            "test_2" => "",
            "test_3" => "val_3",
        };

        let r2 = resource! {
            "test_1" => "should not merge",
            "test_2" => "some_val",
            "test_4" => "val_4",
        };

        r1.merge(r2);
        assert_eq!(r1.get("test_1").unwrap(), "val_1");
        assert_eq!(r1.get("test_2").unwrap(), "some_val");
        assert_eq!(r1.get("test_3").unwrap(), "val_3");
        assert_eq!(r1.get("test_4").unwrap(), "val_4");
    }

    #[test]
    fn test_labels() {
        let r1 = resource! {
            "test_1" => "val_1",
            "test_2" => "",
            "test_3" => "val_3",
        };

        let labels = r1.labels();
        assert_eq!(*labels.get("test_1").unwrap(), "val_1");
        assert_eq!(*labels.get("test_2").unwrap(), "");
        assert_eq!(*labels.get("test_3").unwrap(), "val_3");
    }

    proptest! {
        #[test]
        #[should_panic]
        fn test_invalid_resource_key_ascii(s in "[^[:ascii:]]{1, 255}") {
            resource!(s => "val")
        }

        #[test]
        #[should_panic]
        fn test_invalid_resource_key_len(s in "[[:ascii:]]{256, 3000}") {
            resource!(s => "val")
        }

        #[test]
        #[should_panic]
        fn test_invalud_resource_value_ascii(s in "[^[:ascii:]]{1, 255}") {
            resource!("key" => s)
        }

        #[test]
        #[should_panic]
        fn test_invalud_resource_value_len(s in "[[:ascii:]]{256, 3000}") {
            resource!("key" => s)
        }
    }
}