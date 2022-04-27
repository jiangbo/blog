use std::ptr::null_mut;

pub mod avl_tree;
pub mod binary_search_tree;
pub mod binary_tree;
pub mod rb_tree;
pub mod splay_tree;

pub trait Tree<T> {
    fn pre_order(&self) -> Vec<&T>;

    fn in_order(&self) -> Vec<&T>;

    fn post_order(&self) -> Vec<&T>;

    fn insert(&mut self, value: T);

    fn remove(&mut self, value: &T) -> Option<T>;

    fn contains(&mut self, value: &T) -> bool;
}

pub(crate) enum Color {
    Red,
    Black,
}

type NodeRef<T> = Option<Box<Node<T>>>;
pub(crate) struct Node<T> {
    value: T,
    parent: *mut NodeRef<T>,
    color: Color,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

impl<T> Node<T> {
    fn new_node_ref(value: T) -> NodeRef<T> {
        Self::children(value, None, None)
    }

    fn children(value: T, left: NodeRef<T>, right: NodeRef<T>) -> NodeRef<T> {
        Some(Box::new(Node {
            value,
            parent: null_mut(),
            color: Color::Red,
            left,
            right,
        }))
    }

    fn get_min(tree: &mut NodeRef<T>) -> Option<T> {
        let mut current = tree;
        while let Some(node) = current {
            current = match node.left {
                Some(_) => &mut current.as_mut()?.left,
                None => break,
            }
        }
        let node = current.take()?;
        *current = node.right;
        Some(node.value)
    }

    fn remove(tree: &mut NodeRef<T>) -> Option<T> {
        let mut node = tree.take()?;
        *tree = match (node.left.as_ref(), node.right.as_ref()) {
            (None, None) => None,
            (Some(_), None) => node.left.take(),
            (None, Some(_)) => node.right.take(),
            (Some(_), Some(_)) => Node::children(
                Self::get_min(&mut node.right)?,
                node.left.take(),
                node.right.take(),
            ),
        };
        Some(node.value)
    }

    fn left_rotate(root: &mut NodeRef<T>) {
        if let Some(mut node) = root.take() {
            if let Some(mut new_root) = node.right.take() {
                node.right = new_root.left.take();
                new_root.left = Some(node);
                *root = Some(new_root);
            }
        }
    }

    fn right_rotate(root: &mut NodeRef<T>) {
        if let Some(mut node) = root.take() {
            if let Some(mut new_root) = node.left.take() {
                node.left = new_root.right.take();
                new_root.right = Some(node);
                *root = Some(new_root);
            }
        }
    }
}
