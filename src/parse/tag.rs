use super::{
    get_name_end_index, get_attribute_end_index,
    get_content_end_index, get_reference_end_index,
    get_comment_end_index, get_char_data_end_index,
    get_cd_sect_end_index,
    attribute::parse_attribute
};
use crate::predicate::is_whitespace;
use crate::node::{
    raw_element::{RawContent, RawElement},
    raise_error
};
use crate::gstring::GString;

// https://www.w3.org/TR/xml/#dt-element
// empty_element_tag | start_tag content end_tag
pub fn get_element_end_index(document: &[u16], index: usize) -> Option<usize> {

    if index >= document.len() {
        return None;
    }

    match get_empty_element_tag_end_index(document, index) {
        Some(end_index) => Some(end_index),
        None => match get_start_tag_end_index(document, index) {
            Some(start_tag_end_index) => match get_content_end_index(document, start_tag_end_index + 1) {
                Some(content_end_index) => get_end_tag_end_index(document, content_end_index + 1),

                // The standard XML spec allows a 0-length content, but this parser does not
                None => get_end_tag_end_index(document, start_tag_end_index + 1)
            },
            None => None
        }
    }

}

// https://www.w3.org/TR/xml/#NT-ETag
// '</' name whitespace? '>'
pub fn get_end_tag_end_index(document: &[u16], index: usize) -> Option<usize> {

    if index + 3 >= document.len() || !(document[index] == '<' as u16 && document[index + 1] == '/' as u16) {
        None
    }

    else {

        match get_name_end_index(document, index + 2) {
            Some(name_end_index) => if name_end_index + 1 >= document.len() {
                None
            } else if document[name_end_index + 1] == '>' as u16 {
                Some(name_end_index + 1)
            } else if is_whitespace(&document[name_end_index + 1]) {
                if name_end_index + 2 < document.len() && document[name_end_index + 2] == '>' as u16 {
                    Some(name_end_index + 2)
                } else {
                    None
                }
            } else {
                None
            },
            None => None
        }

    }

}

// https://www.w3.org/TR/xml/#NT-STag
// '<' name (whitespace attribute)* whitespace? '>'
pub fn get_start_tag_end_index(document: &[u16], index: usize) -> Option<usize> {

    if index + 2 >= document.len() || document[index] != '<' as u16 {
        None
    }

    else {

        match get_name_end_index(document, index + 1) {
            Some(name_end_index) => if name_end_index + 1 >= document.len() {
                None
            } else if is_whitespace(&document[name_end_index + 1]) {
                let mut curr_index = name_end_index + 1;
                let mut curr_char = ' ' as u16;

                loop {

                    if curr_char == ' ' as u16 {

                        if curr_index + 1 >= document.len() {
                            return None;
                        }

                        else if document[curr_index + 1] == '>' as u16 {
                            return Some(curr_index + 1);
                        }

                        match get_attribute_end_index(document, curr_index + 1) {
                            Some(attribute_end_index) => {
                                curr_char = 'a' as u16;
                                curr_index = attribute_end_index;
                            },
                            None => { return None; }
                        }

                    }

                    else if curr_char == 'a' as u16 {  // attribute
                        
                        if curr_index + 1 >= document.len() {
                            return None;
                        }

                        else if document[curr_index + 1] == '>' as u16 {
                            return Some(curr_index + 1);
                        }

                        else if is_whitespace(&document[curr_index + 1]) {
                            curr_char = ' ' as u16;
                            curr_index += 1;
                        }

                        else {
                            return None;
                        }

                    }

                }

            } else if document[name_end_index + 1] == '>' as u16 {
                Some(name_end_index + 1)
            } else {
                None
            },
            None => None
        }

    }

}

// https://www.w3.org/TR/xml/#NT-EmptyElemTag
// '<' name (whitespace attribute)* whitespace? '/>'
pub fn get_empty_element_tag_end_index(document: &[u16], index: usize) -> Option<usize> {

    if index >= document.len() {
        None
    }

    else if document[index] == '<' as u16 {

        match get_name_end_index(document, index + 1) {
            Some(name_end_index) => if name_end_index + 2 >= document.len() {
                None
            } else if is_whitespace(&document[name_end_index + 1]) {
                let mut curr_index = name_end_index + 1;
                let mut curr_char = ' ' as u16;

                loop {

                    if curr_char == ' ' as u16 {

                        if curr_index + 2 >= document.len() {
                            return None;
                        }

                        else if document[curr_index + 1] == '/' as u16 && document[curr_index + 2] == '>' as u16 {
                            return Some(curr_index + 2);
                        }

                        match get_attribute_end_index(document, curr_index + 1) {
                            Some(attribute_end_index) => {
                                curr_char = 'a' as u16;
                                curr_index = attribute_end_index;
                            },
                            None => { return None; }
                        }

                    }

                    else if curr_char == 'a' as u16 {  // attribute
                        
                        if curr_index + 2 >= document.len() {
                            return None;
                        }

                        else if document[curr_index + 1] == '/' as u16 && document[curr_index + 2] == '>' as u16 {
                            return Some(curr_index + 2);
                        }

                        else if is_whitespace(&document[curr_index + 1]) {
                            curr_char = ' ' as u16;
                            curr_index += 1;
                        }

                        else {
                            return None;
                        }

                    }

                }

            } else if document[name_end_index + 1] == '/' as u16 && document[name_end_index + 2] == '>' as u16 {
                Some(name_end_index + 2)
            } else {
                None
            },
            None => None
        }

    }

    else {
        None
    }

}

