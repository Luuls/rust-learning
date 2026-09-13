pub mod tree {
use std::{cmp::{Eq, Ord, Ordering}, fmt::{Debug, Display}};

#[derive(Debug)]
pub struct BinaryTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
pub struct TreeNode<T> {
    value: T,
    count: u32, // Number of occurrences of this value
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}

impl<T: Ord + Eq + Copy> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree {
            root: None,
        }
    }

    /// Insert the passed value into the tree.
    pub fn insert(&mut self, value: T) {
        let mut current = &mut self.root;
        loop {
            match current {
                None => {
                    *current = Some(Box::new(TreeNode::new(value)));
                    return;
                }
                Some(node) => {
                    match value.cmp(&node.value) {
                        // if the value is already present, we just increment its counter
                        Ordering::Equal => return node.count += 1,
                        Ordering::Less => current = &mut node.left,
                        Ordering::Greater => current = &mut node.right,
                    };
                }
            }
        }
    }

    /// Removes and returns the passed value from the tree, if it's present.
    pub fn remove(&mut self, value: &T) -> Option<T> {
        let mut current = &mut self.root;

        loop {
            let ordering = match current.as_deref() {
                None => return None,
                Some(node) => value.cmp(&node.value),
            };

            match ordering {
                Ordering::Equal => break,
                Ordering::Less => current = &mut current.as_mut().unwrap().left,
                Ordering::Greater => current = &mut current.as_mut().unwrap().right,
            };
        }


        let node = current.as_mut().unwrap();
        if node.count > 1 {
            node.count -= 1;
            return Some(node.value.clone());
        }

        let removed = current.take().unwrap();
        let removed_value = removed.value;

        match (removed.left, removed.right) {
            // no children, return the removed value
            (None, None) => {},

            // if only left child, set it as the new child of the parent
            (Some(left), None) => *current = Some(left),

            // if only right child, set it as the new child of the parent
            (None, Some(right)) => *current = Some(right),

            // has both children, so we find the lowest from its right subtree (simplistic).
            (Some(left), Some(right)) => {
                let mut right_subtree = Some(right);
                let mut lowest_slot = &mut right_subtree;

                while lowest_slot.as_ref().unwrap().left.is_some() {
                    lowest_slot = &mut lowest_slot.as_mut().unwrap().left;
                }

                let mut lowest = lowest_slot.take().unwrap();
                *lowest_slot = lowest.right.take();

                lowest.left = Some(left);
                lowest.right = right_subtree;

                *current = Some(lowest);
            }
        }

        Some(removed_value)
    }

    /// Removes and returns the minimum value from the tree
    pub fn pop_min(&mut self) -> Option<T> {
        let mut current = &mut self.root;
        while current.as_ref()?.left.is_some() {
            current = &mut current.as_mut().unwrap().left;
        }

        let min_node = current.as_mut().unwrap();
        if min_node.count > 1 {
            min_node.count -= 1;
            return Some(min_node.value.clone());
        }

        let mut min_node = current.take().unwrap();

        // the min node might only have a right child, so we replace it with its child
        *current = min_node.right.take();

        Some(min_node.value)
    }
}

pub enum Error {
    NotFound,
}

impl<T: Display + Debug> Display for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let mut stack: Vec<&TreeNode<T>> = Vec::new();
        let mut current = self.root.as_deref();
        let mut first = true;

        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(node);
                current = node.left.as_deref();
            }

            let node = stack.pop().unwrap();

            if !first {
                write!(f, ", ")?;
            }

            // print as many node.value as node.count
            for _ in 1..node.count {
                write!(f, "{}, ", node.value)?
            }

            write!(f, "{}", node.value)?;

            first = false;

            current = node.right.as_deref();
        }

        write!(f, "]")
    }
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            count: 1,
            left: None,
            right: None,
        }
    }
}
}

fn main() {
    let mut tree: tree::BinaryTree<u32> = tree::BinaryTree::new();
    let numbers = vec![10, 3, 9, 2, 5, 3, 4, 1, 1, 2, 7];
    for number in numbers {
        tree.insert(number);
    }

    println!("{tree}");

    let to_remove = 7;
    match tree.remove(&to_remove) {
        Some(removed) => println!("Value removed: {removed}."),
        None => println!("The value {to_remove} don't exist in the tree."),
    };

    println!("{tree}");
    for _ in 0..5 {
        match tree.pop_min() {
            Some(removed) => println!("Min value removed: {removed}."),
            None => println!("Couldn't pop the min value, the tree was empty."),
        };
    }
    println!("{tree}");
}
