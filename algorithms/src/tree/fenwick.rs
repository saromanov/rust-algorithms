
pub struct FenwickTree {
    data: Vec<i32>,
}

impl FenwickTree {
    pub fn new(size:i32) -> FenwickTree {
        Self {
            data: vec![0:size],
        }
    }

    pub fn query(&self, idx:i32) -> i32 {
        fn res(ft: &FenwickTree, result:i32, i:i32) -> i32 {
            match i {
                0 => result,
                _ => {
                    res(result+ft.data[i], i -= i & (-i))
                }
            }
        }

        return res(self, 0, idx)
    }
}