#![feature(unchecked_math)]
#![feature(array_zip)]
#![feature(new_uninit)]
#![feature(get_mut_unchecked)]

use std::rc::Rc;

use rand::{thread_rng, Rng};

use crate::{
    decider::Decider,
    ground_truth::{compute_ground_truth, total_loss},
    op::Op,
};

mod decider;
mod ground_truth;
mod immediate_shift;
mod op;

fn create_batch(batch_size: usize, f: impl FnOnce(&mut [u32])) -> Rc<[u32]> {
    // FIXME: need a different way to alloc uninitialized memory
    let mut batch = unsafe { Rc::new_uninit_slice(batch_size).assume_init() };
    unsafe { f(Rc::get_mut_unchecked(&mut batch)) };
    batch
}

fn main() {
    let batch_size = 64;
    let n_batches = 64;

    let mut rng = thread_rng();

    let mut blend_src_batches = Vec::new();
    blend_src_batches.resize_with(n_batches, || {
        create_batch(batch_size, |dst| dst.fill_with(|| rng.gen()))
    });

    let mut blend_dst_batches = Vec::new();
    blend_dst_batches.resize_with(n_batches, || {
        create_batch(batch_size, |dst| dst.fill_with(|| rng.gen()))
    });

    let mut truth_batches = Vec::new();
    for i in 0..n_batches {
        truth_batches.push(create_batch(batch_size, |dst| {
            compute_ground_truth(dst, &blend_src_batches[i], &blend_dst_batches[i])
        }))
    }

    let mut best_loss = u64::MAX;
    let mut op_data = Vec::new();
    let mut ops = Vec::new();
    let mut decider = Decider::new(rng);

    loop {
        ops.clear();

        while !decider.new_ground() {
            ops.push(Op::new(&mut decider, ops.len() + 2))
        }

        let mut loss = 0;

        for i in 0..n_batches {
            op_data.clear();

            op_data.push(blend_src_batches[i].clone());
            op_data.push(blend_dst_batches[i].clone());

            for op in &ops {
                let op_result = create_batch(batch_size, |dst| op.evaluate(dst, &op_data));
                op_data.push(op_result)
            }

            loss += total_loss(&truth_batches[i], &op_data.last().unwrap());
        }

        if loss < best_loss {
            best_loss = loss;
            println!(
                "New best, avg. loss = {}: {:?}",
                (loss as f64) / ((batch_size * n_batches) as f64),
                ops
            )
        }

        decider.restart(loss);
    }
}
