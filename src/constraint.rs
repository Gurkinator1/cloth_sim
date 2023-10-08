pub struct Constraint {
    pub p1: usize,
    pub p2: usize,
    pub length: f32
}

impl Constraint {
    pub fn new(p1: usize, p2: usize, len: f32) -> Constraint {
        Constraint { p1, p2, length:len }
    }
}