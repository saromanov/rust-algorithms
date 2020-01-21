
pub struct FenwickTree {
    data: Vec<usize>,
}

impl FenwickTree {
    pub fn new(size:usize) -> FenwickTree {
        Self {
            data: Vec::with_capacity(size),
        }
    }

    pub fn query(&self, idx:usize) -> usize {
        fn res(ft: &FenwickTree, result:usize, i:usize) -> usize {
            match i {
                0 => result,
                _ => {
                    let mut new_i = i;
                    new_i -= i & i.wrapping_sub(1);
                    res(ft, result+ft.data[i],  new_i)
                }
            }
        }

        return res(self, 0, idx)
    }
}