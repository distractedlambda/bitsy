use std::ops::{Deref, Range};

use rand::Rng;

use crate::immediate_shift::ImmediateShift;

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
    pub fn random(rng: &mut impl Rng, num_existing_ops: usize) -> Self {
        Self(match rng.gen_range(0..=3) {
            0 => match rng.gen_range(0..=2) {
                0 => Case::LdrConstant(rng.gen()),

                1 => Case::MovImmediate(if rng.gen() {
                    rng.gen_range(0..=65535)
                } else {
                    random_modified_thumb_immediate(rng)
                }),

                2 => Case::MvnImmediate(random_modified_thumb_immediate(rng)),

                _ => unreachable!(),
            },

            1 => {
                let rn = OpId(rng.gen_range(0..num_existing_ops));
                match rng.gen_range(0..=5) {
                    0 => Case::AddImmediate(
                        rn,
                        if rng.gen() {
                            rng.gen_range(0..=4095)
                        } else {
                            random_modified_thumb_immediate(rng)
                        },
                    ),

                    1 => Case::AndImmediate(rn, random_modified_thumb_immediate(rng)),

                    2 => Case::AsrImmediate(rn, rng.gen_range(1..=31)),

                    3 => Case::Bfc(rn, {
                        let lsb = rng.gen_range(0..=31);
                        let width = rng.gen_range(1..=(32 - lsb));
                        lsb..(lsb + width)
                    }),

                    4 => Case::BicImmediate(rn, random_modified_thumb_immediate(rng)),

                    5 => Case::Clz(rn),

                    _ => unreachable!(),
                }
            }

            2 => {
                let rn = OpId(rng.gen_range(0..num_existing_ops));
                let rm = OpId(rng.gen_range(0..num_existing_ops));
                match rng.gen_range(0..=4) {
                    0 => Case::AddRegister(rn, rm, rng.gen()),

                    1 => Case::AndRegister(rn, rm, rng.gen()),

                    2 => Case::AsrRegister(rn, rm),

                    3 => Case::Bfi(rn, rm, {
                        let lsb = rng.gen_range(0..=31);
                        let width = rng.gen_range(1..=(32 - lsb));
                        lsb..(lsb + width)
                    }),

                    4 => Case::BicRegister(rn, rm, rng.gen()),

                    _ => unreachable!(),
                }
            }

            3 => {
                let rn = OpId(rng.gen_range(0..num_existing_ops));
                let rm = OpId(rng.gen_range(0..num_existing_ops));
                let ra = OpId(rng.gen_range(0..num_existing_ops));
                match rng.gen_range(0..=0) {
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

            Case::LdrConstant(value) => dst.fill(value),

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

fn random_modified_thumb_immediate(rng: &mut impl Rng) -> u32 {
    let abcdefgh = rng.gen_range(0..=255);
    match rng.gen_range(0..=4) {
        0 => abcdefgh,
        1 => (abcdefgh << 16) | abcdefgh,
        2 => (abcdefgh << 24) | (abcdefgh << 8),
        3 => (abcdefgh << 24) | (abcdefgh << 16) | (abcdefgh << 8) | abcdefgh,
        4 => (abcdefgh | 0x80) << rng.gen_range(1..=24),
        _ => unreachable!(),
    }
}
