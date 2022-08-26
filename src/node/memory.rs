use super::element::Element;
use super::pointer::ElementPtr;

static mut FREE_LIST: Vec<usize> = vec![];
pub static mut ELEMENTS: Vec<Element> = vec![];

pub fn init() {
    unsafe {
        ELEMENTS = vec![];
        FREE_LIST = vec![];
    }
}

pub fn delete(pointer: ElementPtr) {
    unsafe {
        ELEMENTS[pointer.ptr].is_alive = false;
        FREE_LIST.push(pointer.ptr);
    }
}

pub fn allocate(mut element: Element) -> ElementPtr {

    unsafe {

        if FREE_LIST.len() == 0 {
            #[cfg(test)]
            assert!(element.pointer.ptr == super::pointer::NULL);

            element.pointer = ElementPtr::new(ELEMENTS.len());
            ELEMENTS.push(element);

            ElementPtr::new(ELEMENTS.len() - 1)
        }

        else {
            let pointer = FREE_LIST.pop().unwrap();
            element.pointer = ElementPtr::new(pointer);
            ELEMENTS[pointer] = element;

            ElementPtr::new(pointer)
        }

    }

}

pub fn get<'a>(pointer: usize) -> &'a Element {
    unsafe {
        &ELEMENTS[pointer]
    }
}

pub fn get_mut<'a>(pointer: usize) -> &'a mut Element {
    unsafe {
        &mut ELEMENTS[pointer]
    }
}