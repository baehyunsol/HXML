use crate::predicate::{is_hexadecimal, is_numeric};

pub fn into_v16(s: &str) -> Vec<u16> {
    String::from(s).encode_utf16().collect()
}

pub fn from_v16(v: &[u16]) -> String {

    if cfg!(test) {
        String::from_utf16(v).unwrap()
    }

    else {
        String::from_utf16_lossy(v)
    }

}

pub fn to_int_dec(s: &[u16]) -> Option<u32> {

    if s.len() == 0 {
        return None;
    }

    let mut result: u64 = 0;

    for c in s.iter() {

        if !is_numeric(c) {
            return None;
        }

        result *= 10;
        result += (c - '0' as u16) as u64;

        if result > u32::MAX as u64 {
            return None;
        }

    }

    Some(result as u32)
}

pub fn to_int_hex(s: &[u16]) -> Option<u32> {

    if s.len() == 0 {
        return None;
    }

    let mut result: u64 = 0;

    for c in s.iter() {

        if !is_hexadecimal(c) {
            return None;
        }

        result *= 16;

        if is_numeric(c) {
            result += (c - '0' as u16) as u64;
        }

        else if *c < 'Z' as u16 {
            result += (c - 'A' as u16) as u64 + 10;
        }

        else {
            result += (c - 'a' as u16) as u64 + 10;
        }

        if result > u32::MAX as u64 {
            return None;
        }

    }

    Some(result as u32)
}

pub fn to_lower(s: &[u16]) -> Vec<u16> {
    s.iter().map(
        |c|
        if 'A' as u16 <= *c && *c <= 'Z' as u16 {
            c + 32
        }
        else {
            *c
        }
    ).collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::*;

    #[test]
    fn decimal_test() {
        let test_cases = vec![
            ("123", Some(123)),
            ("0", Some(0)),
            ("2a", None),
            ("abc", None),
            ("999999999999999", None)
        ];

        let test_cases = test_cases.into_iter().map(
            |(case, answer)| (into_v16(case), answer)
        ).collect::<Vec<(Vec<u16>, Option<u32>)>>();

        for (case, answer) in test_cases.iter() {
            assert_eq!(&to_int_dec(case), answer);
        }

    }

    #[test]
    fn hexadecimal_test() {
        let test_cases = vec![
            ("123", Some(0x123)),
            ("0", Some(0)),
            ("2a", Some(0x2a)),
            ("aBc", Some(0xaBc)),
            ("NaN", None),
            ("999999999999999", None)
        ];

        let test_cases = test_cases.into_iter().map(
            |(case, answer)| (into_v16(case), answer)
        ).collect::<Vec<(Vec<u16>, Option<u32>)>>();

        for (case, answer) in test_cases.iter() {
            assert_eq!(&to_int_hex(case), answer);
        }

    }

}