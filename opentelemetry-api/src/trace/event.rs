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

use std::collections::HashMap;

use crate::trace::attribute_value::AttributeValue;

/// A text annotation with a set of attributes.
pub trait Event {
    /// Return the name of the `Event`.
    fn name(&self) -> &str;

    /// Return the attributes of the `Event`.
    fn attributes(&self) -> HashMap<&str, &AttributeValue>;
}
