use std::mem::{transmute, MaybeUninit};

use bitflags::bitflags;
use cubiomes_sys::enums::StructureType;
use thiserror::Error;

use super::{
    position::{MinecraftPosition, StructureRegionPosition},
    Generator,
};

#[derive(Error, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum StructureGenerationError {
    #[error("Underlying library cubiomes returned a bool that is not 0 or 1.")]
    CubiomesError,
}

// This is empty, since I dont know what flags cubiomes supports
// I could not find them in the documentation
bitflags! {struct StructureFlags: u32{}}

impl Generator {
    pub fn try_generate_structure_in_region(
        &self,
        region_pos: StructureRegionPosition,
    ) -> Option<MinecraftPosition> {
        let pos = self.get_structure_generation_attempt(region_pos)?;

        if self
            .verify_structure_generation_attempt(
                pos,
                region_pos.structure_type,
                StructureFlags::empty(),
            )
            .ok()?
        {
            return Some(pos);
        }

        None
    }

    pub fn get_structure_generation_attempt(
        &self,
        region_pos: StructureRegionPosition,
    ) -> Option<MinecraftPosition> {
        let minecraft_version: cubiomes_sys::enums::MCVersion = self.minecraft_version();
        let seed = self.seed();

        let mut pos: MaybeUninit<cubiomes_sys::Pos> = MaybeUninit::uninit();

        // SAFETY:
        // The ffi function receives correct input data
        //
        // The seed is transmuted as cubiomes wants it as u64
        // even though minecraft uses signed integers
        if unsafe {
            cubiomes_sys::getStructurePos(
                region_pos.structure_type as i32,
                minecraft_version as i32,
                transmute::<i64, u64>(seed),
                region_pos.x,
                region_pos.z,
                pos.as_mut_ptr(),
            )
        } == 0
        {
            return None;
        }

        // SAFETY:
        // as getStructurePos did not return 0 pos should be
        // initialized by cubiomes
        Some(unsafe { pos.assume_init() }.into())
    }

    fn verify_structure_generation_attempt(
        &self,
        pos: MinecraftPosition,
        structure_type: StructureType,
        flags: StructureFlags,
    ) -> Result<bool, StructureGenerationError> {
        match unsafe {
            cubiomes_sys::isViableStructurePos(
                structure_type as i32,
                self.generator,
                pos.x,
                pos.z,
                flags.bits(),
            )
        } {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(StructureGenerationError::CubiomesError),
        }
    }

    pub fn try_get_structure_in_region(&self, pos: MinecraftPosition) -> MinecraftPosition {
        todo!()
    }
}