// it assumes that the tag is valid
pub fn parse_tag(document: &[u16], index: usize) -> ((GString, Vec<(GString, GString)>), usize) {  // ((name, Vec<(att_name, Vec<att_value>)>), end_index)

    let name_end_index = get_name_end_index(document, index + 1).unwrap();
    let name = GString::new(index + 1, name_end_index + 1);

    let mut attributes = vec![];

    let mut curr_index = name_end_index + 1;

    loop {

        if is_whitespace(&document[curr_index]) {
            curr_index += 1;
        }

        else if document[curr_index] == '/' as u16 {
            return ((name, attributes), curr_index + 1);
        }

        else if document[curr_index] == '>' as u16 {
            return ((name, attributes), curr_index)
        }

        // attribute
        else {
            let (curr_attribute, attribute_end_index) = parse_attribute(document, curr_index);
            attributes.push(curr_attribute);
            curr_index = attribute_end_index + 1;
        }

    }

}

pub fn parse_content(document: &[u16], index: usize) -> Option<(RawContent, usize)> {  // Option<(RawContent, end_index)>

    match parse_element(document, index) {
        Some((element, end_index)) => {
            return Some((RawContent::Element(element), end_index));
        },
        None => {}
    }

    match get_char_data_end_index(document, index) {
        Some(char_data_end_index) => {
            return Some((RawContent::CharData(GString::new(index, char_data_end_index + 1)), char_data_end_index));
        },
        None => {}
    }

    match get_cd_sect_end_index(document, index) {
        Some(cd_sect_end_index) => {
            return Some((RawContent::CDSect(GString::new(index + 9, cd_sect_end_index - 2)), cd_sect_end_index));
        },
        None => {}
    }

    match get_comment_end_index(document, index) {
        Some(comment_end_index) => {
            return Some((RawContent::Comment(GString::new(index + 4, comment_end_index - 2)), comment_end_index));
        },
        None => {}
    }

    match get_reference_end_index(document, index) {
        Some(reference_end_index) => {
            return Some((RawContent::Reference(GString::new(index + 1, reference_end_index)), reference_end_index));
        },
        None => {}
    }

    None
}

pub fn parse_element(document: &[u16], index: usize) -> Option<(RawElement, usize)> {  // Option<(RawElement, end_index)>

    match get_empty_element_tag_end_index(document, index) {
        Some(tag_end_index) => {
            let ((name, attributes), _) = parse_tag(document, index);

            return Some((RawElement::new(name, attributes, true, vec![]), tag_end_index));
        },
        None => {}
    }

    match get_start_tag_end_index(document, index) {
        Some(tag_end_index) => {
            let ((name, attributes), _) = parse_tag(document, index);
            let mut curr_index = tag_end_index + 1;
            let mut contents = vec![];

            while let Some((content, content_end_index)) = parse_content(document, curr_index) {
                contents.push(Box::new(content));
                curr_index = content_end_index + 1;
            }

            match get_end_tag_end_index(document, curr_index) {
                Some(end_tag_end_index) if get_end_tag_name(document, curr_index) == name.to_vec() => {
                    return Some((RawElement::new(name, attributes, false, contents), end_tag_end_index));
                },
                _ => {
                    raise_error(format!(
                        "{} tag doesn't have an end tag!\n...  {}  ...",
                        name.to_string(),
                        GString::new(index.max(12) - 12, (index + 18).min(document.len())).to_string()
                    ));
                    return None;
                }
            }

        },
        None => {}
    }

    None
}

// document[index..index + 2] == '</'
fn get_end_tag_name(document: &[u16], index: usize) -> Vec<u16> {
    let name_end_index = get_name_end_index(document, index + 2).unwrap();
    document[index + 2..name_end_index + 1].to_vec()
}

#[cfg(test)]
mod tests {
    use crate::parse::*;
    use crate::testbench::*;

    #[test]
    fn empty_element_tag_test() {
        get_xxx_end_index(
            vec![
                ("not an empty_element_tag", None),
                ("<br/>", Some(4)),
                ("<br />", Some(5)),
                ("<img src=\"sample.png\" />", Some(23)),
                ("<img src=\"sample.png\">", None),
            ],
            get_empty_element_tag_end_index
        );
    }

    #[test]
    fn end_tag_test() {
        get_xxx_end_index(
            vec![
                ("not an end_tag", None),
                ("</태그>", Some(4)),
                ("</div>", Some(5)),
                ("<태그>", None),
                ("<div>", None),
            ],
            get_end_tag_end_index
        );
    }

    #[test]
    fn start_tag_test() {
        get_xxx_end_index(
            vec![
                ("not a start_tag", None),
                ("<태그>", Some(3)),
                ("<div>", Some(4)),
                ("<&>", None),
                ("<&amp;>", None),
            ],
            get_start_tag_end_index
        );
    }

}