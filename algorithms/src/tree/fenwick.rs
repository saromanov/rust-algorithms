
pub struct FenwickTree {
    data: [i32],
}

impl FenwickTree {
    pub fn new(size:i32) -> FenwickTree {
        Self {
            data: [i32;size],
        }
    }

    pub fn query(&self, idx:i32) -> i32 {
        fn res(result:i32, i:i32) -> i32 {
            match i {
                0 => result,
                _ => {
                    res(result+self.data[i], i -= i & (-i))
                }
            }
        }

        return res(0, idx)
    }
}