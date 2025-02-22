//! Module containing structure generation and spawn generation
//!
//! Most structures in minecraft are generated using a grid of regions
//! for these types of structures, see [StructureRegion]
//!
//! Notably, stronghold generation differs, following an iterative
//! method instead. For generating the strongholdpositions, see
//! [StrongholdIterator]

use std::mem::{transmute, MaybeUninit};

use bitflags::bitflags;
use cubiomes_sys::enums::{self};
use thiserror::Error;

use enums::*;

use crate::generator::{BlockPosition, Generator};

/// Reperesents an error in cubiomes
#[derive(Error, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum StructureGenerationError {
    /// Cubiomes did not return 0 or 1. Encountering this error is most likely a bug
    /// please report it on github
    #[error("Underlying library cubiomes returned a bool that is not 0 or 1.")]
    CubiomesError,
}

// This is empty, since I dont know what flags cubiomes supports
// I could not find them in the documentation
bitflags! {struct StructureFlags: u32{}}

impl Generator {
    /// Tries to get the [BlockPosition] of a structure inside of a [StructureRegion]
    /// with this generator.
    ///
    /// # Panics
    /// The function panics if the version of the structure region does not match
    /// the generator
    pub fn try_generate_structure_in_region(
        &mut self,
        region_pos: StructureRegion,
    ) -> Option<BlockPosition> {
        assert_eq!(self.minecraft_version(), region_pos.minecraft_version);

        let pos = self.get_structure_generation_attempt(region_pos)?;

        if self
            .verify_structure_generation_attempt(pos, region_pos.structure_type)
            .ok()?
        {
            return Some(pos);
        }

        None
    }

    /// Used to verify a structure generation attempt
    ///
    /// See [StructureRegion] for an explanation for what a structure generation
    /// attempt means
    pub fn verify_structure_generation_attempt(
        &mut self,
        pos: BlockPosition,
        structure_type: StructureType,
    ) -> Result<bool, StructureGenerationError> {
        // SAFETY: The foreign function is being called properly
        match unsafe {
            cubiomes_sys::isViableStructurePos(
                structure_type as i32,
                self.as_mut_ptr(),
                pos.x,
                pos.z,
                StructureFlags::empty().bits(),
            )
        } {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(StructureGenerationError::CubiomesError),
        }
    }

    fn get_structure_generation_attempt(
        &self,
        region_pos: StructureRegion,
    ) -> Option<BlockPosition> {
        let seed = self.seed();
        region_pos.get_structure_generation_attempt(seed)
    }
}

/// Represents a region for generating a specific structure in a specific version of minecraft
///
/// Minecraft structure generation works by splitting the world into
/// regions, the size of which is specified in the structures configuration.
/// Then in each of thoes regions it will perform one attempt to generate
/// the structure. Then if the biome at the generation attempt is correct
/// the structure is generated
///
/// # Usage
///
/// The module should be used by first finding one of thoes attempts with
/// [Self::get_structure_generation_attempt()] then verifying it by using
/// [Generator::verify_structure_generation_attempt()]. The generator should
/// be initialized for the same seed and region used to generate the
/// generation attempt.
///
/// Alternatively you can just use [Generator::try_generate_structure_in_region()]
/// which will perform all of this automatically.
///
/// # Examples
/// ## Finding structures within a seed
/// ```
#[doc = include_str!("../../examples/find_structures.rs")]
/// ```
///
/// ## Finding a seed with a specific structure at spawn
///
/// It should be noted, that only the lower 48 bits of the seed affect
/// the positions of structure generation attempts. Generating the position of a
/// structure generation attempt is also cheaper than verifying the biome for a structure.
///
/// So if you for example, want to find a seed with a specific set of structures
/// near spawn, you should try to find it by modifying the 48 bottom bits and that an
/// attempt exist in your wanted region. Once you've found the attempts, you can modify
/// the top 16 bits until the biomes match. This example demonstrates how to achieve this.
///
/// ```
#[doc = include_str!("../../examples/efficient_structure_hunting.rs")]
/// ```
///
/// # Details
///
/// The size of each region can be acquired with [Self::region_size_blocks()]
/// or [Self::region_size_chunks()] respectively.
///
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct StructureRegion {
    /// The x position of [self].
    ///
    /// The scale can be acquired with [Self::region_size_blocks()] or [Self::region_size_chunks()]
    pub x: i32,
    /// The z position of [self]
    ///
    /// The scale can be acquired with [Self::region_size_blocks()] or [Self::region_size_chunks()]
    pub z: i32,
    region_size: i8,
    pub(crate) minecraft_version: enums::MCVersion,
    pub(crate) structure_type: enums::StructureType,
}

