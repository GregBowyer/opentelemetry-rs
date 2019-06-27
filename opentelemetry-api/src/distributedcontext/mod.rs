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

pub mod entry;
pub use entry::{Entry, EntryValue, EntryKey, EntryMetadata};

/// A map from `EntryKey` to `EntryValue` and `EntryMetadata` that can be used to
/// label anything that is associated with a specific operation.
///
/// For example, `DistributedContext`s can be used to label stats, log messages, or
/// debugging information.
pub trait DistributedContext<'a> {
    type Iter: Iterator<Item=&'a Entry<'a>>;

    /// Returns an iterator over the entries in this `DistributedContext`.
    fn iter(&self) -> Self::Iter;

    /// Returns the `EntryValue` associated with the given `EntryKey`.
    fn entry_value(&self, key: EntryKey) -> Option<&EntryValue>;
}

/*
/**
 * Builder for the {@link DistributedContext} class.
 *
 * @since 0.1.0
 */
interface Builder {
/**
 * Sets the parent {@link DistributedContext} to use. If not set, the value of {@link
 * DistributedContextManager#getCurrentContext()} at {@link #build()} time will be used as
 * parent.
 *
 * <p>This <b>must</b> be used to create a {@link DistributedContext} when manual Context
 * propagation is used.
 *
 * <p>If called multiple times, only the last specified value will be used.
 *
 * @param parent the {@link DistributedContext} used as parent.
 * @return this.
 * @throws NullPointerException if {@code parent} is {@code null}.
 * @see #setNoParent()
 * @since 0.1.0
 */
Builder setParent(DistributedContext parent);

/**
 * Sets the option to become a {@link DistributedContext} with no parent. If not set, the value
 * of {@link DistributedContextManager#getCurrentContext()} at {@link #build()} time will be
 * used as parent.
 *
 * @return this.
 * @since 0.1.0
 */
Builder setNoParent();

/**
 * Adds the key/value pair and metadata regardless of whether the key is present.
 *
 * @param key the {@code EntryKey} which will be set.
 * @param value the {@code EntryValue} to set for the given key.
 * @param entryMetadata the {@code EntryMetadata} associated with this {@link Entry}.
 * @return this
 * @since 0.1.0
 */
Builder put(EntryKey key, EntryValue value, EntryMetadata entryMetadata);

/**
 * Removes the key if it exists.
 *
 * @param key the {@code EntryKey} which will be removed.
 * @return this
 * @since 0.1.0
 */
Builder remove(EntryKey key);

/**
 * Creates a {@code DistributedContext} from this builder.
 *
 * @return a {@code DistributedContext} with the same entries as this builder.
 * @since 0.1.0
 */
DistributedContext build();
}
*/
