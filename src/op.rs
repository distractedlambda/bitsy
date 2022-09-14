use crate::{
    batch::Batch,
    decider::{Decide, Decider},
};

#[derive(Copy, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Op(Case);

#[derive(Copy, Debug, PartialEq, Eq, Clone, Hash)]
struct OpId(usize);

#[derive(Copy, Debug, PartialEq, Eq, Clone, Hash)]
enum Case {
    Add(OpId, OpId),
    And(OpId, OpId),
    AsrImm(OpId, u8),
    Constant(u32),
    LslImm(OpId, u8),
    LsrImm(OpId, u8),
    Mul(OpId, OpId),
    Neg(OpId),
    Not(OpId),
    Or(OpId, OpId),
    RorImm(OpId, u8),
    Sub(OpId, OpId),
    Xor(OpId, OpId),
}

impl Op {
    pub fn decide_additional<D: Decider>(decider: &mut D, num_existing_ops: usize) -> Self {
        let pick_op = |decider: &mut D| OpId(decider.decide_range(0..num_existing_ops));
        Op(match decider.decide_range(0..=12) {
            0 => Case::Add(pick_op(decider), pick_op(decider)),
            1 => Case::And(pick_op(decider), pick_op(decider)),
            2 => Case::AsrImm(pick_op(decider), decider.decide_range(1..=31)),
            3 => Case::Constant(Decide::decide(decider)),
            4 => Case::LslImm(pick_op(decider), decider.decide_range(1..=31)),
            5 => Case::LsrImm(pick_op(decider), decider.decide_range(1..=31)),
            6 => Case::Mul(pick_op(decider), pick_op(decider)),
            7 => Case::Neg(pick_op(decider)),
            8 => Case::Not(pick_op(decider)),
            9 => Case::Or(pick_op(decider), pick_op(decider)),
            10 => Case::RorImm(pick_op(decider), decider.decide_range(1..=31)),
            11 => Case::Sub(pick_op(decider), pick_op(decider)),
            12 => Case::Xor(pick_op(decider), pick_op(decider)),
            _ => unreachable!(),
        })
    }

    pub fn evaluate<const N: usize>(&self, dst: &mut Batch<u32, N>, srcs: &[Batch<u32, N>]) {
        match self.0 {
            Case::Add(OpId(lhs), OpId(rhs)) => {
                let lhs = &srcs[lhs];
                let rhs = &srcs[rhs];
                for i in 0..N {
                    dst[i] = lhs[i].wrapping_add(rhs[i])
                }
            }

            Case::And(OpId(lhs), OpId(rhs)) => {
                let lhs = &srcs[lhs];
                let rhs = &srcs[rhs];
                for i in 0..N {
                    dst[i] = lhs[i] & rhs[i]
                }
            }

            Case::AsrImm(OpId(lhs), rhs) => {
                let lhs = &srcs[lhs];
                for i in 0..N {
                    dst[i] = unsafe { (lhs[i] as i32).unchecked_shr(rhs as i32) as u32 }
                }
            }

            Case::Constant(value) => dst.fill(value),

            Case::LslImm(OpId(lhs), rhs) => {
                let lhs = &srcs[lhs];
                for i in 0..N {
                    dst[i] = unsafe { lhs[i].unchecked_shl(rhs as u32) }
                }
            }

            Case::LsrImm(OpId(lhs), rhs) => {
                let lhs = &srcs[lhs];
                for i in 0..N {
                    dst[i] = unsafe { lhs[i].unchecked_shr(rhs as u32) }
                }
            }

            Case::Mul(OpId(lhs), OpId(rhs)) => {
                let lhs = &srcs[lhs];
                let rhs = &srcs[rhs];
                for i in 0..N {
                    dst[i] = lhs[i].wrapping_mul(rhs[i])
                }
            }

            Case::Neg(OpId(operand)) => {
                let operand = &srcs[operand];
                for i in 0..N {
                    dst[i] = operand[i].wrapping_neg();
                }
            }

            Case::Not(OpId(operand)) => {
                let operand = &srcs[operand];
                for i in 0..N {
                    dst[i] = !operand[i];
                }
            }

            Case::Or(OpId(lhs), OpId(rhs)) => {
                let lhs = &srcs[lhs];
                let rhs = &srcs[rhs];
                for i in 0..N {
                    dst[i] = lhs[i] | rhs[i]
                }
            }

            Case::RorImm(OpId(lhs), rhs) => {
                let lhs = &srcs[lhs];
                for i in 0..N {
                    dst[i] = lhs[i].rotate_right(rhs as u32)
                }
            }

            Case::Sub(OpId(lhs), OpId(rhs)) => {
                let lhs = &srcs[lhs];
                let rhs = &srcs[rhs];
                for i in 0..N {
                    dst[i] = lhs[i].wrapping_sub(rhs[i])
                }
            }

            Case::Xor(OpId(lhs), OpId(rhs)) => {
                let lhs = &srcs[lhs];
                let rhs = &srcs[rhs];
                for i in 0..N {
                    dst[i] = lhs[i] ^ rhs[i]
                }
            }
        }
    }
}
