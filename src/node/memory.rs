use super::element::Element;

static mut FREE_LIST: Vec<ElementPtr> = vec![];
pub static mut ELEMENTS: Vec<Element> = vec![];

pub type ElementPtr = usize;

pub fn init() {
    unsafe {
        ELEMENTS = vec![];
        FREE_LIST = vec![];
    }
}

pub fn delete(pointer: ElementPtr) {
    unsafe {
        ELEMENTS[pointer].is_alive = false;
        FREE_LIST.push(pointer);
    }
}

pub fn allocate(mut element: Element) -> ElementPtr {

    unsafe {

        if FREE_LIST.len() == 0 {
            element.pointer = ELEMENTS.len();
            ELEMENTS.push(element);

            ELEMENTS.len() - 1
        }

        else {
            let pointer = FREE_LIST.pop().unwrap();
            element.pointer = pointer;
            ELEMENTS[pointer] = element;

            pointer
        }

    }

}

pub fn get(pointer: ElementPtr) -> &'static Element {
    unsafe {
        &ELEMENTS[pointer]
    }
}

pub fn get_mut(pointer: ElementPtr) -> &'static mut Element {
    unsafe {
        &mut ELEMENTS[pointer]
    }
}