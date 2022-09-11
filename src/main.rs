#![feature(unchecked_math)]
#![feature(array_zip)]

use rand::{thread_rng, Rng};

use crate::{
    ground_truth::{compute_ground_truth, total_loss},
    op::Op,
};

mod ground_truth;
mod immediate_shift;
mod op;

struct ReferenceData(Box<[u32]>);

impl ReferenceData {
    fn generate(rng: &mut impl Rng, n_samples: usize) -> Self {
        let mut data = Vec::new();
        data.resize(n_samples * 3, 0);
        let mut data = data.into_boxed_slice();

        for sample in 0..n_samples {
            data[sample] = rng.gen();
            data[n_samples + sample] = rng.gen();
        }

        let (inputs, ground_truth) = data.split_at_mut(n_samples * 2);
        compute_ground_truth(ground_truth, &inputs[..n_samples], &inputs[n_samples..]);

        Self(data)
    }

    fn blend_src(&self) -> &[u32] {
        &self.0[..self.0.len() / 3]
    }

    fn blend_dst(&self) -> &[u32] {
        &self.0[(self.0.len() / 3)..(2 * (self.0.len() / 3))]
    }

    fn blend_res(&self) -> &[u32] {
        &self.0[(2 * (self.0.len() / 3))..]
    }
}

fn main() {
    let batch_size = 256;
    let n_batches = 1;
    let max_ops = 8;

    let mut rng = thread_rng();
    let reference_data = ReferenceData::generate(&mut rng, n_batches * batch_size);

    let mut best_loss = u64::MAX;

    loop {
        let mut op_data = Vec::new();
        let mut ops = Vec::new();

        let n_ops = rng.gen_range(1..=max_ops);
        for i in 0..n_ops {
            ops.push(Op::random(&mut rng, i + 2))
        }

        let mut loss = 0;

        for i in 0..n_batches {
            op_data.push(Vec::from(
                &reference_data.blend_src()[(i * batch_size)..((i + 1) * batch_size)],
            ));

            op_data.push(Vec::from(
                &reference_data.blend_dst()[(i * batch_size)..((i + 1) * batch_size)],
            ));

            for op in &ops {
                let mut op_result = Vec::new();
                op_result.resize(batch_size, 0);
                op.evaluate(&mut op_result, &op_data);
                op_data.push(op_result);
            }

            loss += total_loss(
                &reference_data.blend_res()[(i * batch_size)..((i + 1) * batch_size)],
                &op_data.last().unwrap(),
            );
        }

        if loss < best_loss {
            best_loss = loss;
            println!(
                "New best, avg. loss = {}: {:?}",
                (loss as f64) / ((batch_size * n_batches) as f64),
                ops
            )
        }
    }
}
