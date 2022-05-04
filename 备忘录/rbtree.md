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
                    match &mut *uncle {
                        None => Self::fix_black_uncle(node),
                        Some(u) => {
                            match u.color {
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
                    if !uncle.is_null() && (*uncle).is_some() {}
                }
            }
        }
    }

    unsafe fn fix_black_uncle(node: &mut NodeRef<T>) {
        let p = Self::parent(node).unwrap();
        let g = Self::parent(p).unwrap();

        if Self::is_left_child(g, p) {
            // 祖父的左节点是父节点
            if Self::is_right_child(p, node) {
                // 父亲的右节点是当前孩子节点(LR)，左旋成(LL)
                Self::left_rotate(p);
            }
            // 父亲的左节点是当前孩子节点(LL)
            Self::right_rotate(g);
            Self::set_color(g, Color::Black);
            Self::set_color(p, Color::Red);
        } else {
            // 祖父的右节点是父节点
            if Self::is_left_child(p, node) {
                // 父亲的左节点是当前孩子节点(RL)，右旋成(RR)
                Self::right_rotate(p);
            }
            // 父亲的右节点是当前孩子节点(RR)
            Self::left_rotate(g);
            Self::set_color(g, Color::Black);
            Self::set_color(p, Color::Red);
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
        if let Some(node) = (*node).as_mut() {
            node.color = color;
        }
    }

    unsafe fn is_left_child(parent: *mut NodeRef<T>, child: *mut NodeRef<T>) -> bool {
        !Self::is_right_child(parent, child)
    }

    unsafe fn is_right_child(parent: *mut NodeRef<T>, child: *mut NodeRef<T>) -> bool {
        let parent = (*parent).as_ref().unwrap();

        match &parent.right {
            None => false,
            Some(right) => {
                let child = (*child).as_ref().unwrap();
                right.value == child.value
            }
        }
    }

    unsafe fn left_rotate(tree: *mut NodeRef<T>) {
        let parent = Self::parent(tree);
        let root = &mut *tree;

        if let Some(mut node) = root.take() {
            if let Some(mut new_root) = node.right.take() {
                node.right = new_root.left.take();
                new_root.left = Some(node);
                *root = Some(new_root);
            }
        }

        match parent {
            Some(parent) => root.as_mut().unwrap().parent = parent,
            None => root.as_mut().unwrap().parent = null_mut(),
        }

        let left_pointer: *mut NodeRef<T> = &mut root.as_mut().unwrap().left;
        let left = (*left_pointer).as_mut().unwrap();
        left.parent = tree;

        let right_pointer: *mut NodeRef<T> = &mut root.as_mut().unwrap().right;
        let right = (*right_pointer).as_mut().unwrap();
        right.parent = tree;

        if let Some(right) = &mut left.right {
            right.parent = left_pointer;
        }
    }

    unsafe fn right_rotate(tree: *mut NodeRef<T>) {
        let parent = Self::parent(tree);
        let root = &mut *tree;
        if let Some(mut node) = root.take() {
            if let Some(mut new_root) = node.left.take() {
                node.left = new_root.right.take();
                new_root.right = Some(node);
                *root = Some(new_root);
            }
        }

        match parent {
            Some(parent) => root.as_mut().unwrap().parent = parent,
            None => root.as_mut().unwrap().parent = null_mut(),
        }

        let left_pointer: *mut NodeRef<T> = &mut root.as_mut().unwrap().left;
        let left = (*left_pointer).as_mut().unwrap();
        left.parent = tree;

        let right_pointer: *mut NodeRef<T> = &mut root.as_mut().unwrap().right;
        let right = (*right_pointer).as_mut().unwrap();
        right.parent = tree;
        if let Some(left) = &mut right.left {
            left.parent = right_pointer;
        }
    }

    fn pre_order1(&self) {
        let mut stack = vec![self.tree.root()];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                stack.push(&node.right);
                stack.push(&node.left);
                println!("node:{:p}, parent: {:?}", node, node.parent);
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
        assert!(tree.in_order().windows(2).all(|w| w[0] <= w[1]));
        println!("{:?}", tree.pre_order());
        tree.pre_order1();
    }
}

















use std::{
    cell::{RefCell, RefMut},
    cmp::Ordering,
    rc::{Rc, Weak},
};

use super::Tree;

pub enum Color {
    Red,
    Black,
}

struct Node<T> {
    value: T,
    color: Color,
    parent: Parent<T>,
    left: Child<T>,
    right: Child<T>,
}

impl<T> Node<T> {
    fn new_black_child(value: T) -> Child<T> {
        Self::new_child(value, Color::Black, None)
    }

    fn new_child_with_parent(value: T, parent: Parent<T>) -> Child<T> {
        Self::new_child(value, Color::Red, parent)
    }

    fn new_child(value: T, color: Color, parent: Parent<T>) -> Child<T> {
        Some(Rc::new(RefCell::new(Node {
            value,
            color,
            parent,
            left: None,
            right: None,
        })))
    }
}

