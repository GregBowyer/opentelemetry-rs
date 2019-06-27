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

use bitflags::bitflags;
use std::mem;

bitflags! {
    /// Represents global trace options.
    ///
    /// These options are propagated to all child `Span`.
    /// These determine features such as whether a `Span` should be traced.
    /// It is implemented as a bitmask.
    pub struct TraceOptions: u8 {
        const DEFAULT_OPTIONS = 0b00000000;
        const IS_SAMPLED = 0b00000001;
    }
}

impl TraceOptions {
    pub const fn get_size() -> usize {
        mem::size_of::<Self>()
    }
}

impl Default for TraceOptions {
    fn default() -> Self {
        TraceOptions::DEFAULT_OPTIONS
    }
}
