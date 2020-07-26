use std::fmt;
use std::cmp;

#[allow(unused)]
struct BinaryTree<T>
where T: fmt::Debug + cmp::PartialEq + cmp::PartialOrd {
    key: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

#[allow(unused)]
impl<T> BinaryTree<T> 
where T: fmt::Debug + cmp::PartialEq + cmp::PartialOrd {
    fn new_node(key: T, left: Option<Box<BinaryTree<T>>>, right: Option<Box<BinaryTree<T>>>) -> BinaryTree<T> {
        BinaryTree::<T> {
            key: key,
            left: left,
            right: right
        }
    }

    fn insert(&mut self, key: Option<T>) {
        if key.is_none() {
            return ;
        }
        let key = key.unwrap();
        if self.key < key {
            if let Some(ref mut right) = self.right {
                right.insert(Some(key));
            } else {
                self.right = Some(Box::new(Self::new_node(key, None, None)));
            }
        } else {
            if let Some(ref mut left) = self.left {
                left.insert(Some(key));
            } else {
                self.left = Some(Box::new(Self::new_node(key, None, None)));
            }
        } 
    }

    fn display(&self) {
        print!("{:?}\t", self.key);
        if let Some(ref left) = self.left {
            left.display();
        }
        if let Some(ref right) = self.right {
            right.display();
        }
    }

    // TODO 
    fn delete(&mut self, key: T) {
        
    }
}