type Child<T> = Option<Rc<RefCell<Node<T>>>>;
type Parent<T> = Option<Weak<RefCell<Node<T>>>>;

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
            Some(node) => Self::insert_node(node, value),
            None => self.root = Node::new_black_child(value),
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
    fn set_color(node: &Child<T>, color: Color) {
        if let Some(node) = node {
            node.borrow_mut().color = color;
        }
    }

    fn parent(node: &Child<T>) -> Child<T> {
        node.as_ref()?.as_ref().borrow().parent.as_ref()?.upgrade()
    }

    fn brother(node: &Child<T>) -> Child<T> {
        let parent = &Self::parent(node);

        if parent.is_none() || node.is_none() {
            return None;
        }

        let p = parent.as_ref().unwrap().as_ref().borrow();
        match Self::is_left_child(parent, node) {
            true => p.right.clone(),
            false => p.left.clone(),
        }
    }

    fn is_left_child(parent: &Child<T>, child: &Child<T>) -> bool {
        !Self::is_right_child(parent, child)
    }

    fn is_right_child(parent: &Child<T>, child: &Child<T>) -> bool {
        if parent.is_none() || child.is_none() {
            return false;
        }

        let parent = &parent.as_ref().unwrap().as_ref().borrow().value;
        let child = &child.as_ref().unwrap().as_ref().borrow().value;
        parent == child
    }

    // fn left_child(tree: &Child<T>) -> Child<T> {
    //     tree.as_ref()?.as_ref().borrow().left.clone()
    // }

    // fn left_child_node(tree: &Child<T>) -> Child<T> {
    //     tree.as_ref()?.as_ref().borrow().left.clone()
    // }

    // fn right_child(tree: &Child<T>) -> Child<T> {
    //     tree.as_ref()?.as_ref().borrow().right.clone()
    // }

    // fn right_child_node(tree: &Child<T>) -> Child<T> {
    //     tree.as_ref()?.as_ref().borrow().right.clone()
    // }

    fn node_mut(tree: &Child<T>) -> RefMut<Node<T>> {
        tree.as_ref().unwrap().as_ref().borrow_mut()
    }

    fn left_rotate(tree: &Child<T>) {
        if let Some(root1) = tree {
            let mut root2 = root1.as_ref().borrow_mut();
            let right1 = root2.right.clone();
            let parent = root2.parent.clone();
            if let Some(right2) = &right1 {
                let mut right3 = right2.as_ref().borrow_mut();
                root2.parent = Some(Rc::downgrade(right2));
                root2.right = right3.left.clone();
                right3.left = tree.clone();
                right2.swap(root1);
                right3.parent = parent;
                if right3.left.is_some() {
                    Self::node_mut(&right3.left).parent = Some(Rc::downgrade(root1));
                }
            }
        }
    }

    fn right_rotate(tree: &Child<T>) {
        if let Some(root1) = tree {
            let mut root2 = root1.as_ref().borrow_mut();
            let left1 = root2.left.clone();
            let parent = root2.parent.clone();
            if let Some(left2) = &left1 {
                let mut left3 = left2.as_ref().borrow_mut();
                root2.parent = Some(Rc::downgrade(left2));
                root2.left = left3.right.clone();
                left3.right = tree.clone();
                left2.swap(root1);
                left3.parent = parent;
                if left3.right.is_some() {
                    Self::node_mut(&left3.right).parent = Some(Rc::downgrade(root1));
                }
            }
        }
    }

    fn insert_node(node: &Rc<RefCell<Node<T>>>, value: T) {
        let borrow = node.borrow();
        let current: &Child<T> = match value.cmp(&borrow.value) {
            Ordering::Less => &borrow.left,
            Ordering::Greater => &borrow.right,
            Ordering::Equal => return,
        };

        match current {
            Some(node) => Self::insert_node(node, value),
            None => {
                let parent = Some(Rc::downgrade(node));
                let mut node = node.borrow_mut();
                if value < node.value {
                    node.left = Node::new_child_with_parent(value, parent);
                } else {
                    node.right = Node::new_child_with_parent(value, parent);
                }
                Self::fix_insert(current);
            }
        };
    }

    // fix red black tree insert
    fn fix_insert(node: &Child<T>) {
        let p = &Self::parent(node);

        if p.is_none() {
            // 没有父节点，表明当前是根节点，只需要将节点染黑
            Self::set_color(p, Color::Black);
            return;
        }

        // 有父节点
        let color = &p.as_ref().unwrap().as_ref().borrow().color;
        if matches!(color, Color::Black) {
            // 父节点是黑色的，所有红黑树的规则都满足，不需要处理。
            return;
        }

        // 父节点是红色的，则祖父节点必然存在，检查叔叔节点
        match &Self::brother(p) {
            None => Self::fix_black_uncle(node),
            Some(uncle) => {
                match uncle.as_ref().borrow().color {
                    Color::Red => Self::fix_red_uncle(p),
                    // 祖父、父亲、叔父都是黑色
                    Color::Black => Self::fix_black_uncle(node),
                }
            }
        }
    }

    fn fix_red_uncle(parent: &Child<T>) {
        // 如果是红色的叔叔节点，需要将祖父节点染红，父和叔节点染黑
        Self::set_color(parent, Color::Black);
        Self::set_color(&Self::brother(parent), Color::Black);
        Self::set_color(&Self::parent(parent), Color::Red);
    }
    fn fix_black_uncle(node: &Child<T>) {
        let p = &Self::parent(node);
        let g = &Self::parent(p);

        if Self::is_left_child(g, p) {
            // 祖父的左节点是父节点
            if Self::is_right_child(p, node) {
                // 父亲的右节点是当前孩子节点(LR)，左旋成(LL)
                Self::left_rotate(p);
            }
            // 父亲的左节点是当前孩子节点(LL)
            Self::right_rotate(g);
            Self::set_color(g, Color::Black);
            Self::set_color(p, Color::Red);
        } else {
            // 祖父的右节点是父节点
            if Self::is_left_child(p, node) {
                // 父亲的左节点是当前孩子节点(RL)，右旋成(RR)
                Self::right_rotate(p);
            }
            // 父亲的右节点是当前孩子节点(RR)
            Self::left_rotate(g);
            Self::set_color(g, Color::Black);
            Self::set_color(p, Color::Red);
        }
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
