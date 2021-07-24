pub struct Counter {
    count: usize,
    min: usize,
    max: usize,
}

impl Counter {
    pub fn new(min: usize, max: usize) -> Self {
        assert!(min <= max);

        Counter { count: 0, min, max }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn increment(&mut self) {
        match self.count.checked_add(1) {
            Some(new_count) if new_count > self.max => self.count = self.min,
            Some(new_count) => self.count = new_count,
            None => self.count = self.min,
        }
    }

    pub fn decrement(&mut self) {
        match self.count.checked_sub(1) {
            Some(new_count) => self.count = new_count,
            None => self.count = self.max,
        };
    }
}
