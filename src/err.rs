use crate::gstring::GString;
use crate::utils::{from_v16, into_v16};

#[derive(Clone)]
pub struct HxmlError {
    message: String
}

impl HxmlError {

    pub(crate) fn new(message: String, index: usize) -> Self {
        let message = {

            if index == usize::MAX {
                format!("Error: {}", message)
            }

            else {
                let lines = GString::all().split('\n' as u16);
                let mut error_line_index = 0;

                for (i, line) in lines.iter().enumerate() {

                    if line.end >= index {
                        error_line_index = i;
                        break;
                    }

                }

                let lines: Vec<Vec<u16>> = lines.iter().enumerate().map(
                    |(index, line)| {
                        let line = line.to_vec();

                        let line = if line.len() > 86 {
                            vec![
                                line[0..86].to_vec(),
                                vec!['.' as u16; 3]
                            ].concat()
                        } else {
                            line
                        };

                        let marker = if index == error_line_index {
                            into_v16(">>>")
                        } else {
                            vec![' ' as u16; 3]
                        };

                        let line_no = into_v16(&format!("{index:05} | "));

                        vec![marker, line_no, line].concat()
                    }
                ).collect();

                let line_output_start = error_line_index.max(3) - 3;
                let line_output_end = (line_output_start + 8).min(lines.len());

                let lines = lines[line_output_start..line_output_end].into_iter().map(|line| from_v16(&line)).collect::<Vec<String>>().join("\n");

                format!("Error: {}\n\n{lines}\n", message)
            }

        };
        HxmlError { message }
    }

}

impl std::fmt::Debug for HxmlError {

    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.message)
    }

}

impl std::fmt::Display for HxmlError {

    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.message)
    }

}

pub static mut ERRORS: Vec<HxmlError> = vec![];

pub fn reset_errors() {
    unsafe { ERRORS = vec![]; }
}

pub fn read_errors() -> Vec<HxmlError> {
    unsafe { ERRORS.clone() }
}

pub fn raise_error(e: HxmlError) {
    unsafe { ERRORS.push(e); }
}

