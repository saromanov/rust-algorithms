use std::fmt;
use std::cmp::max;

type Leaf<T> = Box<BinaryTree<T>>;

enum BinaryTree<T>{
    Empty,
    Node(T, Leaf<T>, Leaf<T>)
}

fn empty_node<T>() -> Leaf<T> {
    Box::new(BinaryTree::Empty)
}

impl <T> BinaryTree<T> where
    T: PartialOrd
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
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_create() {
        let n1 = Node(10);
        assert_eq!(add(1, 2), 3);
    }
}