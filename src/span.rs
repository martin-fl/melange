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

    pub fn combine(s1: Span, s2: Span) -> Span {
        let start = std::cmp::min(s1.start, s2.start);
        let len = std::cmp::max(s1.start + s1.len - start, s2.start + s2.len - start);
        Span::new(start, len)
    }
}
