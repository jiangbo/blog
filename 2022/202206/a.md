use std::{cmp::Ordering, fmt::Debug};

fn main() {
    let mut tree = BinarySearchTree::new();
    vec![44, 22, 11, 33, 66, 66, 55, 77]
        .into_iter()
        .for_each(|e| tree.insert(e));
    tree.in_order();
    println!("{:?}", tree.search(&88));
    println!("{:?}", tree.search(&77));
    println!("{:?}", tree.max());
    println!("{:?}", tree.min());
    println!("{:?}", tree.get_max());
    println!("{:?}", tree.get_min());
    tree.in_order();
}

type NodeRef<T> = Option<Box<Node<T>>>;
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
                Some(_) => &mut current.as_mut().unwrap().right,
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
                Some(_) => &mut current.as_mut().unwrap().left,
                None => break,
            }
        }
        let node = current.take()?;
        *current = node.right;
        Some(node.value)
    }
}

struct BinarySearchTree<T: Ord + Debug> {
    root: NodeRef<T>,
}

impl<T: Ord + Debug> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
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
        let mut current = &mut self.root;
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
    fn search(&self, value: &T) -> bool {
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
        let mut current = &mut self.root;
        while let Some(node) = current.as_mut() {
            current = match node.value.cmp(value) {
                Ordering::Less => &mut current,
                Ordering::Greater => &mut node.left,
                Ordering::Equal => break,
            }
        }

        let mut node = current.take()?;

        *current = match (node.left.as_mut(), node.right.as_mut()) {
            (None, None) => None,
            (Some(_), None) => node.left.take(),
            (None, Some(_)) => node.right.take(),
            (Some(_), Some(_)) => {
                Node::get_max(&mut node.right);
                None
            }
        };

        None
    }
}
