#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Span {
    start: u32,
    len: u32,
}

impl Span {
    pub fn new(start: u32, len: u32) -> Self {
        Self { start, len }
    }

    pub fn start(self) -> u32 {
        self.start
    }

    pub fn len(self) -> u32 {
        self.len
    }
}
