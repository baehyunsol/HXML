use super::get_name_end_index;
use crate::predicate::is_whitespace;
use crate::utils::{into_v16, to_lower};

// https://www.w3.org/TR/xml/#NT-PI
// '<?' pi_target (whitespace (char* - (char* '?>' char*)))? '?>'
pub fn get_processing_instruction_end_index(document: &[u16], index: usize) -> Option<usize> {

    if index + 2 >= document.len()
        || document[index] != '<' as u16
        || document[index + 1] != '?' as u16
    {
        return None;
    }

    match get_pi_target_end_index(document, index + 2) {
        Some(pi_target_end_index) => if pi_target_end_index + 2 >= document.len() {
            None
        } else if is_whitespace(&document[pi_target_end_index + 1]) {
            let mut curr_index = pi_target_end_index + 1;

            loop {

                if curr_index + 1 >= document.len() {
                    return None;
                }

                if document[curr_index] == '?' as u16 && document[curr_index + 1] == '>' as u16 {
                    return Some(curr_index + 1);
                }

                curr_index += 1;
            }

        } else if pi_target_end_index + 2 < document.len() &&
        document[pi_target_end_index + 1] == '?' as u16 &&
        document[pi_target_end_index + 2] == '>' as u16 {
            Some(pi_target_end_index + 2)
        } else {
            None
        },
        None => None
    }

}

// https://www.w3.org/TR/xml/#NT-PITarget
// name - (('X' | 'x') ('M' | 'm') ('L' | 'l'))
pub fn get_pi_target_end_index(document: &[u16], index: usize) -> Option<usize> {

    match get_name_end_index(document, index) {
        Some(name_end_index) => {
            let name_lower = to_lower(&document[index..name_end_index + 1]);

            if name_lower != into_v16("xml") {
                Some(name_end_index)
            }

            else {
                None
            }

        },
        None => None
    }

}