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

use std::mem;
use rand::Rng;

const INVALID: TraceId = TraceId(0);

/// Represents a trace identifier.
///
/// A valid trace identifier is a 16-byte array with at least one non-zero byte.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TraceId(u128);

impl From<[u8; 16]> for TraceId {
    fn from(v: [u8; 16]) -> Self {
        TraceId(u128::from_be_bytes(v))
    }
}

impl TraceId {
    /// Returns the size in bytes of the `TraceId`.
    pub const fn get_size() -> usize {
        mem::size_of::<Self>()
    }

    /// Returns the invalid `TraceId`. All bytes are '\0'.
    pub const fn get_invalid() -> TraceId {
        INVALID
    }

    /// Generates a new random `TraceId`.
    pub fn generate_random_id(rng: &mut impl Rng) -> TraceId {
        TraceId(rng.gen_range(1u128, std::u128::MAX))
    }

    /// Returns a `TraceId` whose representation is copied from `src`
    pub fn from_bytes(src: [u8; 16]) -> TraceId {
        TraceId(u128::from_be_bytes(src))
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

    /// Returns whether the `TraceId` is valid.
    /// A valid trace identifier is a 16-byte array with at least one non-zero byte.
    pub fn is_valid(&self) -> bool {
        *self == INVALID
    }

    /// Returns the lowercase base16 encoding of this {@code TraceId}.
    pub fn as_hex(&self) -> String {
        format!("{:x}", self.0)
    }
}
