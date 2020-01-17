use std::fmt;

pub trait TNode<T> {
    fn new(data: T) -> Self;
    // return if binary tree is full
    fn is_full(self) -> bool;

    fn get_left(&self) -> &Option<Box<Node<T>>>;

    fn get_right(&self) -> &Option<Box<Node<T>>>;

    fn get_value(&self) -> &T;
}

impl <T> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", 1, 2)
    }
}
#[derive(PartialEq, Debug, Clone)]
pub struct Node <T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl <T>TNode<T> for Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data: data,
            left: None,
            right: None,
        }
    }

    fn is_full(self) -> bool {
        fn checking<T>(x: &Option<Box<Node<T>>>) -> bool {
            match x {
                Some(data) => {
                     match (data.left.as_ref(), data.right.as_ref()) {
                        (Some(a), Some(b)) => {
                            return checking(&a.left) && checking(&b.right)
                        }
                        (None, None) => {
                            return true
                        }
                        (_, _) => {
                            return false
                        }
                    }
                }
                _ => {
                    return false
                }
            }
        };
        let data = Some(Box::new(self));
        return checking(&data)
    }

    fn get_left(&self) -> &Option<Box<Node<T>>> {
       &self.left
    }

    fn get_right(&self) -> &Option<Box<Node<T>>> {
       &self.right
    }

    fn get_value(&self) -> &T{
        &self.data
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct BinaryTree<B>{
    root: Option<B>
}

impl <B> BinaryTree<B> where
    B: TNode<i32>
    {
    pub fn new(root:B) -> Self {
        Self {
            root: Some(root),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.root {
            Some(_) => false,
            _ => true
        }
    }

    pub fn insert(&mut self, data: B){ 
        match self.is_empty() {
            true => self.root = Some(data),
            false => {
                let mut parent_node = self.root.as_ref();
                loop {
                    if data.get_value() < parent_node.unwrap().get_value() {
                        parent_node.get_left()
                    }
                }
            }
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