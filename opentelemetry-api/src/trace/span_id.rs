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

/// Represents a span identifier.
/// A valid span identifier is an 8-byte array with at least one non-zero byte.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SpanId(u64);

const INVALID: SpanId = SpanId(0);

impl SpanId {
    /// Constructs a `SpanId` whose representation is specified by a long value.
    ///
    /// There is no restriction on the specified value, other than the already established validity
    /// rules applying to `SpanId`. Specifying 0 for this value will effectively make the new
    /// `SpanId` invalid.
    ///
    /// <p>This is equivalent to calling {@link #fromBytes(byte[], int)} with the specified value
    /// stored as big-endian.
    pub fn new(id: u64) -> Self {
        SpanId(id)
    }

    /// Returns the size in bytes of the {@code SpanId}.
    pub const fn get_size() -> usize {
        mem::size_of::<SpanId>()
    }

    /// Returns the invalid `SpanId. All bytes are 0.
    pub const fn invalid() -> SpanId {
        INVALID
    }

    /// Generates a new random `SpanId`.
    pub fn generate_random_id(rng: &mut impl Rng) -> SpanId {
        SpanId(rng.gen_range(1u64, std::u64::MAX))
    }

    /// Returns a `SpanId` whose representation is copied from `src`
    pub fn from_bytes(src: [u8; 8]) -> SpanId {
        SpanId(u64::from_be_bytes(src))
    }

    pub fn to_bytes(&self) -> [u8; 8] {
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
