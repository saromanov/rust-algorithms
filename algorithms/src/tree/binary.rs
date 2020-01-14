pub trait Node<T> {
    pub fn new(data: T) -> Self;
    // return if binary tree is full
    pub fn is_full(&self) -> bool;

    pub fn get_left(&self) -> Option<Node<T>>;

    pub fn get_right(&self) -> Option<Node<T>>;
};

pub struct Node <T> {
    data: T
    left: Option<Node<T>>
    right: Option<Node<T>>
};

impl Node<T: Ord> for Node {
    pub fn new(data: T) -> Node {
        Node {
            data: data,
            left: None,
            right: None,
        }
    }

    pub fn is_full(&self) -> bool {
        if self.left == None && self.right == None {
            true
        }
        if self.left != None && self.right != None {
            is_full(self.left) && is_full(self.get_right)
        }

        false
    }

    pub fn get_left(&self) -> Option<Node<T>> {
        self.left
    }

    pub fn get_right(&self) -> Option<Node<T>> {
        self.right
    }
}