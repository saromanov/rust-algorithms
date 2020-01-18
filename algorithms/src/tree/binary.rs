extern crate num;

use std::fmt;
use std::cmp::max;
use std::ops::Neg;
use self::num::{NumCast, Integer};

pub tree: BinaryTree;

type Leaf<T> = Box<BinaryTree<T>>;

#[derive(PartialEq)]
pub enum BinaryTree<T>{
    Empty,
    Node(T, Leaf<T>, Leaf<T>)
}

fn empty_node<T>() -> Leaf<T> {
    Box::new(BinaryTree::Empty)
}

impl <T> BinaryTree<T> where
    T: PartialOrd + Integer + Neg<Output = T> + NumCast
    {

    pub fn insert(self, value:T) -> Self { 
       match self {
           BinaryTree::Empty => {
               BinaryTree::Node(value, empty_node(), empty_node())
           },
           BinaryTree::Node(data, left, right) => {
               if value < data {
                   BinaryTree::Node(data, Box::new(left.insert(value)), right)
               } else {
                   BinaryTree::Node(data, left, Box::new(right.insert(value)))
               }
           },
       }
    }

    fn depth(&self) -> usize {
        match *self {
            BinaryTree::Empty => 0,
            BinaryTree::Node(_, ref left, ref right) =>
                1 + max(left.depth(), right.depth())
        }
    }

    fn max(self) -> T{
         match self {
            BinaryTree::Empty => T::zero(),
            BinaryTree::Node(data, _, right) => {
                if *right == BinaryTree::Empty {
                   data
                } else {
                    right.max()
                }
            },
        }
    }

    fn min(self) -> T{
         match self {
            BinaryTree::Empty => T::zero(),
            BinaryTree::Node(data, left, _) => {
                if *left == BinaryTree::Empty {
                   data
                } else {
                    left.max()
                }
            },
        }
    }
    
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_depth() {
        let mut bt = BinaryTree::Empty.insert(10).insert(5).insert(2);
        assert_eq!(bt.depth(), 2);
    }
}