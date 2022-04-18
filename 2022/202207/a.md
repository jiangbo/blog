use std::{cmp::Ordering, fmt::Debug};

fn main() {
    let mut tree = AVLTree::new();
    (0..10).for_each(|i| tree.insert(i));
    tree.pre_order();
    println!();
    tree.in_order();
    println!();
    tree.remove(&0);
    tree.pre_order();
    println!();
    tree.in_order();
    println!("remove 2");
    tree.remove(&2);
    tree.pre_order();
    println!();
    tree.in_order();
}

type NodeRef<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
struct Node<T: Ord + Debug> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

impl<T: Ord + Debug> Node<T> {
    fn new_node_ref(value: T) -> NodeRef<T> {
        Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }))
    }

    fn get_max(root: &mut NodeRef<T>) -> Option<T> {
        let mut current = root;
        while let Some(node) = current {
            current = match node.right {
                Some(_) => &mut current.as_mut()?.right,
                None => break,
            }
        }
        let node = current.take()?;
        *current = node.left;
        Some(node.value)
    }

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

    fn height(root: &NodeRef<T>) -> usize {
        match root {
            Some(node) => {
                let left = Node::height(&node.left);
                let right = Node::height(&node.right);
                1 + std::cmp::max(left, right)
            }
            None => 0,
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

    fn right_rotate(root: &mut NodeRef<T>) {
        if let Some(mut node) = root.take() {
            if let Some(mut new_root) = node.left.take() {
                node.left = new_root.right.take();
                new_root.right = Some(node);
                *root = Some(new_root);
            }
        }
    }

    fn rebalance(tree: &mut NodeRef<T>) {
        let node = tree.as_mut().unwrap();
        let left = Node::height(&node.left);
        let right = Node::height(&node.right);
        let rebalance_factor = left as i8 - right as i8;
        println!("rebalance_factor:{rebalance_factor}");
        if rebalance_factor == 2 {
            let left = node.left.as_mut().unwrap();
            let left_left = Node::height(&left.left);
            let left_right = Node::height(&left.right);
            let left_rebalance_factor = left_left as i8 - left_right as i8;
            if left_rebalance_factor == -1 {
                Node::left_rotate(&mut node.left);
            }
            Node::right_rotate(tree);
        } else if rebalance_factor == -2 {
            let right = node.right.as_mut().unwrap();
            let right_left = Node::height(&right.left);
            let right_right = Node::height(&right.right);
            let right_rebalance_factor = right_left as i8 - right_right as i8;

            if right_rebalance_factor == 1 {
                Node::right_rotate(&mut node.right);
            }
            Node::left_rotate(tree);
        }
    }

    fn remove(root: &mut NodeRef<T>, value: &T) -> Option<T> {
        let node = root.as_mut()?;
        let temp = match node.value.cmp(value) {
            Ordering::Less => &mut node.right,
            Ordering::Greater => &mut node.left,
            Ordering::Equal => return Self::remove_node(root),
        };
        println!("temp:{:?}", temp);
        let result = Node::remove(temp, value);
        if temp.is_some() {
            Self::rebalance(temp);
        }
        result
    }

    fn remove_node(root: &mut NodeRef<T>) -> Option<T> {
        let mut node = root.take()?;
        *root = match (node.left.as_ref(), node.right.as_ref()) {
            (None, None) => None,
            (Some(_), None) => node.left.take(),
            (None, Some(_)) => node.right.take(),
            (Some(_), Some(_)) => Some(Box::new(Node {
                value: Node::get_min(&mut node.right)?,
                left: node.left.take(),
                right: node.right.take(),
            })),
        };
        Some(node.value)
    }
}

struct AVLTree<T: Ord + Debug> {
    root: NodeRef<T>,
}

impl<T: Ord + Debug> AVLTree<T> {
    fn new() -> Self {
        AVLTree { root: None }
    }

    fn pre_order(&self) {
        let mut stack = vec![&self.root];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                println!("{:?}", node.value);
                stack.push(&node.right);
                stack.push(&node.left);
            }
        }
    }

    fn in_order(&self) {
        let (mut stack, mut current) = (Vec::new(), &self.root);
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

    fn insert(&mut self, value: T) {
        match self.root.as_mut() {
            Some(node) => Self::insert0(node, value),
            None => self.root = Node::new_node_ref(value),
        }
        Node::rebalance(&mut self.root);
    }

    fn insert0(root: &mut Node<T>, value: T) {
        let target = match value.cmp(&root.value) {
            Ordering::Less => &mut root.left,
            Ordering::Greater => &mut root.right,
            // 相等元素不插入
            Ordering::Equal => return,
        };

        match target {
            Some(node) => Self::insert0(node, value),
            None => *target = Node::new_node_ref(value),
        }

        Node::rebalance(target);
    }
    fn contains(&self, value: &T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Less => &node.left,
                Ordering::Greater => &node.right,
                Ordering::Equal => return true,
            };
        }
        false
    }

    fn max(&self) -> Option<&T> {
        self.max_or_min(|node| &node.right)
    }
    fn min(&self) -> Option<&T> {
        self.max_or_min(|node| &node.left)
    }

    fn max_or_min<F>(&self, child: F) -> Option<&T>
    where
        F: Fn(&Box<Node<T>>) -> &NodeRef<T>,
    {
        let mut current = &self.root;
        while let Some(node) = current {
            current = match child(node) {
                Some(_) => child(node),
                None => return Some(&node.value),
            }
        }
        None
    }

    fn get_max(&mut self) -> Option<T> {
        Node::get_max(&mut self.root)
    }

    fn get_min(&mut self) -> Option<T> {
        Node::get_min(&mut self.root)
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        let result = Node::remove(&mut self.root, value);
        if self.root.is_some() {
            Node::rebalance(&mut self.root);
        }
        result
    }
}
