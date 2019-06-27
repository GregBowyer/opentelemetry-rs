use std::borrow::Cow;
const MAX_LEN: usize = 255;

pub(crate) fn validate_and_convert_str<'a, N: Into<Cow<'a, str>>>(to_check: N) -> Cow<'a, str> {
    let to_ret = to_check.into();
    assert!(to_ret.len() < MAX_LEN, "Should be an ASCII string not longer than {}", MAX_LEN);
    let is_allowed = to_ret.chars().all(|x| !x.is_ascii_control() && x.is_ascii());
    assert!(is_allowed, "Should be an ASCII string, contains control or none ascii chars");
    to_ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        #[should_panic]
        fn test_internal_validate_str(s in "[^[:ascii:]]{1, 255}") {
            validate_and_convert_str(s)
        }

        #[test]
        #[should_panic]
        fn test_internal_validate_str_len(s in "[[:ascii:]]{256, 3000}") {
            validate_and_convert_str(s)
        }
    }
}