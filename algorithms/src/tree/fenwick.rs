
pub struct FenwickTree {
    data: [i32],
}

impl FenwickTree {
    pub fn new(size:i32) -> FenwickTree {
        Self {
            data: [i32;size],
        }
    }
}