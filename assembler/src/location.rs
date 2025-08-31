#[derive(Copy, Clone, Debug)]
pub struct Location {
    pub index: usize,
    pub line: usize,
    pub col: usize,
}

impl Location {
    pub fn start() -> Location {
        Location {
            index: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn advance_col(self) -> Location {
        Location {
            index: self.index + 1,
            line: self.line,
            col: self.col + 1,
        }
    }

    pub fn advance_line(self) -> Location {
        Location {
            index: self.index + 1,
            line: self.line + 1,
            col: 1,
        }
    }
}
