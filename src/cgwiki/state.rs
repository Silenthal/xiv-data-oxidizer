use ironworks::{excel::Excel, sestring::format::Input};

use crate::cgwiki::write::Write;

pub struct CGWState<'a> {
    pub input: &'a Input,
    pub writer: &'a mut dyn Write,
    pub excel: &'a Excel,
    pub time: u32,
}

impl<'a> CGWState<'a> {
    pub fn new(input: &'a Input, writer: &'a mut dyn Write, excel: &'a Excel) -> Self {
        let mut ret = Self {
            input,
            writer,
            excel,
            time: 0,
        };
        ret.init_time();
        ret
    }

    pub fn init_time(&mut self) {
        const FFXIV_EPOCH: u32 = 1377590400;
        self.time = FFXIV_EPOCH;
    }
}
