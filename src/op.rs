use std::ops::{Deref, Range};

use rand::Rng;

use crate::{decider::Decider, immediate_shift::ImmediateShift};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Op(Case);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct OpId(usize);

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Case {
    AddImmediate(OpId, u32),
    AddRegister(OpId, OpId, ImmediateShift),
    AndImmediate(OpId, u32),
    AndRegister(OpId, OpId, ImmediateShift),
    AsrImmediate(OpId, u8),
    AsrRegister(OpId, OpId),
    Bfc(OpId, Range<u8>),
    Bfi(OpId, OpId, Range<u8>),
    BicImmediate(OpId, u32),
    BicRegister(OpId, OpId, ImmediateShift),
    Clz(OpId),
    EorImmediate(OpId, u32),
    EorRegister(OpId, OpId, ImmediateShift),
    LdrConstant(u32),
    LslImmediate(OpId, u8),
    LslRegister(OpId, OpId),
    LsrImmediate(OpId, u8),
    LsrRegister(OpId, OpId),
    Mla(OpId, OpId, OpId),
    Mls(OpId, OpId, OpId),
    MovImmediate(u32),
    Movt(OpId, u16),
    Mul(OpId, OpId),
    MvnImmediate(u32),
    MvnRegister(OpId, ImmediateShift),
    OrnImmediate(OpId, u32),
    OrnRegister(OpId, OpId, ImmediateShift),
    OrrImmediate(OpId, u32),
    OrrRegister(OpId, OpId, ImmediateShift),
    Pkh(OpId, OpId, i8),
    Qadd(OpId, OpId),
    Qadd16(OpId, OpId),
    Qadd8(OpId, OpId),
    Qasx(OpId, OpId),
    Qdadd(OpId, OpId),
    Qdsub(OpId, OpId),
    Qsax(OpId, OpId),
    Qsub(OpId, OpId),
    Qsub16(OpId, OpId),
    Qsub8(OpId, OpId),
    Rbit(OpId),
    Rev(OpId),
    Rev16(OpId),
    Revsh(OpId),
    RorImmediate(OpId, u8),
    RorRegister(OpId, OpId),
    RsbImmediate(OpId, u32),
    RsbRegister(OpId, OpId, ImmediateShift),
    Sadd16(OpId, OpId),
    Sadd8(OpId, OpId),
    Sasx(OpId, OpId),
    Sbfx(OpId, Range<u8>),
    Sdiv(OpId, OpId),
    Shadd16(OpId, OpId),
    Shadd8(OpId, OpId),
    Shasx(OpId, OpId),
    Shsax(OpId, OpId),
    Shsub16(OpId, OpId),
    Shsub8(OpId, OpId),
    Smlabb(OpId, OpId, OpId),
    Smlabt(OpId, OpId, OpId),
    Smlatb(OpId, OpId, OpId),
    Smlatt(OpId, OpId, OpId),
    Smlad(OpId, OpId, OpId),
    Smladx(OpId, OpId, OpId),
    Smlawb(OpId, OpId, OpId),
    Smlawt(OpId, OpId, OpId),
    Smlsd(OpId, OpId, OpId),
    Smlsdx(OpId, OpId, OpId),
    Smmla(OpId, OpId, OpId),
    Smmlar(OpId, OpId, OpId),
    Smmls(OpId, OpId, OpId),
    Smmlsr(OpId, OpId, OpId),
    Smmul(OpId, OpId),
    Smmulr(OpId, OpId),
    Smuad(OpId, OpId),
    Smuadx(OpId, OpId),
    Smulbb(OpId, OpId),
    Smulbt(OpId, OpId),
    Smultb(OpId, OpId),
    Smultt(OpId, OpId),
    Smulwb(OpId, OpId),
    Smulwt(OpId, OpId),
    Smusd(OpId, OpId),
    Smusdx(OpId, OpId),
}

