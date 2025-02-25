use super::error::{GeneratorError, TryFromRangeError};

/// A scale for the [Range]
///
/// The scale reperesents valid options for giving to the generator
/// as a scale.
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Scale {
    /// A scale of 1:1, Block scale
    Block = 1,
    /// A scale of 1:4
    Quad = 4,
    /// A scale of 1:16, the size of a minecraft chunk
    Chunk = 16,
    /// A scale of 1:64, the size of 4 minecraft chunks
    QuadChunk = 64,
    /// A scale of 1:256, half of a minecraft region
    HalfRegion = 256,
}

impl Scale {
    /// Scales a block coordinate according to this scale
    ///
    /// Divides the input number by the scale
    #[must_use]
    pub const fn scale_coord(&self, num: i32) -> i32 {
        num / *self as i32
    }

    /// Reverses scaling done with this scale
    ///
    /// Turns whatever number is at this scale back to block coordinates
    ///
    /// Multiplies the input number with this scale
    #[must_use]
    pub const fn unscale_coord(&self, num: i32) -> i32 {
        num * *self as i32
    }
}

/// Size and position for a [`super::Cache`]
///
/// The range represents a location and size of a cache.
///
/// The position and size of the range is scaled by its [`Range::scale`] attribute.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct Range {
    /// Scale used for the coordinates
    pub scale: Scale,
    /// Position of the top left corner of the range, scaled by the [Scale]
    ///
    /// to map this into minecraft coordinates the x is multiplied by the scale
    pub x: i32,
    /// Position of the top left corner of the range, scaled by the [Scale]
    ///
    /// to map this into minecraft coordinates the x is multiplied by the scale
    pub z: i32,
    /// Size of the range in x coordinates, scaled by the [Scale]
    ///
    pub size_x: u32,
    /// Size of the range in z coordinates, scaled by the [Scale]
    pub size_z: u32,
    /// The y coordinate of the range scaled either 1:1 or 1:4 depending on [Scale]
    ///
    /// Scale is 1:1 for [`Scale::Block`] other scales get a 1:4 mapping of the
    /// y coordinate
    pub y: i32,
    /// Veritcal size of the cube. 0 and 1 mean a 2d plane
    pub size_y: u32,
}

impl TryFrom<Range> for cubiomes_sys::Range {
    type Error = GeneratorError;

    fn try_from(value: Range) -> Result<Self, Self::Error> {
        Ok(Self {
            scale: value.scale as i32,
            x: value.x,
            z: value.z,
            sx: value
                .size_x
                .try_into()
                .map_err(|_| TryFromRangeError::XSizeOutOfBounds)
                .and_then(|sx| err_if_zero(sx, TryFromRangeError::XSizeOutOfBounds))?,
            sz: value
                .size_z
                .try_into()
                .map_err(|_| TryFromRangeError::ZSizeOutOfBounds)
                .and_then(|sz| err_if_zero(sz, TryFromRangeError::YSizeOutOfBouns))?,

            y: value.y,
            sy: value
                .size_y
                .try_into()
                .map_err(|_| TryFromRangeError::YSizeOutOfBouns)?,
        })
    }
}

impl Range {
    /// Checks if a given minecraft coordinate is within this range.
    ///
    /// First scales and then checks if a given coordinate is within this range.
    #[must_use]
    pub fn is_inside(&self, x: i32, z: i32) -> bool {
        ((self.x <= self.scale.scale_coord(x))
            && (self.scale.scale_coord(x) < (self.x + self.size_x as i32)))
            && ((self.z <= self.scale.scale_coord(z))
                && (self.scale.scale_coord(z) < (self.z + self.size_z as i32)))
    }

    /// Tries to calculate a coordinate relative to this range.
    ///
    /// Tries to turn a global minecraft coordinate, to one inside this cache.
    ///
    /// Returns none if the coordinate is outside this cache
    ///
    #[must_use]
    pub fn global_to_local_coord(&self, x: i32, z: i32) -> Option<(u32, u32)> {
        if self.is_inside(x, z) {
            Some((
                (self.scale.scale_coord(x) - self.x)
                    .try_into()
                    .unwrap_or_else(|_| unreachable!()),
                (self.scale.scale_coord(z) - self.z)
                    .try_into()
                    .unwrap_or_else(|_| unreachable!()),
            ))
        } else {
            None
        }
    }
}

fn err_if_zero(num: i32, err: TryFromRangeError) -> Result<i32, TryFromRangeError> {
    if num == 0 {
        return Err(err);
    }
    Ok(num)
}
