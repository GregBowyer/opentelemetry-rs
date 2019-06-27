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

/// Represents all the possible values for an attribute.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum AttributeValue<'a> {
    String(Cow<'a, str>),
    Boolean(bool),
    Long(i64),
    Double(f64)
}

macro_rules! impl_from {
    ($what: ty, $variant: expr) => (
        impl <'a> From<$what> for AttributeValue<'a> {
            fn from(val: $what) -> Self {
                $variant(Cow::from(val))
            }
        }
    );

    ($what: ty, $conv: ty, $variant: expr) => (
        impl <'a> From<$what> for AttributeValue<'a> {
            fn from(val: $what) -> Self {
                $variant(val as $conv)
            }
        }
    );
}

impl_from!(i8, i64, AttributeValue::Long);
impl_from!(u8, i64, AttributeValue::Long);
impl_from!(i16, i64, AttributeValue::Long);
impl_from!(u16, i64, AttributeValue::Long);
impl_from!(i32, i64, AttributeValue::Long);
impl_from!(u32, i64, AttributeValue::Long);
impl_from!(i64, i64, AttributeValue::Long);
impl_from!(u64, i64, AttributeValue::Long);

impl_from!(f32, f64, AttributeValue::Double);
impl_from!(f64, f64, AttributeValue::Double);

impl_from!(String, AttributeValue::String);
impl_from!(&'a str, AttributeValue::String);
impl_from!(&'a String, AttributeValue::String);

impl_from!(bool, bool, AttributeValue::Boolean);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_str_from() {
        assert_eq!(AttributeValue::String("test".into()), "test".into());
        assert_eq!(AttributeValue::String("test".into()), "test".to_string().into())
    }
}

