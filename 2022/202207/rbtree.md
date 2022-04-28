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
        let parent = Self::parent(node);
        match parent {
            // 没有父节点，表明当前是根节点，只需要将节点染黑
            None => node.as_mut().unwrap().color = Color::Black,
            // 有父节点
            Some(parent) => {
                // 父节点是黑色的，所有红黑树的规则都满足，不需要处理。
                // 父节点是红色的，则祖父节点必然存在，因为红色节点不能是根节点
                if let Color::Red = (*parent).as_ref().unwrap().color {
                    let uncle = Self::brother(parent);
                    if !uncle.is_null() {
                        match (*uncle).as_ref().unwrap().color {
                            Color::Red => {
                                // 如果是红色的叔叔节点，需要将祖父节点染红，父和叔节点染黑
                                Self::set_color(parent, Color::Black);
                                Self::set_color(uncle, Color::Black);
                                let grandparent = Self::parent(parent).unwrap();
                                Self::set_color(grandparent, Color::Red);
                            }
                            // 祖父、父亲、叔父都是黑色
                            Color::Black => Self::fix_black_uncle(node),
                        }
                    }
                }
            }
        }
    }

    unsafe fn fix_black_uncle(node: &mut NodeRef<T>) {
        let p = Self::parent(node).unwrap();
        let g = Self::parent(p).unwrap();
        let left: *mut NodeRef<T> = &mut (*g).as_mut().unwrap().left;

        if Self::is_left_child(g, p) {
            // 祖父的左节点是父节点
            if Self::is_left_child(p, node) {
                // 父亲的左节点是当前孩子节点(LL)
            }
        }
    }

    unsafe fn parent(node: *mut NodeRef<T>) -> Option<*mut NodeRef<T>> {
        let node = node.as_mut()?;
        match node.as_ref().unwrap().parent.is_null() {
            true => None,
            false => Some(node.as_ref().unwrap().parent),
        }
    }
    unsafe fn brother(p: *mut NodeRef<T>) -> *mut NodeRef<T> {
        let g = &mut *Self::parent(p).unwrap();
        match &g.as_ref().unwrap().left {
            None => &mut g.as_mut().unwrap().left,
            Some(l) => match l.value == (*p).as_ref().unwrap().value {
                false => &mut g.as_mut().unwrap().left,
                true => &mut g.as_mut().unwrap().right,
            },
        }
    }

    unsafe fn set_color(node: *mut NodeRef<T>, color: Color) {
        (*node).as_mut().unwrap().color = color
    }

    unsafe fn is_left_child(parent: *mut NodeRef<T>, child: *mut NodeRef<T>) -> bool {
        let parent = (*parent).as_ref().unwrap();

        match &parent.left {
            None => false,
            Some(left) => {
                let child = (*child).as_ref().unwrap();
                left.value == child.value
            }
        }
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

    fn right_rotate(tree: *mut NodeRef<T>) {
        let root = unsafe { &mut *tree };
        if let Some(mut node) = root.take() {
            if let Some(mut new_root) = node.left.take() {
                node.left = new_root.right.take();
                new_root.right = Some(node);
                *root = Some(new_root);
            }
        }
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
