#[derive(Debug)]
pub struct BST {
    root: Link
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

impl Link {
    pub fn search(&self, val: i32) -> bool {
        match *self {
            Link::Empty => false,
            Link::More(ref node) => {
                if val == node.elem { 
                    true
                } else if val < node.elem {
                    node.left.search(val) 
                } else {
                    node.right.search(val)
                }
            }
        }
    }
    pub fn insert(&mut self, val: i32) -> bool {
        match *self {
            Link::Empty => {
                // Insert into this node.
                let node = Box::new(Node {
                    elem: val,
                    left: Link::Empty,
                    right: Link::Empty
                });
                *self = Link::More(node);
                true
            }
            Link::More(ref mut node) => {
                if val == node.elem { 
                    false
                } else if val < node.elem {
                    node.left.insert(val) 
                } else {
                    node.right.insert(val)
                }
            }
        }
    }
}

impl BST {
    pub fn new() -> Self {
        BST { root: Link::Empty }
    }
    pub fn search(&self, val: i32) -> bool {
        self.root.search(val)
    }
    pub fn insert(&mut self, val: i32) -> bool {
        self.root.insert(val)
    }
}


#[cfg(test)]
mod test {
    use super::BST;
     
    #[test]
    fn basics() {
        let mut tree = BST::new();

        // Check containment when empty.
        assert_eq!(tree.search(1), false);

        // Populate tree.
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);

        // Check containment when nonempty.
        assert_eq!(tree.search(5), false);
        assert_eq!(tree.search(3), true);

        // Test re-insertion fails.
        assert_eq!(tree.insert(2), false);
        assert_eq!(tree.insert(4), true);
    }

}
