use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImmediateShift(Case);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Case {
    None,
    Lsl(u8),
    Lsr(u8),
    Asr(u8),
    Ror(u8),
}

impl Default for ImmediateShift {
    fn default() -> Self {
        Self(Case::None)
    }
}

impl Distribution<ImmediateShift> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ImmediateShift {
        ImmediateShift(match rng.gen_range(0..5) {
            0 => Case::None,
            1 => Case::Lsl(rng.gen_range(0..31)),
            2 => Case::Lsr(rng.gen_range(0..31)),
            3 => Case::Asr(rng.gen_range(0..31)),
            4 => Case::Ror(rng.gen_range(0..31)),
            _ => unreachable!(),
        })
    }
}

impl ImmediateShift {
    pub fn apply(self, dst: &mut [u32], src: &[u32]) {
        assert_eq!(src.len(), dst.len());
        match self.0 {
            Case::None => dst.copy_from_slice(src),

            Case::Lsl(amount) => {
                for i in 0..src.len() {
                    unsafe {
                        *dst.get_unchecked_mut(i) =
                            src.get_unchecked(i).unchecked_shl(amount.into())
                    }
                }
            }

            Case::Lsr(amount) => {
                for i in 0..src.len() {
                    unsafe {
                        *dst.get_unchecked_mut(i) =
                            src.get_unchecked(i).unchecked_shr(amount.into())
                    }
                }
            }

            Case::Asr(amount) => {
                for i in 0..src.len() {
                    unsafe {
                        *dst.get_unchecked_mut(i) =
                            (*src.get_unchecked(i) as i32).unchecked_shr(amount.into()) as u32
                    }
                }
            }

            Case::Ror(amount) => {
                for i in 0..src.len() {
                    unsafe {
                        *dst.get_unchecked_mut(i) = src.get_unchecked(i).rotate_right(amount.into())
                    }
                }
            }
        }
    }
}
