use std::mem::MaybeUninit;

use cubiomes_sys::{enums, Pos};

use super::{structures::StructureGenerationError, Scale};

///A 2d position inside minecraft
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MinecraftPosition {
    pub x: i32,
    pub z: i32,
}

impl MinecraftPosition {
    /// Creates a new instance of a minecraft position at block scale
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    /// Creates a new instance of minecraft position from coordinates at the
    /// specific [Scale]
    pub fn from_scaled(x: i32, z: i32, scale: Scale) -> Self {
        Self {
            x: scale.unscale_coord(x),
            z: scale.unscale_coord(z),
        }
    }

    /// Scales this minecraft position to a [Scale]
    pub fn as_scaled(&self, scale: Scale) -> (i32, i32) {
        (scale.scale_coord(self.x), scale.scale_coord(self.z))
    }
}

impl From<Pos> for MinecraftPosition {
    fn from(value: Pos) -> Self {
        Self {
            x: value.x,
            z: value.z,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct StructureRegionPosition {
    pub x: i32,
    pub z: i32,
    pub(crate) minecraft_version: enums::MCVersion,
    pub(crate) structure_type: enums::StructureType,
}

impl StructureRegionPosition {
    pub fn new(
        pos: MinecraftPosition,
        minecraft_version: enums::MCVersion,
        structure_type: enums::StructureType,
    ) -> Result<Self, StructureGenerationError> {
        let scale = get_structure_scale(pos, structure_type, minecraft_version);

        Ok(Self {
            x: pos.x / scale? as i32,
            z: pos.z / scale? as i32,
            minecraft_version,
            structure_type,
        })
    }

    pub fn set_new_minecraft_pos(&self, pos: MinecraftPosition) {
        todo!()
    }

    fn region_size(&self) -> i8 {
        todo!()
    }
}

fn get_structure_scale(
    pos: MinecraftPosition,
    structure_type: enums::StructureType,
    minecraft_version: enums::MCVersion,
) -> Result<i8, StructureGenerationError> {
    unsafe {
        let mut sconf: MaybeUninit<cubiomes_sys::StructureConfig> = MaybeUninit::uninit();

        match cubiomes_sys::getStructureConfig(
            structure_type as i32,
            minecraft_version as i32,
            sconf.as_mut_ptr(),
        ) {
            0 => Err(StructureGenerationError::CubiomesError),
            _ => Ok(sconf.assume_init().regionSize as i8),
        }
    }
}
