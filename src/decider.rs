use std::{fmt::Debug, ops::RangeInclusive, ptr::NonNull};

use num::PrimInt;
use rand::Rng;

pub struct Decider<R> {
    rng: R,
    root_node: NonNull<Node>,
    current_node: NonNull<Node>,
    history: Vec<NonNull<Node>>,
}

unsafe impl<R: Send> Send for Decider<R> {}

unsafe impl<R: Sync> Sync for Decider<R> {}

impl<R> Drop for Decider<R> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.root_node.as_ptr());
        }
    }
}

struct Node {
    min_loss: u64,
    false_path: Option<NonNull<Node>>,
    true_path: Option<NonNull<Node>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe {
            if let Some(node) = self.false_path {
                let _ = Box::from_raw(node.as_ptr());
            }

            if let Some(node) = self.true_path {
                let _ = Box::from_raw(node.as_ptr());
            }
        }
    }
}

impl Node {
    fn new() -> Self {
        Self {
            min_loss: u64::MAX,
            false_path: None,
            true_path: None,
        }
    }
}

impl<R: Rng> Decider<R> {
    pub fn new(rng: R) -> Self {
        let root_node = Box::new(Node::new());
        let root_node = Box::into_raw(root_node);
        let root_node = unsafe { NonNull::new_unchecked(root_node) };
        Self {
            rng,
            root_node,
            current_node: root_node,
            history: Vec::new(),
        }
    }

    pub fn new_ground(&self) -> bool {
        unsafe { self.current_node.as_ref().min_loss == u64::MAX }
    }

    pub fn restart(&mut self, loss: u64) {
        unsafe {
            self.current_node.as_mut().min_loss = loss;

            while let Some(mut node) = self.history.pop() {
                let node = node.as_mut();
                node.min_loss = node.min_loss.min(loss);
            }

            self.current_node = self.root_node;
        }
    }

    pub fn decide(&mut self) -> bool {
        self.history.push(self.current_node);
        unsafe {
            let node = self.current_node.as_mut();

            let true_prob =
                if let (Some(false_path), Some(true_path)) = (node.false_path, node.true_path) {
                    let false_path_loss = false_path.as_ref().min_loss;
                    let true_path_loss = true_path.as_ref().min_loss;
                    (false_path_loss as f64) / ((false_path_loss + true_path_loss) as f64)
                } else {
                    0.5
                };

            let choice = self.rng.gen_bool(true_prob);

            let edge = if choice {
                &mut node.true_path
            } else {
                &mut node.false_path
            };

            if let Some(node) = *edge {
                self.current_node = node;
            } else {
                let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node::new())));
                *edge = Some(node);
                self.current_node = node;
            }

            choice
        }
    }

    pub fn decide_range<I: PrimInt + Debug>(&mut self, range: RangeInclusive<I>) -> I {
        debug_assert!(range.start() <= range.end());

        let mut range = range;

        while range.start() != range.end() {
            let half = (range
                .end()
                .saturating_sub(*range.start())
                .saturating_add(I::one()))
                >> 1;
            if self.decide() {
                range = (*range.start() + half)..=*range.end();
            } else {
                range = *range.start()..=(*range.end() - half);
            }
        }

        *range.start()
    }
}
