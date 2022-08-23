pub mod raw_element;
pub mod element;
pub mod memory;
pub mod attribute;

pub static mut ERRORS: Vec<String> = vec![];

pub fn reset_errors() {
    unsafe { ERRORS = vec![]; }
}

pub fn read_errors() -> Vec<String> {
    unsafe { ERRORS.clone() }
}

pub fn raise_error(e: String) {
    unsafe { ERRORS.push(e); }
}