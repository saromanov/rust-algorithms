extern crate num;

use std::fmt;
use std::cmp::max;
use std::ops::Neg;
use self::num::{NumCast, Integer};

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
                    left.min()
                }
            },
        }
    }

    fn search(&self, value:T) -> bool {
        match *self {
            BinaryTree::Empty => false,
            BinaryTree::Node(ref data, ref left, ref right) => {
                if value == *data {
                    true
                }
                else if value > *data {
                    right.search(value)
                }
                else {
                    left.search(value)
                }
            }
        }
    }
    
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_depth() {
        let bt = super::BinaryTree::Empty.insert(10).insert(5).insert(2);
        assert_eq!(bt.depth(), 3);
        let bt2 = super::BinaryTree::Empty.insert(10);
    }

    #[test]
    fn test_max() {
        let bt = super::BinaryTree::Empty.insert(10).insert(5).insert(2).insert(20);
        assert_eq!(bt.max(), 20);
        let bt2 = super::BinaryTree::Empty.insert(10);
        assert_eq!(bt2.max(), 10);
        let bt3 = super::BinaryTree::Empty.insert(-5).insert(30).insert(2).insert(20);
        assert_eq!(bt3.max(), 30);
    }

    #[test]
    fn test_min() {
        let bt = super::BinaryTree::Empty.insert(10).insert(5).insert(2).insert(20);
        assert_eq!(bt.min(), 2);
        let bt2 = super::BinaryTree::Empty.insert(10);
        assert_eq!(bt2.min(), 10);
        let bt3 = super::BinaryTree::Empty.insert(-5).insert(30).insert(2).insert(20);
        assert_eq!(bt3.min(), -5);
    }

    #[test]
    fn test_search() {
        let bt = super::BinaryTree::Empty.insert(10).insert(5).insert(2).insert(20);
        assert_eq!(bt.search(2), true);
        assert_eq!(bt.search(7), false);
        assert_eq!(bt.search(-2), false);
        assert_eq!(bt.search(20), true);
    }
}