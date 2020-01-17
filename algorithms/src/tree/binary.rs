use std::fmt;

pub trait TNode<T> {
    fn new(data: T) -> Self;
    // return if binary tree is full
    fn is_full(self) -> bool;

    fn get_left(&self) -> &Option<Box<Node<T>>>;

    fn get_right(&self) -> &Option<Box<Node<T>>>;
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
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_create() {
        let n = Node(10);
        assert_eq!(add(1, 2), 3);
    }
}