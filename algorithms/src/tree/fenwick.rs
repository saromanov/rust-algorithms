
pub struct FenwickTree {
    data: Vec<i32>,
}

impl FenwickTree {
    pub fn new(size:usize) -> FenwickTree {
        Self {
            data: Vec::with_capacity(size),
        }
    }

    pub fn query(&self, idx:i32) -> i32 {
        fn res(ft: &FenwickTree, result:i32, i:i32) -> i32 {
            match i {
                0 => result,
                _ => {
                    let mut new_i = i;
                    new_i -= i & (-i);
                    res(ft, result+ft.data[i],  new_i)
                }
            }
        }

        return res(self, 0, idx)
    }
}