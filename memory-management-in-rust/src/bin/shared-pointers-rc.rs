use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Tree<T> {
    root: Node<T>, // this moves node
}

// Each child has a pointer to its parent.
// if the child has an Rc pointer to its parent, it will never drop.
// This is a circular dependency, and to avoid it,
// the pointer to the parent will be a Weak pointer
#[derive(Debug, Clone)]
struct Node<T> {
    parent: Option<Weak<Node<T>>>, // no need for refcell in weak, no mut of root?
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
    value: T,
}

// impl<T> Node<T> {
impl<T: Copy> Node<T> {
    fn new(value: T) -> Self {
        Node {
            parent: None,
            left: None,
            right: None,
            value,
        }
    }

    // Thanks to nodes derive clone
    fn add_left(&mut self, value: T) {
        let root: Node<T> = self.clone();
        let rc: Rc<Node<T>> = Rc::new(root);
        let weak: Weak<Node<T>> = Rc::downgrade(&rc);
        let option: Option<Weak<Node<T>>> = Some(weak);

        let left = Node {
            parent: option,
            left: None,
            right: None,
            value,
        };

        self.left = Some(Rc::new(RefCell::new(left)));
    }

    fn add_right(&mut self, mut node: Node<T>) {
        node.add_parent(self);
        self.right = Some(Rc::new(RefCell::new(node)));
    }

    // Thanks to impl has constraint that T must be copy
    fn add_parent(&mut self, parent: &Node<T>) {
        let v = parent.value.clone();
        let node: Node<T> = Node::new(v);
        let rc: Rc<Node<T>> = Rc::new(node);
        let weak: Weak<Node<T>> = Rc::downgrade(&rc);
        let option: Option<Weak<Node<T>>> = Some(weak);

        self.parent = option;
    }
}

fn main() {
    let mut two = Node::new(2);
    // let tree = Tree { root: two };
    let three = Node::new(3);

    two.add_left(1);
    two.add_right(three);

    println!("root: {:?}", two);
    println!("root: {:?}", two.left);
    println!("root: {:?}", two.left.unwrap().borrow().parent);
    println!("root: {:?}", two.right);
    println!("root: {:?}", two.right.unwrap().borrow().parent);
}
