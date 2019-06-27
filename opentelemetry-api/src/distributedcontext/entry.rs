use std::borrow::Cow;
use std::convert::Into;
use crate::internal::validate_and_convert_str;

pub struct Entry<'a> {
    pub key: EntryKey<'a>,
    pub value: EntryValue<'a>,
    pub metadata: EntryMetadata,
}

impl <'a> Entry<'a> {
    pub fn new(key: EntryKey<'a>, value: EntryValue<'a>, metadata: EntryMetadata) -> Self {
        Entry { key, value, metadata }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct EntryKey<'a>(Cow<'a, str>);

impl <'a> EntryKey<'a> {
    pub fn new<N: Into<Cow<'a, str>>>(name: N) -> Self {
        EntryKey(validate_and_convert_str(name))
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct EntryValue<'a>(Cow<'a, str>);

impl <'a> EntryValue<'a> {
    pub fn new<N: Into<Cow<'a, str>>>(name: N) -> Self {
        EntryValue(validate_and_convert_str(name))
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct EntryMetadata(EntryTtl);

impl EntryMetadata {
    pub fn new(ttl: EntryTtl) -> Self {
        EntryMetadata(ttl)
    }
}

///
/// `EntryTtl` is an integer that represents number of hops an entry can propagate.
///
/// Anytime a sender serializes a entry, sends it over the wire and receiver deserializes the
/// entry then the entry is considered to have travelled one hop.
///
/// There could be one or more proxy(ies) between sender and receiver. Proxies are treated as
/// transparent entities and they are not counted as hops.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum EntryTtl {
    NoPropagation,
    Propagation(usize),
    UnlimitedPropagation,
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;
    proptest! {
        #[test]
        #[should_panic]
        fn test_invalid_entry_key(s in "[^[:ascii:]]{1, 255}") {
            EntryKey::new(s);
        }

        #[test]
        #[should_panic]
        fn test_invalid_entry_key_len(s in "[[:ascii:]]{256, 3000}") {
            EntryKey::new(s);
        }
    }
}