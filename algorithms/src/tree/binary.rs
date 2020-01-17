use std::fmt;

pub trait TNode<T> {
    fn new(data: T) -> Self;
    // return if binary tree is full
    fn is_full(&self) -> bool;

    fn get_left(&self) -> Option<Node<T>>;

    fn get_right(&self) -> Option<Node<T>>;
}

impl <T> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", 1, 2)
    }
}

pub struct Node <T> {
    data: T,
    left: Option<Node<T>>,
    right: Option<Node<T>>,
}

impl <T>TNode<T> for Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data: data,
            left: None,
            right: None,
        }
    }

fn is_full(&self) -> bool {
        if self.left == None && self.right == None {
            true
        }
        if self.left != None && self.right != None {
            self.is_full(self.left) && self.is_full(self.get_right)
        }

        false
    }

    fn get_left(&self) -> Option<Node<T>> {
        self.left
    }

    fn get_right(&self) -> Option<Node<T>> {
        self.right
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