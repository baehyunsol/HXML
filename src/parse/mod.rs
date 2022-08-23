use crate::predicate::{is_name_char, is_name_start_char, is_whitespace, is_valid_char};
use crate::utils::into_v16;

mod tag;
pub use tag::*;

mod reference;
pub use reference::*;

mod attribute;
pub use attribute::*;

mod pi;
pub use pi::*;

mod prolog;
pub use prolog::*;

// get_xxx_end_index(content: &[u16], index: usize) -> Option<usize>
// if content[index..end_index + 1] is a valid `xxx`, it returns Some(end_index)
// otherwise, it returns None
// content[end_index] is the last character of `xxx`.
// `get_xxx_end_index` functions cannot parse recursively

// https://www.w3.org/TR/xml/#NT-Name
// name_start_char (name_char)*
pub fn get_name_end_index(content: &[u16], mut index: usize) -> Option<usize> {

    if index >= content.len() || !is_name_start_char(&content[index]) {
        return None;
    }

    index += 1;

    while index < content.len() && is_name_char(&content[index]) {
        index += 1;
    }

    Some(index - 1)
}

// https://www.w3.org/TR/xml/#NT-Eq
// whitespace? '=' whitespace?
pub fn get_eq_end_index(content: &[u16], mut index: usize) -> Option<usize> {

    if index >= content.len() {
        return None;
    }

    if is_whitespace(&content[index]) {
        index += 1;
    }

    if index >= content.len() || content[index] != '=' as u16 {
        None
    }

    else if index + 1 < content.len() && is_whitespace(&content[index + 1]) {
        Some(index + 1)
    }

    else {
        Some(index)
    }

}

// https://www.w3.org/TR/xml/#NT-content
// char_data? ((element | reference | cd_sect | processing_instruction | comment) char_data?)*
// The standard XML spec allows a 0-length content, but this parser does not
pub fn get_content_end_index(content: &[u16], index: usize) -> Option<usize> {

    // I won't be implementing `get_content_end_index` in this way
    // 'cuz it's recursive and the parser does not support recursive declaration in this way
    todo!()
}

// https://www.w3.org/TR/xml/#NT-CharData
// The standard XML spec allows a 0-length char_data, but this parser does not
// [^<&]* - ([^<&]* ']]>' [^<&]*)
pub fn get_char_data_end_index(content: &[u16], index: usize) -> Option<usize> {

    if index >= content.len() || content[index] == '<' as u16 || content[index] == '&' as u16 {
        return None;
    }

    let mut index = index + 1;

    loop {

        if index == content.len() {
            return Some(index - 1);
        }

        else if content[index] == '<' as u16 || content[index] == '&' as u16 {
            return Some(index - 1);
        }

        else if content[index] == ']' as u16 {

            if index + 2 < content.len() && content[index + 1] == ']' as u16 && content[index + 2] == '>' as u16  {
                return Some(index - 1);
            }

            else {
                index += 1;
            }

        }

        else {
            index += 1;
        }

    }

}

// https://www.w3.org/TR/xml/#NT-Comment
// '<!--' ((char - '-') | ('-' (char - '-')))* '-->'
pub fn get_comment_end_index(content: &[u16], mut index: usize) -> Option<usize> {

    if index + 3 >= content.len() || content[index] != '<' as u16 ||
    content[index + 1] != '!' as u16 || content[index + 2] != '-' as u16 ||
    content[index + 3] != '-' as u16 {
        return None;
    }

    index += 4;

    loop {

        if index + 2 >= content.len() || !is_valid_char(&content[index]) {
            return None;
        }

        if content[index] == '-' as u16 {

            if index + 2 < content.len() && content[index - 1] != '-' as u16 &&
            content[index + 1] == '-' as u16 && content[index + 2] == '>' as u16 {
                return Some(index + 2);
            }

        }

        index += 1;
    }

}

// https://www.w3.org/TR/xml/#NT-CDSect
// '<![CDATA[' (char* - (char* ']]>' char*)) ']]>'
pub fn get_cd_sect_end_index(content: &[u16], mut index: usize) -> Option<usize> {

    if index + 9 >= content.len() || &content[index..index + 9] != &into_v16("<![CDATA[") {
        return None;
    }

    index += 9;

    loop {

        if index + 2 >= content.len() || !is_valid_char(&content[index]) {
            return None;
        }

        if content[index] == ']' as u16 && content[index + 1] == ']' as u16 && content[index + 2] == '>' as u16 {
            return Some(index + 2);
        }

        index += 1;
    }

}

#[cfg(test)]
mod tests {
    use crate::parse::*;
    use crate::testbench::*;

    #[test]
    fn comment_test() {
        get_xxx_end_index(
            vec![
                ("not an comment", None),
                ("<!---->", None),
                ("<!-- -->", Some(7)),
                ("<!-- --->", None),
                ("<!-- declarations for <head> & <body> -->", Some(40)),
            ],
            get_comment_end_index
        );
    }

    #[test]
    fn char_data_test() {
        get_xxx_end_index(
            vec![
                ("char_data", Some(8)),
                ("<!-- not a char data -->", None),
            ],
            get_char_data_end_index
        );
    }

    #[test]
    fn cd_sect_test() {
        get_xxx_end_index(
            vec![
                ("not a cd_sect", None),
                ("<![CDATA[ foo ]]>", Some(16)),
                ("<!-- not a char data -->", None),
            ],
            get_cd_sect_end_index
        );
    }

}