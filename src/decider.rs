use std::{
    ops::{Bound, RangeBounds},
    ptr::NonNull,
};

use num::PrimInt;
use rand::Rng;

pub trait Decider {
    fn decide_bool(&mut self) -> bool;

    fn decide_range<I: PrimInt>(&mut self, range: impl RangeBounds<I>) -> I;
}

pub trait Decide {
    fn decide(decider: &mut impl Decider) -> Self;
}

pub struct TreeDecider<R> {
    rng: R,
    root_node: NonNull<Node>,
    current_node: NonNull<Node>,
    history: Vec<NonNull<Node>>,
}

unsafe impl<R: Send> Send for TreeDecider<R> {}

unsafe impl<R: Sync> Sync for TreeDecider<R> {}

impl<R> Drop for TreeDecider<R> {
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

impl<R: Rng> TreeDecider<R> {
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
}

impl<R: Rng> Decider for TreeDecider<R> {
    fn decide_bool(&mut self) -> bool {
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

    fn decide_range<I: PrimInt>(&mut self, range: impl RangeBounds<I>) -> I {
        let mut first = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + I::one(),
            Bound::Unbounded => I::min_value(),
        };

        let mut last = match range.end_bound() {
            Bound::Included(&end) => end,
            Bound::Excluded(&end) => end - I::one(),
            Bound::Unbounded => I::max_value(),
        };

        debug_assert!(first <= last);

        while first != last {
            let half = (last.saturating_sub(first).saturating_add(I::one())) >> 1;
            if self.decide_bool() {
                first = first + half;
            } else {
                last = last - half;
            }
        }

        first
    }
}

impl Decide for bool {
    fn decide(decider: &mut impl Decider) -> Self {
        decider.decide_bool()
    }
}

impl Decide for u8 {
    fn decide(decider: &mut impl Decider) -> Self {
        let mut result = 0;

        for i in (0..Self::BITS).rev() {
            result |= (decider.decide_bool() as Self) << i
        }

        result
    }
}

impl Decide for u16 {
    fn decide(decider: &mut impl Decider) -> Self {
        let mut result = 0;

        for i in (0..Self::BITS).rev() {
            result |= (decider.decide_bool() as Self) << i
        }

        result
    }
}

impl Decide for u32 {
    fn decide(decider: &mut impl Decider) -> Self {
        let mut result = 0;

        for i in (0..Self::BITS).rev() {
            result |= (decider.decide_bool() as Self) << i
        }

        result
    }
}

impl Decide for u64 {
    fn decide(decider: &mut impl Decider) -> Self {
        let mut result = 0;

        for i in (0..Self::BITS).rev() {
            result |= (decider.decide_bool() as Self) << i
        }

        result
    }
}

impl Decide for i8 {
    fn decide(decider: &mut impl Decider) -> Self {
        u8::decide(decider) as Self
    }
}

impl Decide for i16 {
    fn decide(decider: &mut impl Decider) -> Self {
        u16::decide(decider) as Self
    }
}

impl Decide for i32 {
    fn decide(decider: &mut impl Decider) -> Self {
        u32::decide(decider) as Self
    }
}

impl Decide for i64 {
    fn decide(decider: &mut impl Decider) -> Self {
        u64::decide(decider) as Self
    }
}
