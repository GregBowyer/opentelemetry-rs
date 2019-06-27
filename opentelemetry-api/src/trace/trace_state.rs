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

use std::borrow::Cow;

/// Carries tracing-system specific context in a list of key-value pairs. TraceState allows different
/// vendors propagate additional information and inter-operate with their legacy Id formats.
///
/// Implementation is optimized for a small list of key-value pairs.
///
/// `Key` is opaque string up to 256 characters printable.
/// It MUST begin with a lowercase letter, and can only contain lowercase letters a-z, digits 0-9,
/// underscores _, dashes -, asterisks *, and forward slashes /.
///
/// `Value` is opaque string up to 256 characters printable ASCII RFC0020 characters (i.e., the
/// range 0x20 to 0x7E) except comma , and =.
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TraceState<'a> {
    pub entries: Vec<Entry<'a>>
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Entry<'a> {
    pub key: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

const MAX_KEY_LEN: usize = 255;
const MAX_VAL_LEN: usize = 255;
const MAX_KEY_VALUE_PAIRS: usize = 32;

impl <'a> TraceState<'a> {
    fn new(entries: Vec<Entry<'a>>) -> Self {
        assert!(entries.len() <= MAX_KEY_VALUE_PAIRS, "Invalid size");
        TraceState { entries }
    }

    /// Returns the value to which the specified key is mapped
    pub fn get(&self, key: &str) -> Option<&Entry> {
        self.entries.iter().find(|x| x.key == key)
    }

    /// Return this tracestate as a builder
    pub fn as_builder(&'a self) -> TraceStateBuilder<'a> {
        TraceStateBuilder {
            parent: Some(self),
            entries: None,
        }
    }
}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TraceStateBuilder<'a> {
    pub parent: Option<&'a TraceState<'a>>,
    pub entries: Option<Vec<Entry<'a>>>,
}

impl <'a> TraceStateBuilder<'a> {
    /// Adds or updates the `Entry` that has the given `key if it is present.
    ///
    /// The new `Entry` will always be added in the front of the list of entries.
    pub fn set<K, V>(mut self, key: K, value: V) -> Self
        where K: Into<Cow<'a, str>>,
              V: Into<Cow<'a, str>>
    {
        let mut entries = self.entries.get_or_insert(self.parent.map_or(vec![], |x| x.entries.clone()));
        let key = validate_key(key);
        let value = validate_value(value);
        entries.retain(|x| x.key != key);
        entries.insert(0, Entry { key, value });
        self
    }

    /// Removes the `Entry` that has the given `key` if it is present.
    pub fn remove<K: Into<Cow<'a, str>>>(mut self, key: K) -> Self {
        let key = validate_key(key);
        let mut entries = self.entries.get_or_insert(self.parent.map_or(vec![], |x| x.entries.clone()));
        entries.retain(|x| x.key != key);
        self
    }

    /// Returns a `Builder` based on an empty `Tracestate`.
    pub fn builder() -> Self {
        TraceStateBuilder::default()
    }

    /// Builds a TraceState by adding the entries to the parent in front of the key-value pairs list
    /// and removing duplicate entries.
    pub fn build(self) -> TraceState<'a> {
        match self.entries {
            None => TraceState::new(self.parent.map_or(vec![], |x| x.entries.clone())),
            Some(values) => TraceState::new(values),
        }
    }
}

// Key is opaque string up to 256 characters printable. It MUST begin with a lowercase letter, and
// can only contain lowercase letters a-z, digits 0-9, underscores _, dashes -, asterisks *, and
// forward slashes /.
fn validate_key<'a, N: Into<Cow<'a, str>>>(key: N) -> Cow<'a, str> {
    let key = key.into();
    assert!(key.len() <= MAX_KEY_LEN, "Should be an ASCII string not longer than {}", MAX_KEY_LEN);
    assert!(!key.is_empty(), "Key should not be empty");
    assert!(key.chars().nth(0).unwrap().is_ascii_lowercase(), "First char of key must be 'a'-'z'");
    assert!(key.chars().all(|c| {
        c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-' || c == '*' || c == '/'
    }), "Key cannot contain characters outside of {'a'-'z', '0'-'9', _, -, *, / }");
    key
}

fn validate_value<'a, V: Into<Cow<'a, str>>>(value: V) -> Cow<'a, str> {
    let value = value.into();
    assert!(value.len() <= MAX_VAL_LEN, "Should be an ASCII string not longer than {}", MAX_VAL_LEN);
    assert!(value.chars().all(|c| !c.is_ascii_control() || c != ',' || c != '='),
            "Value cannot contain none ascii chars, unprintable chars or ',' & '='");
    value
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Valid key alphabets should always work
        #[test]
        fn test_validate_key_correct_alphabet(s in "[a-z][a-z0-9_\\-*/]{0, 254}") {
            assert_eq!(validate_key(s.clone()), s)
        }

        /// Valid key alphabets that are too long should fail
        #[test]
        #[should_panic]
        fn test_validate_key_alphabets_too_long(s in "[a-z][a-z0-9_\\-*/]{255, 3000}") {
            validate_key(s)
        }

        /// Obviously invalid key alphabets should break
        #[test]
        #[should_panic]
        fn test_validate_key_incorrect_alphabet(s in "[^[a-z][a-z0-9_\\-*/]{0, 254}]") {
            validate_key(s)
        }

        /// Valid value alphabets should always work
        #[test]
        fn test_validate_value_correct_alphabet(s in "[[:ascii:]&&[^,=]]{0, 255}") {
            assert_eq!(validate_value(s.clone()), s)
        }

        /// Obviously invalid value alphabets should break
        #[test]
        #[should_panic]
        fn test_validate_value_alphabets_too_long(s in "[[:ascii:]&&[^,=]]{256, 3000}") {
            validate_value(s)
        }

        /// Obviously invalid value alphabets should break
        #[test]
        #[should_panic]
        fn test_validate_value_incorrect_alphabets(s in "[^[[:ascii:]&&[^,=]{1, 255}]") {
            validate_value(s)
        }
    }
}
