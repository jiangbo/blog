use std::cmp::Ordering;

use super::{binary_search_tree::BinarySearchTree, Node, NodeRef, Tree};

#[derive(Default)]
pub struct SplayTree<T> {
    tree: BinarySearchTree<T>,
}

impl<T: Ord> Tree<T> for SplayTree<T> {
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
        Self::splay(self.root_mut(), &value);
        let root = self.root_mut().take();
        *self.root_mut() = match root {
            None => Node::new_node_ref(value),
            Some(mut node) => match node.value.cmp(&value) {
                Ordering::Equal => Some(node),
                Ordering::Less => Some(Box::new(Node {
                    value,
                    right: node.right.take(),
                    left: Some(node),
                })),
                Ordering::Greater => Some(Box::new(Node {
                    value,
                    left: node.left.take(),
                    right: Some(node),
                })),
            },
        }
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        Self::splay(self.root_mut(), value);
        let node = self.root_mut().as_mut()?;
        match node.value.cmp(value) {
            Ordering::Equal => Node::remove(self.root_mut()),
            _ => None,
        }
    }

    fn contains(&mut self, value: &T) -> bool {
        Self::splay(self.root_mut(), value);
        self.root()
            .as_ref()
            .map_or(false, |node| &node.value == value)
    }
}

impl<T: Ord> SplayTree<T> {
    fn root(&self) -> &NodeRef<T> {
        self.tree.root()
    }
    fn root_mut(&mut self) -> &mut NodeRef<T> {
        self.tree.root_mut()
    }

    fn splay(tree: &mut NodeRef<T>, value: &T) {
        if let Some(grandparent) = tree.as_mut() {
            match grandparent.value.cmp(value) {
                Ordering::Greater => Self::splay_left(tree, value),
                Ordering::Less => Self::splay_right(tree, value),
                Ordering::Equal => (),
            }
        }
    }

    fn splay_left(tree: &mut NodeRef<T>, value: &T) {
        let grandparent = tree.as_mut().unwrap();
        if let Some(parent) = grandparent.left.as_mut() {
            match parent.value.cmp(value) {
                Ordering::Greater => {
                    Self::splay(&mut parent.left, value);
                    Node::right_rotate(tree);
                }
                Ordering::Less => {
                    Self::splay(&mut parent.right, value);
                    Node::left_rotate(tree);
                }
                Ordering::Equal => (),
            }
            Node::right_rotate(tree);
        }
    }

    fn splay_right(tree: &mut NodeRef<T>, value: &T) {
        let grandparent = tree.as_mut().unwrap();
        if let Some(parent) = grandparent.right.as_mut() {
            match parent.value.cmp(value) {
                Ordering::Greater => {
                    Self::splay(&mut parent.left, value);
                    Node::right_rotate(tree);
                }
                Ordering::Less => {
                    Self::splay(&mut parent.right, value);
                    Node::left_rotate(tree);
                }
                Ordering::Equal => (),
            }
            Node::left_rotate(tree);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splay_插入() {
        let mut tree = SplayTree::default();
        // (0..10).for_each(|e| tree.insert(e));
        // assert!(tree.pre_order().windows(2).all(|w| w[0] >= w[1]));
        // assert!(tree.in_order().windows(2).all(|w| w[0] <= w[1]))
        (0..4).for_each(|e| tree.insert(e));
        println!("{:?}", tree.pre_order());
        tree.contains(&0);
        println!("{:?}", tree.pre_order());
    }

    #[test]
    fn splay_检索() {
        let mut tree = SplayTree::default();
        (0..10).for_each(|e| tree.insert(e));
        assert!(tree.contains(&0));
        let mut expected = vec![&0, &8, &6, &4, &2, &1, &3, &5, &7, &9];
        assert_eq!(tree.pre_order(), expected);
        assert!(tree.contains(&9));
        expected = vec![&9, &8, &0, &6, &4, &2, &1, &3, &5, &7];
        assert_eq!(tree.pre_order(), expected);
        assert!(!tree.contains(&-1));
        expected = vec![&0, &8, &6, &4, &2, &1, &3, &5, &7, &9];
        assert_eq!(tree.pre_order(), expected);
    }

    #[test]
    fn splay_删除() {
        let mut tree = SplayTree::default();
        (0..10).for_each(|e| tree.insert(e));
        // assert_eq!(tree.remove(&0), Some(0));
        // let mut arr = [8, 6, 4, 2, 1, 3, 5, 7, 9];
        // assert_eq!(tree.pre_order(), arr.iter().collect::<Vec<_>>());
        // assert_eq!(tree.remove(&10), None);
        tree.contains(&9);
        println!("{:?}", tree.pre_order());
        tree.contains(&0);
        println!("{:?}", tree.pre_order());
    }
}
