use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

use rand::{distributions::Standard, prelude::Distribution};

#[repr(align(32))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Batch<T, const N: usize>([T; N]);

impl<T, const N: usize> Distribution<Batch<T, N>> for Standard
where
    Standard: Distribution<[T; N]>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Batch<T, N> {
        Batch(rng.gen())
    }
}

impl<T, const N: usize> Deref for Batch<T, N> {
    type Target = [T; N];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Batch<T, N> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize> AsRef<[T; N]> for Batch<T, N> {
    #[inline(always)]
    fn as_ref(&self) -> &[T; N] {
        &self.0
    }
}

impl<T, const N: usize> AsMut<[T; N]> for Batch<T, N> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [T; N] {
        &mut self.0
    }
}

impl<T, const N: usize> Borrow<[T; N]> for Batch<T, N> {
    #[inline(always)]
    fn borrow(&self) -> &[T; N] {
        &self.0
    }
}

impl<T, const N: usize> BorrowMut<[T; N]> for Batch<T, N> {
    #[inline(always)]
    fn borrow_mut(&mut self) -> &mut [T; N] {
        &mut self.0
    }
}

impl<T, const N: usize> From<[T; N]> for Batch<T, N> {
    #[inline(always)]
    fn from(array: [T; N]) -> Self {
        Self(array)
    }
}

impl<T, const N: usize> From<Batch<T, N>> for [T; N] {
    #[inline(always)]
    fn from(batch: Batch<T, N>) -> Self {
        batch.0
    }
}