impl Op {
    pub fn new<R: Rng>(decider: &mut Decider<R>, num_existing_ops: usize) -> Self {
        Self(match decider.decide_range(0..=3) {
            0 => match decider.decide_range(0..=2) {
                0 => Case::LdrConstant(decider.decide_range(0..=u32::MAX)),

                1 => Case::MovImmediate(if decider.decide() {
                    decider.decide_range(0..=65535)
                } else {
                    decide_modified_thumb_immediate(decider)
                }),

                2 => Case::MvnImmediate(decide_modified_thumb_immediate(decider)),

                _ => unreachable!(),
            },

            1 => {
                let rn = OpId(decider.decide_range(0..=(num_existing_ops - 1)));
                match decider.decide_range(0..=7) {
                    0 => Case::AddImmediate(
                        rn,
                        if decider.decide() {
                            decider.decide_range(0..=4095)
                        } else {
                            decide_modified_thumb_immediate(decider)
                        },
                    ),

                    1 => Case::AndImmediate(rn, decide_modified_thumb_immediate(decider)),

                    2 => Case::AsrImmediate(rn, decider.decide_range(1..=31)),

                    3 => Case::Bfc(rn, {
                        let lsb = decider.decide_range(0..=31);
                        let width = decider.decide_range(1..=(32 - lsb));
                        lsb..(lsb + width)
                    }),

                    4 => Case::BicImmediate(rn, decide_modified_thumb_immediate(decider)),

                    5 => Case::Clz(rn),

                    6 => Case::EorImmediate(rn, decide_modified_thumb_immediate(decider)),

                    7 => Case::LslImmediate(rn, decider.decide_range(1..=31)),

                    _ => unreachable!(),
                }
            }

            2 => {
                let rn = OpId(decider.decide_range(0..=(num_existing_ops - 1)));
                let rm = OpId(decider.decide_range(0..=(num_existing_ops - 1)));
                match decider.decide_range(0..=6) {
                    0 => Case::AddRegister(rn, rm, ImmediateShift::new(decider)),

                    1 => Case::AndRegister(rn, rm, ImmediateShift::new(decider)),

                    2 => Case::AsrRegister(rn, rm),

                    3 => Case::Bfi(rn, rm, {
                        let lsb = decider.decide_range(0..=31);
                        let width = decider.decide_range(1..=(32 - lsb));
                        lsb..(lsb + width)
                    }),

                    4 => Case::BicRegister(rn, rm, ImmediateShift::new(decider)),

                    5 => Case::EorRegister(rn, rm, ImmediateShift::new(decider)),

                    6 => Case::LslRegister(rn, rm),

                    _ => unreachable!(),
                }
            }

            3 => {
                let rn = OpId(decider.decide_range(0..=(num_existing_ops - 1)));
                let rm = OpId(decider.decide_range(0..=(num_existing_ops - 1)));
                let ra = OpId(decider.decide_range(0..=(num_existing_ops - 1)));
                match decider.decide_range(0..=0) {
                    0 => Case::Mla(rn, rm, ra),
                    _ => unreachable!(),
                }
            }

            _ => unreachable!(),
        })
    }

