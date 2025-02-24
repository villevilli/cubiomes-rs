use cubiomes_sys::Pos;

use crate::generator::Scale;

///A 2d position inside minecraft
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlockPosition {
    /// The x axis of the position
    pub x: i32,
    /// The z axis of the position
    pub z: i32,
}

impl BlockPosition {
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

    /// Scales the position down by a given number
    ///
    /// Internally divides both axis by scale
    pub fn scale_by_num(&self, scale: i32) -> (i32, i32) {
        (self.x.div_euclid(scale), self.z.div_euclid(scale))
    }
}

impl From<Pos> for BlockPosition {
    fn from(value: Pos) -> Self {
        Self {
            x: value.x,
            z: value.z,
        }
    }
}
