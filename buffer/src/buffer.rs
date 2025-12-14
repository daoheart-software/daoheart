use ropey::Rope;

pub struct Buffer {
    rope: Rope,
}

impl Buffer {
    pub fn new(rope: Rope) -> Self {
        Buffer { rope }
    }

    pub fn lines_at(&self, line_idx: usize) -> ropey::iter::Lines<'_> {
        self.rope.lines_at(line_idx)
    }

    pub fn lines_count(&self) -> usize {
        self.rope.len_lines()
    }
}
