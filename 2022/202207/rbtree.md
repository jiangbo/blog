use std::{cmp::Ordering, ptr::null_mut};

use super::{binary_search_tree::BinarySearchTree, Color, Node, NodeRef, Tree};

#[derive(Default)]
pub struct RedBlackTree<T: Ord> {
    tree: BinarySearchTree<T>,
}

impl<T: Ord> Tree<T> for RedBlackTree<T> {
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
        let (node, inserted) = unsafe { self.insert_node(value) };
        if inserted {
            unsafe { Self::fix_insert(node) };
        }
    }

    fn remove(&mut self, _: &T) -> Option<T> {
        todo!()
    }

    fn contains(&mut self, value: &T) -> bool {
        self.tree.contains(value)
    }
}

impl<T: Ord> RedBlackTree<T> {
    fn root_mut(&mut self) -> &mut NodeRef<T> {
        self.tree.root_mut()
    }

    unsafe fn insert_node(&mut self, value: T) -> (&mut NodeRef<T>, bool) {
        let mut current: *mut NodeRef<T> = self.root_mut();
        let mut parent: *mut NodeRef<T> = null_mut();
        while let Some(node) = &mut *current {
            parent = current;
            current = match value.cmp(&node.value) {
                Ordering::Less => &mut node.left,
                Ordering::Greater => &mut node.right,
                // 相等元素不插入
                Ordering::Equal => return (&mut *current, false),
            };
        }
        let current = &mut *current;
        *current = Node::new_node_ref(value);
        current.as_mut().unwrap().parent = parent;
        (current, true)
    }

    unsafe fn fix_insert(node: &mut NodeRef<T>) {
        if Self::has_parent(node) {
            node.as_mut().unwrap().color = Color::Black;
            return;
        }
        let parent = &mut *node.as_mut().unwrap().parent;
        if let Color::Black = parent.as_ref().unwrap().color {
            return;
        }
    }

    fn has_parent(node: &NodeRef<T>) -> bool {
        match node {
            Some(node) => !node.parent.is_null(),
            None => false,
        }
    }

    unsafe fn uncle(node: &NodeRef<T>) -> &NodeRef<T> {
        let parent = &*node.as_ref().unwrap().parent;
        let grandparent = &*parent.as_ref().unwrap().parent;
        let left = grandparent.as_ref().unwrap().left;
        let right = grandparent.as_ref().unwrap().right;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rbtree_插入() {
        let mut tree = RedBlackTree::default();
        (0..10).for_each(|e| tree.insert(e));
        assert!(tree.in_order().windows(2).all(|w| w[0] <= w[1]))
    }
}
