pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    pub fn turn_right(&mut self) {
        use Direction::*;
        *self = match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn turn_left(&mut self) {
        use Direction::*;
        *self = match *self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    pub fn turn_back(&mut self) {
        use Direction::*;
        *self = match self {
            Up=>Down,
            Right=>Left,
            Down=>Up,
            Left=>Right
        }
    }
}