    pub fn evaluate(&self, dst: &mut [u32], srcs: &[impl Deref<Target = [u32]>]) {
        for src in srcs {
            assert_eq!(dst.len(), src.len())
        }

        match self.0 {
            Case::AddImmediate(OpId(rn), imm) => {
                let rn = &srcs[rn];
                for i in 0..dst.len() {
                    unsafe { *dst.get_unchecked_mut(i) = rn.get_unchecked(i).wrapping_add(imm) }
                }
            }

            Case::AddRegister(OpId(rn), OpId(rm), shift) => {
                let rn = &srcs[rn];
                let rm = &srcs[rm];
                shift.apply(dst, rm);
                for i in 0..dst.len() {
                    unsafe {
                        let dst = dst.get_unchecked_mut(i);
                        *dst = dst.wrapping_add(*rn.get_unchecked(i));
                    }
                }
            }

            Case::AndImmediate(OpId(rn), imm) => {
                let rn = &srcs[rn];
                for i in 0..dst.len() {
                    unsafe { *dst.get_unchecked_mut(i) = rn.get_unchecked(i) & imm }
                }
            }

            Case::AndRegister(OpId(rn), OpId(rm), shift) => {
                let rn = &srcs[rn];
                let rm = &srcs[rm];
                shift.apply(dst, rm);
                for i in 0..dst.len() {
                    unsafe {
                        *dst.get_unchecked_mut(i) &= rn.get_unchecked(i);
                    }
                }
            }

            Case::AsrImmediate(OpId(rn), imm) => {
                let rn = &srcs[rn];
                for i in 0..dst.len() {
                    unsafe {
                        *dst.get_unchecked_mut(i) =
                            (*rn.get_unchecked(i) as i32).unchecked_shr(imm.into()) as u32
                    }
                }
            }

            Case::AsrRegister(OpId(rn), OpId(rm)) => {
                let rn = &srcs[rn];
                let rm = &srcs[rm];
                for i in 0..dst.len() {
                    unsafe {
                        let shift = (*rm.get_unchecked(i) as i32) & 0xff;
                        *dst.get_unchecked_mut(i) = if shift >= 32 {
                            0
                        } else {
                            (*rn.get_unchecked(i) as i32).unchecked_shr(shift) as u32
                        }
                    }
                }
            }

            Case::Bfc(OpId(rn), ref range) => {
                let rn = &srcs[rn];
                let mask = !((1u32.wrapping_shl((range.end - range.start) as u32)).wrapping_sub(1)
                    << range.start);
                for i in 0..dst.len() {
                    unsafe { *dst.get_unchecked_mut(i) = rn.get_unchecked(i) & mask }
                }
            }

            Case::Bfi(OpId(rd), OpId(rn), ref range) => {
                let rd = &srcs[rd];
                let rn = &srcs[rn];
                let src_mask =
                    (1u32.wrapping_shl((range.end - range.start) as u32)).wrapping_sub(1);
                let dst_mask = !(src_mask << range.start);
                for i in 0..dst.len() {
                    unsafe {
                        *dst.get_unchecked_mut(i) = (rd.get_unchecked(i) & dst_mask)
                            | (rn.get_unchecked(i) & src_mask).unchecked_shl(range.start.into())
                    }
                }
            }

            Case::BicImmediate(OpId(rn), imm) => {
                let rn = &srcs[rn];
                for i in 0..dst.len() {
                    unsafe { *dst.get_unchecked_mut(i) = rn.get_unchecked(i) & !imm }
                }
            }

            Case::BicRegister(OpId(rn), OpId(rm), shift) => {
                let rn = &srcs[rn];
                let rm = &srcs[rm];
                shift.apply(dst, rm);
                for i in 0..dst.len() {
                    unsafe {
                        let dst = dst.get_unchecked_mut(i);
                        *dst = rn.get_unchecked(i) & !*dst;
                    }
                }
            }

            Case::Clz(OpId(rn)) => {
                let rn = &srcs[rn];
                for i in 0..dst.len() {
                    unsafe { *dst.get_unchecked_mut(i) = rn.get_unchecked(i).leading_zeros() }
                }
            }

            Case::EorImmediate(OpId(rn), imm) => {
                let rn = &srcs[rn];
                for i in 0..dst.len() {
                    unsafe { *dst.get_unchecked_mut(i) = rn.get_unchecked(i) ^ imm };
                }
            }

            Case::EorRegister(OpId(rn), OpId(rm), shift) => {
                let rn = &srcs[rn];
                let rm = &srcs[rm];
                shift.apply(dst, rm);
                for i in 0..dst.len() {
                    unsafe { *dst.get_unchecked_mut(i) ^= rn.get_unchecked(i) }
                }
            }

            Case::LdrConstant(value) => dst.fill(value),

            Case::LslImmediate(OpId(rn), amt) => {
                let rn = &srcs[rn];
                for i in 0..dst.len() {
                    unsafe {
                        *dst.get_unchecked_mut(i) = rn.get_unchecked(i).unchecked_shl(amt.into())
                    }
                }
            }

            Case::LslRegister(OpId(rn), OpId(rm)) => {
                let rn = &srcs[rn];
                let rm = &srcs[rm];
                for i in 0..dst.len() {
                    unsafe {
                        let shift = *rm.get_unchecked(i) & 0xff;
                        *dst.get_unchecked_mut(i) = if shift >= 32 {
                            0
                        } else {
                            rn.get_unchecked(i).unchecked_shl(shift)
                        }
                    }
                }
            }

            Case::Mla(OpId(rn), OpId(rm), OpId(ra)) => {
                let rn = &srcs[rn];
                let rm = &srcs[rm];
                let ra = &srcs[ra];
                for i in 0..dst.len() {
                    unsafe {
                        *dst.get_unchecked_mut(i) = rn
                            .get_unchecked(i)
                            .wrapping_mul(*rm.get_unchecked(i))
                            .wrapping_add(*ra.get_unchecked(i))
                    }
                }
            }

            Case::MovImmediate(imm) => dst.fill(imm),

            Case::MvnImmediate(imm) => dst.fill(!imm),

            _ => todo!(),
        }
    }
}

fn decide_modified_thumb_immediate<R: Rng>(decider: &mut Decider<R>) -> u32 {
    let abcdefgh = decider.decide_range(0..=255);
    match decider.decide_range(0..=4) {
        0 => abcdefgh,
        1 => (abcdefgh << 16) | abcdefgh,
        2 => (abcdefgh << 24) | (abcdefgh << 8),
        3 => (abcdefgh << 24) | (abcdefgh << 16) | (abcdefgh << 8) | abcdefgh,
        4 => (abcdefgh | 0x80) << decider.decide_range(1..=24),
        _ => unreachable!(),
    }
}
