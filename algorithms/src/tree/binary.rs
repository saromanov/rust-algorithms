use std::fmt;


enum BinaryTree<T>{
    Empty,
    Node(T, Box<Tree<T>>, Box<Tree<T>>)
}

impl <B> BinaryTree<B> where
    B: PartialOrd
    {

    pub fn insert(self, value:T){ 
       
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