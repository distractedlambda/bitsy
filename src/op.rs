use std::mem::{transmute, variant_count};

use rand::Rng;

use crate::{batch::Batch, decider::Decider};

#[derive(Copy, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Op(Case);

#[derive(Copy, Debug, PartialEq, Eq, Clone, Hash)]
struct OpId(usize);

#[derive(Copy, Debug, PartialEq, Eq, Clone, Hash)]
enum Case {
    Constant(u32),
    Unary(UnaryOpcode, OpId),
    Binary(BinaryOpcode, OpId, OpId),
}

#[repr(usize)]
#[allow(unused)]
#[derive(Copy, Debug, PartialEq, Eq, Clone, Hash)]
enum UnaryOpcode {
    Clz,
    Neg,
    ByteReverse,
    BitReverse,
    BitwiseNeg,
    Sext16,
    Sext8,
}

#[repr(usize)]
#[allow(unused)]
#[derive(Copy, Debug, PartialEq, Eq, Clone, Hash)]
enum BinaryOpcode {
    Add,
    And,
    Asr,
    Lsl,
    Lsr,
    Mul,
    Or,
    Xor,
    Sub,
    RotateRight,
    UAdd8,
    UAdd16,
}

impl UnaryOpcode {
    fn decide<R: Rng>(decider: &mut Decider<R>) -> Self {
        unsafe { transmute(decider.decide_range(0..=(variant_count::<Self>() - 1))) }
    }
}

impl BinaryOpcode {
    fn decide<R: Rng>(decider: &mut Decider<R>) -> Self {
        unsafe { transmute(decider.decide_range(0..=(variant_count::<Self>() - 1))) }
    }
}

impl Op {
    pub fn new<R: Rng>(decider: &mut Decider<R>, num_existing_ops: usize) -> Self {
        Self(
            match decider.decide_range(0..=(variant_count::<Case>() - 1)) {
                0 => Case::Constant(decider.decide_range(0..=u32::MAX)),

                1 => {
                    let opcode = UnaryOpcode::decide(decider);
                    let operand = OpId(decider.decide_range(0..=(num_existing_ops - 1)));
                    Case::Unary(opcode, operand)
                }

                2 => {
                    let opcode = BinaryOpcode::decide(decider);
                    let lhs = OpId(decider.decide_range(0..=(num_existing_ops - 1)));
                    let rhs = OpId(decider.decide_range(0..=(num_existing_ops - 1)));
                    Case::Binary(opcode, lhs, rhs)
                }

                _ => unreachable!(),
            },
        )
    }

    pub fn evaluate<const N: usize>(&self, dst: &mut Batch<u32, N>, srcs: &[Batch<u32, N>]) {
        match self.0 {
            Case::Constant(value) => dst.fill(value),

            Case::Unary(opcode, OpId(operand)) => {
                let operand = &srcs[operand];
                match opcode {
                    UnaryOpcode::Clz => {
                        for i in 0..N {
                            dst[i] = operand[i].leading_zeros()
                        }
                    }

                    UnaryOpcode::Neg => {
                        for i in 0..N {
                            dst[i] = operand[i].wrapping_neg()
                        }
                    }

                    UnaryOpcode::ByteReverse => {
                        for i in 0..N {
                            dst[i] = operand[i].swap_bytes()
                        }
                    }

                    UnaryOpcode::BitReverse => {
                        for i in 0..N {
                            dst[i] = operand[i].reverse_bits()
                        }
                    }

                    UnaryOpcode::BitwiseNeg => {
                        for i in 0..N {
                            dst[i] = !operand[i]
                        }
                    }

                    UnaryOpcode::Sext16 => {
                        for i in 0..N {
                            dst[i] = operand[i] as u16 as i16 as i32 as u32
                        }
                    }

                    UnaryOpcode::Sext8 => {
                        for i in 0..N {
                            dst[i] = operand[i] as u8 as i8 as i32 as u32
                        }
                    }
                }
            }

            Case::Binary(opcode, OpId(lhs), OpId(rhs)) => {
                let lhs = &srcs[lhs];
                let rhs = &srcs[rhs];
                match opcode {
                    BinaryOpcode::Add => {
                        for i in 0..N {
                            dst[i] = lhs[i].wrapping_add(rhs[i]);
                        }
                    }

                    BinaryOpcode::And => {
                        for i in 0..N {
                            dst[i] = lhs[i] & rhs[i];
                        }
                    }

                    BinaryOpcode::Asr => {
                        for i in 0..N {
                            let shift = rhs[i] & 0xff;
                            dst[i] = unsafe {
                                // is this correct?
                                (lhs[i] as i32).unchecked_shr((shift as i32).min(31)) as u32
                            }
                        }
                    }

                    BinaryOpcode::Xor => {
                        for i in 0..N {
                            dst[i] = lhs[i] ^ rhs[i]
                        }
                    }

                    BinaryOpcode::Lsl => {
                        for i in 0..N {
                            let shift = rhs[i] & 0xff;
                            dst[i] = if shift >= 32 {
                                0
                            } else {
                                unsafe { lhs[i].unchecked_shl(shift) }
                            }
                        }
                    }

                    BinaryOpcode::Lsr => {
                        for i in 0..N {
                            let shift = rhs[i] & 0xff;
                            dst[i] = if shift >= 32 {
                                0
                            } else {
                                unsafe { lhs[i].unchecked_shr(shift) }
                            }
                        }
                    }

                    BinaryOpcode::Mul => {
                        for i in 0..N {
                            dst[i] = lhs[i].wrapping_mul(rhs[i])
                        }
                    }

                    BinaryOpcode::Or => {
                        for i in 0..N {
                            dst[i] = lhs[i] | rhs[i]
                        }
                    }

                    BinaryOpcode::Sub => {
                        for i in 0..N {
                            dst[i] = lhs[i].wrapping_sub(rhs[i])
                        }
                    }

                    BinaryOpcode::RotateRight => {
                        for i in 0..N {
                            dst[i] = lhs[i].rotate_right(rhs[i])
                        }
                    }

                    BinaryOpcode::UAdd8 => {
                        for i in 0..N {
                            let lhs_bytes = lhs[i].to_ne_bytes();
                            let rhs_bytes = rhs[i].to_ne_bytes();
                            dst[i] = u32::from_ne_bytes([
                                lhs_bytes[0].wrapping_add(rhs_bytes[0]),
                                lhs_bytes[1].wrapping_add(rhs_bytes[1]),
                                lhs_bytes[2].wrapping_add(rhs_bytes[2]),
                                lhs_bytes[3].wrapping_add(rhs_bytes[3]),
                            ])
                        }
                    }

                    BinaryOpcode::UAdd16 => {
                        for i in 0..N {
                            dst[i] = (((lhs[i] & 0xff_ff) + (rhs[i] & 0xff_ff)) & 0xff_ff)
                                | (((lhs[i] >> 16) + (rhs[i] >> 16)) << 16)
                        }
                    }
                }
            }
        }
    }
}