impl StructureRegion {
    /// Creates a new [StructureRegion]
    ///
    /// This function creates a new structure region position at the given
    /// region x and z value with the given [StructureType] and [enums::MCVersion]
    pub fn new(
        region_x: i32,
        region_z: i32,
        minecraft_version: enums::MCVersion,
        structure_type: enums::StructureType,
    ) -> Result<Self, StructureGenerationError> {
        let region_scale = get_structure_scale(structure_type, minecraft_version)?;

        Ok(Self {
            x: region_x,
            z: region_z,
            region_size: region_scale,
            minecraft_version,
            structure_type,
        })
    }

    /// Creates a new [StructureRegion] with a [BlockPosition]
    ///
    /// The block position is automatically converted into the correct scale
    /// for the specific [StructureType]
    pub fn from_block_position(
        pos: BlockPosition,
        minecraft_version: enums::MCVersion,
        structure_type: enums::StructureType,
    ) -> Result<Self, StructureGenerationError> {
        let region_scale = get_structure_scale(structure_type, minecraft_version)?;

        // Multiply the scale by 16 since structure positions are in chunk size for some reason
        let (x, z) = pos.scale_by_num((region_scale as i32) * 16);

        Ok(Self {
            x,
            z,
            region_size: region_scale,
            minecraft_version,
            structure_type,
        })
    }

    /// Tries to get the [BlockPosition] of a generation attempt for self
    ///
    /// Check [self] for what a generation attempt means
    pub fn get_structure_generation_attempt(&self, seed: i64) -> Option<BlockPosition> {
        let minecraft_version = self.minecraft_version;

        let mut pos: MaybeUninit<cubiomes_sys::Pos> = MaybeUninit::uninit();

        // SAFETY:
        // The ffi function receives correct input data
        //
        // The seed is transmuted as cubiomes wants it as u64
        // even though minecraft uses signed integers
        if unsafe {
            cubiomes_sys::getStructurePos(
                self.structure_type as i32,
                minecraft_version as i32,
                transmute::<i64, u64>(seed),
                self.x,
                self.z,
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

    /// Moves [self] to the region of the given [BlockPosition]
    pub fn set_new_minecraft_pos(&mut self, pos: BlockPosition) {
        (self.x, self.z) = pos.scale_by_num(self.region_size as i32);
    }

    /// Gets the region sife of [self] in chunks
    pub fn region_size_chunks(&self) -> i32 {
        self.region_size as i32
    }

    /// Gets the region size of [self] in blocks
    pub fn region_size_blocks(&self) -> i32 {
        (self.region_size as i32) * 16
    }

    /// Gets the minecraft version of [self]
    pub fn minecraft_verions(&self) -> enums::MCVersion {
        self.minecraft_version
    }

    /// Gets the structure type of [self]
    pub fn structure_type(&self) -> enums::StructureType {
        self.structure_type
    }
}

fn get_structure_scale(
    structure_type: enums::StructureType,
    minecraft_version: enums::MCVersion,
) -> Result<i8, StructureGenerationError> {
    // SAFETY: sconf is initialized if GetStructureConfig did not return 0
    unsafe {
        let mut sconf: MaybeUninit<cubiomes_sys::StructureConfig> = MaybeUninit::uninit();

        match cubiomes_sys::getStructureConfig(
            structure_type as i32,
            minecraft_version as i32,
            sconf.as_mut_ptr(),
        ) {
            0 => Err(StructureGenerationError::CubiomesError),
            _ => Ok(sconf.assume_init().regionSize),
        }
    }
}

pub struct StrongholdIterator;
