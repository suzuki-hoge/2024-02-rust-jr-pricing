pub struct NumberOfPassengers {
    pub adult: usize,
    pub child: usize,
}

impl NumberOfPassengers {
    pub fn total(&self) -> usize {
        self.adult + self.child
    }
}
