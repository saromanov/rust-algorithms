
pub struct FenwickTree {
    data: Vec<usize>,
    size: usize,
}


impl FenwickTree {
    pub fn new(size:usize) -> FenwickTree {
        Self {
            size: size,
            data: Vec::with_capacity(size),
        }
    }

    pub fn update(self, idx:usize, value:usize) -> bool {
        if idx > self.size {
            false;
        }
        fn res(mut ft: FenwickTree, value:usize, i:usize) -> bool {
            match i {
                0 => true,
                _ => {
                    let mut new_i = i;
                    ft.data[i] += value;
                    new_i += i & i.wrapping_sub(1);
                    res(ft, value, new_i)
                }
            }
        }

        return res(self, value, idx) 
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

#[cfg(test)]
mod tests {

    #[test]
    fn test_basic() {
        let bt = super::FenwickTree::new(10);
        assert_eq!(bt.update(1,20), true);
    }
}