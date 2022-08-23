use super::{get_name_end_index, get_eq_end_index, get_reference_end_index};
use crate::gstring::GString;

// https://www.w3.org/TR/xml/#NT-Attribute
// name eq attribute_value
pub fn get_attribute_end_index(document: &[u16], index: usize) -> Option<usize> {

    if index >= document.len() {
        return None;
    }

    match get_name_end_index(document, index) {
        Some(name_end_index) => match get_eq_end_index(document, name_end_index + 1) {
            Some(eq_end_index) => get_attribute_value_end_index(document, eq_end_index + 1),
            None => None
        },
        None => None
    }

}

// https://www.w3.org/TR/xml/#NT-AttValue
// '"' ([^<&"] | reference)* '"' |  "'" ([^<&'] | reference)* "'"
pub fn get_attribute_value_end_index(document: &[u16], mut index: usize) -> Option<usize> {

    if index >= document.len() {
        None
    }

    else if document[index] == '"' as u16 || document[index] == '\'' as u16 {
        let quote = document[index];
        index += 1;

        loop {

            if document[index] == quote {
                return Some(index);
            }

            else if document[index] == '<' as u16 {
                return None;
            }

            else if document[index] == '&' as u16 {

                match get_reference_end_index(document, index) {
                    None => { return None; }
                    Some(reference_end_index) => {
                        index = reference_end_index;
                        continue;
                    }
                }

            }

            index += 1;
        }

    }

    else {
        None
    }

}

pub fn parse_attribute(document: &[u16], index: usize) -> ((GString, GString), usize) {  // ((att_name, att_value), end_index)
    let name_end_index = get_name_end_index(document, index).unwrap();
    let name = GString::new(index, name_end_index + 1);

    let eq_end_index = get_eq_end_index(document, name_end_index + 1).unwrap();
    let att_value_end_index = get_attribute_value_end_index(document, eq_end_index + 1).unwrap();

    let att_value = GString::new(eq_end_index + 2, att_value_end_index);  // exclude quotes

    ((name, att_value), att_value_end_index)
}