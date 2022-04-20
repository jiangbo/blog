use std::cmp::Ordering;

use super::{binary_search_tree::BinarySearchTree, Node, NodeRef, Tree};

#[derive(Default)]
pub struct AvlTree<T> {
    tree: BinarySearchTree<T>,
}

impl<T: Ord> Tree<T> for AvlTree<T> {
    fn pre_order(&self) -> Vec<&T> {
        self.tree.pre_order()
    }

    fn in_order(&self) -> Vec<&T> {
        self.tree.in_order()
    }

    fn post_order(&self) -> Vec<&T> {
        self.tree.post_order()
    }

    fn insert(&mut self, value: T) {
        match self.tree.root_mut() {
            Some(node) => Self::insert_node(node, value),
            None => *self.tree.root_mut() = Node::new_node_ref(value),
        }
        Self::rebalance(self.tree.root_mut());
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        let root = self.tree.root_mut();
        let result = Self::remove_node(root, value);
        if root.is_some() {
            Self::rebalance(root);
        }
        result
    }

    fn contains(&mut self, value: &T) -> bool {
        self.tree.contains(value)
    }
}

impl<T: Ord> AvlTree<T> {
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

    fn height(root: &NodeRef<T>) -> usize {
        match root {
            Some(node) => {
                let left = Self::height(&node.left);
                let right = Self::height(&node.right);
                1 + std::cmp::max(left, right)
            }
            None => 0,
        }
    }

    fn rebalance(tree: &mut NodeRef<T>) {
        let node = tree.as_mut().unwrap();
        let left = Self::height(&node.left);
        let right = Self::height(&node.right);
        let rebalance_factor = left as i8 - right as i8;
        if rebalance_factor == 2 {
            let left = node.left.as_mut().unwrap();
            let left_left = Self::height(&left.left);
            let left_right = Self::height(&left.right);
            let left_rebalance_factor = left_left as i8 - left_right as i8;
            if left_rebalance_factor == -1 {
                Self::left_rotate(&mut node.left);
            }
            Self::right_rotate(tree);
        } else if rebalance_factor == -2 {
            let right = node.right.as_mut().unwrap();
            let right_left = Self::height(&right.left);
            let right_right = Self::height(&right.right);
            let right_rebalance_factor = right_left as i8 - right_right as i8;

            if right_rebalance_factor == 1 {
                Self::right_rotate(&mut node.right);
            }
            Self::left_rotate(tree);
        }
    }

    fn insert_node(root: &mut Node<T>, value: T) {
        let target = match value.cmp(&root.value) {
            Ordering::Less => &mut root.left,
            Ordering::Greater => &mut root.right,
            Ordering::Equal => return,
        };

        match target {
            Some(node) => Self::insert_node(node, value),
            None => *target = Node::new_node_ref(value),
        }

        Self::rebalance(target);
    }

    fn remove_node(tree: &mut NodeRef<T>, value: &T) -> Option<T> {
        let node = tree.as_mut()?;
        let temp = match node.value.cmp(value) {
            Ordering::Less => &mut node.right,
            Ordering::Greater => &mut node.left,
            Ordering::Equal => return Node::remove(tree),
        };
        let result = Self::remove_node(temp, value);
        if temp.is_some() {
            Self::rebalance(temp);
        }
        result
    }
}
