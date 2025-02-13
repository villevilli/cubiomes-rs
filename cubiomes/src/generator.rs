use std::mem::{self, transmute};

use bitflags::bitflags;
use cubiomes_sys::{Dimension, biome_enum, num_traits::FromPrimitive};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum GeneratorError {
    #[error("Biome id {0} is out of range and is not a valid biomeid")]
    BiomeIDOutOfRange(i32),
    #[error(
        "Function getBiomeAt failed (the ffi function returned -1), did you perhaps forgot to initialize the seed?"
    )]
    GetBiomeAtFailure,
}

bitflags! {
    pub struct Flags: u32 {
        const LargeBiomes = 0x1;
        const NoBetaOcean = 0x2;
        const ForceOceanVariants = 0x4;

        //the source may set any bits
        const _ = !0;
    }
}

pub enum Scale {
    Block = 1,
    Biome = 4,
}

pub struct Generator {
    generator: cubiomes_sys::Generator,
}

impl Generator {
    pub fn new(version: biome_enum::MCVersion, flags: Flags) -> Self {
        unsafe {
            let mut generator: cubiomes_sys::Generator = mem::zeroed();

            cubiomes_sys::setupGenerator(&mut generator, version as i32, flags.bits());
            Self {
                generator: generator,
            }
        }
    }

    pub fn apply_seed(&mut self, dimension: Dimension, seed: i64) {
        unsafe {
            cubiomes_sys::applySeed(&mut self.generator, dimension.0, transmute(seed));
        }
    }

    pub fn get_biome_at(
        &self,
        scale: Scale,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<biome_enum::BiomeID, GeneratorError> {
        unsafe {
            match cubiomes_sys::getBiomeAt(&self.generator, scale as i32, x, y, z) {
                -1 => Err(GeneratorError::GetBiomeAtFailure),
                n => FromPrimitive::from_i32(n).ok_or(GeneratorError::BiomeIDOutOfRange(n)),
            }
        }
    }
}
