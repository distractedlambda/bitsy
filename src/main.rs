#![feature(unchecked_math)]
#![feature(variant_count)]

use batch::Batch;
use rand::{thread_rng, Rng};

use crate::{
    decider::Decider,
    ground_truth::{compute_ground_truth, total_loss},
    op::Op,
};

mod batch;
mod decider;
mod ground_truth;
mod op;

const BATCH_SIZE: usize = 32;

fn main() {
    let max_ops = 16;
    let n_batches = 1024;

    let mut rng = thread_rng();

    let mut blend_src_batches = Vec::with_capacity(n_batches);
    blend_src_batches.resize_with(n_batches, || rng.gen());

    let mut blend_dst_batches = Vec::with_capacity(n_batches);
    blend_dst_batches.resize_with(n_batches, || rng.gen());

    let mut truth_batches = Vec::with_capacity(n_batches);
    for i in 0..n_batches {
        truth_batches.push(Batch::from([0; BATCH_SIZE]));
        compute_ground_truth(
            truth_batches.last_mut().unwrap(),
            &blend_src_batches[i],
            &blend_dst_batches[i],
        );
    }

    let mut best_loss = u64::MAX;
    let mut op_data = Vec::new();
    let mut ops = Vec::new();
    let mut decider = Decider::new(rng);

    loop {
        ops.clear();

        let n_ops = decider.decide_range(1..=max_ops);
        for _ in 0..n_ops {
            ops.push(Op::new(&mut decider, ops.len() + 2))
        }

        let mut loss = 0;

        for i in 0..n_batches {
            op_data.clear();

            op_data.push(blend_src_batches[i]);
            op_data.push(blend_dst_batches[i]);

            for op in &ops {
                op_data.push(Batch::from([0; BATCH_SIZE]));
                let (dst, srcs) = op_data.split_last_mut().unwrap();
                op.evaluate(dst, srcs)
            }

            loss += total_loss(&truth_batches[i], op_data.last().unwrap());
        }

        if loss < best_loss {
            best_loss = loss;
            println!(
                "New best, avg. loss = {}: {:?}",
                (loss as f64) / ((BATCH_SIZE * n_batches) as f64),
                ops
            )
        }

        decider.restart(loss);
    }
}
