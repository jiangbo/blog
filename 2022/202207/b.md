use game::tree::{binary_search_tree::BinarySearchTree, Tree};

fn main() {
    let mut tree = BinarySearchTree::default();
    (0..10).for_each(|i| tree.insert(i));
    tree.pre_order();
    tree.in_order();
    tree.post_order();
    println!("{:?}", tree.contains(&4));
    println!("{:?}", tree.remove(&4));
    println!("{:?}", tree.contains(&4));
}
----------

pub mod tree;

---------------

pub trait Tree<T> {
    fn pre_order(&self) -> Vec<&T>;

    fn in_order(&self);

    fn post_order(&self);

    fn insert(&mut self, value: T);

    fn remove(&mut self, value: &T) -> Option<T>;

    fn contains(&self, value: &T) -> bool;
}

type NodeRef<T> = Option<Box<Node<T>>>;
pub(crate) struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

impl<T> Node<T> {
    fn new_node_ref(value: T) -> NodeRef<T> {
        Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }))
    }
}

pub mod binary_search_tree;
pub mod binary_tree;

---------------

use std::fmt::Debug;

use super::{NodeRef, Tree};

#[derive(Default)]
pub struct BinaryTree<T> {
    root: NodeRef<T>,
}

impl<T: Debug> Tree<T> for BinaryTree<T> {
    fn pre_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut stack = vec![&self.root];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                stack.push(&node.right);
                stack.push(&node.left);
                result.push(&node.value);
            }
        }
        result
    }

    fn in_order(&self) {
        let mut stack = Vec::new();
        let mut current = &self.root;
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(current);
                current = &node.left;
            }
            current = stack.pop().unwrap();
            println!("{:?}", current.as_ref().unwrap().value);
            current = &current.as_ref().unwrap().right;
        }
    }

    fn post_order(&self) {
        let mut stack = vec![&self.root];
        let mut result = vec![];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                result.push(&node.value);
                stack.push(&node.left);
                stack.push(&node.right);
            }
        }
        result.iter().rev().for_each(|node| println!("{node:?}",));
    }

    fn insert(&mut self, _: T) {
        unimplemented!()
    }

    fn remove(&mut self, _: &T) -> Option<T> {
        unimplemented!()
    }

    fn contains(&self, _: &T) -> bool {
        unimplemented!()
    }
}

impl<T> BinaryTree<T> {
    pub(crate) fn root_mut(&mut self) -> &mut NodeRef<T> {
        &mut self.root
    }

    pub(crate) fn root(&self) -> &NodeRef<T> {
        &self.root
    }
}

------

use std::{cmp::Ordering, fmt::Debug};

use super::{binary_tree::BinaryTree, Node, NodeRef, Tree};

#[derive(Default)]
pub struct BinarySearchTree<T> {
    tree: BinaryTree<T>,
}

impl<T: Debug + Ord> BinarySearchTree<T> {
    fn get_min(root: &mut NodeRef<T>) -> Option<T> {
        let mut current = root;
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

    pub(crate) fn root_mut(&mut self) -> &mut NodeRef<T> {
        self.tree.root_mut()
    }
    pub(crate) fn root(&self) -> &NodeRef<T> {
        self.tree.root()
    }
}

impl<T: Debug + Ord> Tree<T> for BinarySearchTree<T> {
    fn pre_order(&self) -> Vec<&T> {
        self.tree.pre_order()
    }

    fn in_order(&self) {
        self.tree.in_order();
    }

    fn post_order(&self) {
        self.tree.post_order();
    }

    fn insert(&mut self, value: T) {
        let mut current = self.root_mut();
        while let Some(node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Less => &mut node.left,
                Ordering::Greater => &mut node.right,
                // 相等元素不插入
                Ordering::Equal => return,
            };
        }
        *current = Node::new_node_ref(value)
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        let mut current = self.root_mut();
        while let Some(node) = current {
            current = match node.value.cmp(value) {
                Ordering::Less => &mut current.as_mut()?.right,
                Ordering::Greater => &mut current.as_mut()?.left,
                Ordering::Equal => break,
            }
        }

        let mut node = current.take()?;
        *current = match (node.left.as_ref(), node.right.as_ref()) {
            (None, None) => None,
            (Some(_), None) => node.left.take(),
            (None, Some(_)) => node.right.take(),
            (Some(_), Some(_)) => Some(Box::new(Node {
                value: Self::get_min(&mut node.right)?,
                left: node.left.take(),
                right: node.right.take(),
            })),
        };
        Some(node.value)
    }

    fn contains(&self, value: &T) -> bool {
        let mut current = self.root();
        while let Some(node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Less => &node.left,
                Ordering::Greater => &node.right,
                Ordering::Equal => return true,
            };
        }
        false
    }
}
