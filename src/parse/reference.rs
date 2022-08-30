use super::get_name_end_index;
use crate::predicate::{is_numeric, is_hexadecimal, is_valid_char};
use crate::utils::{to_int_dec, to_int_hex};

// https://www.w3.org/TR/xml/#NT-Reference
// entity_reference | char_reference
pub fn get_reference_end_index(document: &[u16], index: usize) -> Option<usize> {

    match get_entity_reference_end_index(document, index) {
        Some(entity_reference_end_index) => Some(entity_reference_end_index),
        None => get_char_reference_end_index(document, index)
    }

}

// https://www.w3.org/TR/xml/#NT-EntityRef
// '&' name ';'
pub fn get_entity_reference_end_index(document: &[u16], index: usize) -> Option<usize> {

    if index + 2 >= document.len() || document[index] != '&' as u16 {
        return None;
    }

    match get_name_end_index(document, index + 1) {
        Some(name_end_index) if name_end_index + 1 < document.len() && document[name_end_index + 1] == ';' as u16 => Some(name_end_index + 1),
        _ => None
    }

}

// https://www.w3.org/TR/xml/#NT-CharRef
// '&#' [0-9]+ ';' | '&#x' [0-9a-fA-F]+ ';'
// TODO: a character referred to using char-ref must be a valid character
pub fn get_char_reference_end_index(document: &[u16], mut index: usize) -> Option<usize> {

    if index + 3 >= document.len() || document[index] != '&' as u16 || document[index + 1] != '#' as u16 {
        return None;
    }

    index += 2;

    if document[index] == 'x' as u16 && is_hexadecimal(&document[index + 1]) {
        index += 1;
        let num_begin_index = index;

        while index < document.len() && is_hexadecimal(&document[index]) {
            index += 1;
        }

        if index < document.len() && document[index] == ';' as u16 {
            let num_end_index = index;

            match to_int_hex(&document[num_begin_index..num_end_index]) {
                Some(n) if n <= u16::MAX as u32 && is_valid_char(&(n as u16)) => Some(index),
                _ => None
            }

        }

        else {
            None
        }

    }

    else if is_numeric(&document[index]) {
        let num_begin_index = index;

        while index < document.len() && is_numeric(&document[index]) {
            index += 1;
        }

        if index < document.len() && document[index] == ';' as u16 {
            let num_end_index = index;

            match to_int_dec(&document[num_begin_index..num_end_index]) {
                Some(n) if n <= u16::MAX as u32 && is_valid_char(&(n as u16)) => Some(index),
                _ => None
            }

        }

        else {
            None
        }

    }

    else {
        None
    }

}


#[cfg(test)]
mod tests {
    use crate::parse::*;
    use crate::testbench::*;

    #[test]
    fn entity_reference_test() {
        get_xxx_end_index(
            vec![
                ("not a entity_reference", None),
                ("&;", None),
                ("&#;", None),
                ("&#x;", None),
                ("&nbsp;", Some(5)),
                ("&nbsp", None),
                ("&#123;", None),
                ("&#123", None),
                ("&#x123;", None),
                ("&#x123", None),
                ("&#x10fa;", None),
                ("&#x10fa", None),
                ("&#x10xy;", None),
                ("&#x10xy", None),
                ("&#1234567890;", None),
                ("&#1234567890", None),
            ],
            get_entity_reference_end_index
        );
    }

    #[test]
    fn char_reference_test() {
        get_xxx_end_index(
            vec![
                ("not a char_reference", None),
                ("&;", None),
                ("&#;", None),
                ("&#x;", None),
                ("&nbsp;", None),
                ("&nbsp", None),
                ("&#123;", Some(5)),
                ("&#123", None),
                ("&#x123;", Some(6)),
                ("&#x123", None),
                ("&#x10fa;", Some(7)),
                ("&#x10fa", None),
                ("&#x10xy;", None),
                ("&#x10xy", None),
                ("&#1234567890;", None),
                ("&#1234567890", None),
            ],
            get_char_reference_end_index
        );
    }

}