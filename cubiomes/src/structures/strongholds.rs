//! Module containing [StrongholdIter], an iterator which generates all the
//! strongholds in a [Generator]

use crate::generator::{BlockPosition, Generator};
use std::mem::{transmute, MaybeUninit};

/// An iterator over the strongholds in a [Generator]
///
/// As the strongholds in minecraft are generated iteratively, we use an
/// iterator for generating them.
///
/// The iterator produces the [BlockPosition] of the next stronghold until all
/// strongholds are generated.
///
/// # Examples
/// Generate and collect all the strongholds in a seed.
/// ```
#[doc = include_str!("../../examples/generate_strongholds.rs")]
/// ```
#[derive(Debug)]
pub struct StrongholdIter<'generator> {
    generator: &'generator Generator,
    inner: cubiomes_sys::StrongholdIter,
    strongholds_left: usize,
}

impl<'generator> Generator {
    /// Constructs an iterator over the strongholds in this generator
    ///
    /// Constructs a new [StrongholdIter] from [self]. See [StrongholdIter] for
    /// usage
    #[must_use]
    pub fn strongholds(&'generator self) -> StrongholdIter<'generator> {
        let mut sh_iter: MaybeUninit<cubiomes_sys::StrongholdIter> = MaybeUninit::uninit();

        // SAFETY: ffi function is called correctly
        unsafe {
            cubiomes_sys::initFirstStronghold(
                sh_iter.as_mut_ptr(),
                self.minecraft_version() as i32,
                transmute::<i64, u64>(self.seed()),
            );
        }

        // We subtract one since cubiomes strongholds left is weird like that
        let strongholds_left =
            // SAFETY: ffi function is called correctly
            unsafe { cubiomes_sys::nextStronghold(sh_iter.as_mut_ptr(), self.as_ptr()) } as usize
                - 1;

        StrongholdIter {
            generator: self,
            // SAFETY: sh_iter was initialized by ffi
            inner: unsafe { sh_iter.assume_init() },
            strongholds_left,
        }
    }
}

impl Iterator for StrongholdIter<'_> {
    type Item = BlockPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if 0 == self.strongholds_left {
            return None;
        }
        // We subtract one since cubiomes strongholds left is weird like that
        self.strongholds_left =
            // SAFETY: ffi function is called correctly, and as we checked 
            // strongholds_left we aren't iterating beyond its borders
            unsafe { cubiomes_sys::nextStronghold(&mut self.inner, self.generator.as_ptr()) }
                as usize
                - 1;

        Some(self.inner.pos.into())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.strongholds_left, Some(self.strongholds_left))
    }
}
