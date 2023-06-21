use crate::utils::from_v16;

#[derive(Copy, Clone, Debug)]
pub struct GString {
    pub start: usize,
    pub end: usize
}

pub static mut GLOBAL_STRING: Vec<u16> = vec![];

pub fn set_global_string(string: Vec<u16>) {

    unsafe {
        GLOBAL_STRING = string;
    }

}

impl GString {

    pub fn new(start: usize, end: usize) -> Self {
        GString { start, end }
    }

    pub fn all() -> Self {
        unsafe { GString { start: 0, end: GLOBAL_STRING.len() } }
    }

    pub fn to_vec(&self) -> Vec<u16> {
        unsafe {
            #[cfg(test)]
            assert!(self.end >= self.start && self.end <= GLOBAL_STRING.len());
            GLOBAL_STRING[self.start..self.end].to_vec()
        }
    }

    pub fn to_string(&self) -> String {
        from_v16(&self.to_vec())
    }

    pub fn to_slice(&self) -> &[u16] {
        unsafe {
            &GLOBAL_STRING[self.start..self.end]
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn g_len(&self) -> usize {
        unsafe {
            GLOBAL_STRING.len()
        }
    }

    pub fn index(&self, index: usize) -> u16 {
        unsafe {
            GLOBAL_STRING[self.start + index]
        }
    }

    pub fn slice(&self, start: usize, end: usize) -> GString {
        GString::new(self.start + start, self.start + end)
    }

    pub fn iter(&self) -> std::slice::Iter<u16> {

        unsafe {
            GLOBAL_STRING[self.start .. self.end].iter()
        }

    }

    pub fn split(&self, delim: u16) -> Vec<GString> {
        let mut last_index = self.start;
        let mut result = vec![];

        for (ind, c) in self.iter().enumerate() {

            if *c == delim {
                result.push(GString::new(last_index, ind + self.start));
                last_index = self.start + ind + 1;
            }

        }

        result.push(GString::new(last_index, self.end));

        result
    }

}

#[cfg(test)]
mod tests {
    use crate::gstring::*;
    use crate::utils::*;

    #[test]
    fn split_test() {
        set_global_string(into_v16("[1, 2, 3, 4]"));
        let elements = GString::new(1, 11);

        assert_eq!(elements.to_string(), String::from("1, 2, 3, 4"));

        let splits = elements.split(',' as u16).iter().map(|element| element.to_string()).collect::<Vec<String>>();

        assert_eq!(
            splits,
            vec![
                String::from("1"),
                String::from(" 2"),
                String::from(" 3"),
                String::from(" 4"),
            ]
        );
    }
}