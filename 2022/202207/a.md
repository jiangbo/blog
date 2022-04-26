use std::{cmp::Ordering, ptr::null_mut};

use super::Tree;

enum Color {
    Red,
    Black,
}

struct RBNode<T: Ord> {
    value: T,
    color: Color,
    parent: *mut RBNode<T>,
    left: *mut RBNode<T>,
    right: *mut RBNode<T>,
}

impl<T: Ord> RBNode<T> {
    fn new(value: T) -> RBNode<T> {
        RBNode {
            value,
            color: Color::Red,
            parent: null_mut(),
            left: null_mut(),
            right: null_mut(),
        }
    }
}

pub struct RedBlackTree<T: Ord> {
    root: *mut RBNode<T>,
}

impl<T: Ord> Default for RedBlackTree<T> {
    fn default() -> Self {
        RedBlackTree { root: null_mut() }
    }
}

impl<T: Ord> Tree<T> for RedBlackTree<T> {
    fn pre_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut stack = vec![self.root];
        while let Some(node) = stack.pop() {
            if !node.is_null() {
                let node = unsafe { &*node };
                stack.push(node.right);
                stack.push(node.left);
                result.push(&node.value);
            }
        }
        result
    }

    fn in_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = self.root;
        unsafe {
            while !current.is_null() || !stack.is_empty() {
                while current.is_null() {
                    stack.push(current);
                    current = (*current).left;
                }
                current = stack.pop().unwrap();
                result.push(&(*current).value);
                current = (*current).right;
            }
        }
        result
    }

    fn post_order(&self) -> Vec<&T> {
        let mut stack = vec![self.root];
        let mut result = vec![];
        while let Some(node) = stack.pop() {
            if !node.is_null() {
                let node = unsafe { &*node };
                result.push(&node.value);
                stack.push(node.left);
                stack.push(node.right);
            }
        }
        result.reverse();
        result
    }

    fn insert(&mut self, value: T) {
        todo!()
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        todo!()
    }

    fn contains(&mut self, value: &T) -> bool {
        let mut current = self.root;
        unsafe {
            while !current.is_null() {
                current = match (*current).value.cmp(value) {
                    Ordering::Less => (*current).left,
                    Ordering::Greater => (*current).right,
                    Ordering::Equal => return true,
                };
            }
        }
        false
    }
}
