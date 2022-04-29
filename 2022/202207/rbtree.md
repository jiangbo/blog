use std::rc::{Rc, Weak};

use super::Tree;

pub enum Color {
    Red,
    Black,
}

struct Node<T> {
    value: T,
    color: Color,
    parent: Option<Rc<Node<T>>>,
    left: Option<Rc<Node<T>>>,
    right: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            color: Color::Red,
            parent: None,
            left: None,
            right: None,
        }
    }

    fn new_child(value: T) -> Child<T> {
        Some(Rc::new(Node {
            value,
            color: Color::Red,
            parent: None,
            left: None,
            right: None,
        }))
    }
}

type Child<T> = Option<Rc<Node<T>>>;
type Parent<T> = Option<Weak<Node<T>>>;

#[derive(Default)]
pub struct RedBlackTree<T: Ord> {
    root: Child<T>,
}

impl<T: Ord> Tree<T> for RedBlackTree<T> {
    fn pre_order(&self) -> Vec<&T> {
        todo!()
    }

    fn in_order(&self) -> Vec<&T> {
        todo!()
    }

    fn post_order(&self) -> Vec<&T> {
        todo!()
    }

    fn insert(&mut self, value: T) {
        match &mut self.root {
            None => {
                self.root = Node::new_child(value);
                Self::set_color(&mut self.root, Color::Black);
            }
            Some(_) => {}
        }
    }

    fn remove(&mut self, _: &T) -> Option<T> {
        todo!()
    }

    fn contains(&mut self, value: &T) -> bool {
        todo!()
    }
}

impl<T: Ord> RedBlackTree<T> {
    fn set_color(node: &mut Child<T>, color: Color) -> &mut Child<T> {
        if let Some(node) = node {
            if let Some(node) = Rc::get_mut(node) {
                node.color = color;
            }
        }
        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rbtree_插入() {
        let mut tree = RedBlackTree::default();
        (0..5).for_each(|e| tree.insert(e));
        assert!(tree.in_order().windows(2).all(|w| w[0] <= w[1]));
        println!("{:?}", tree.pre_order());
    }
}
