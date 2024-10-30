use crate::{UVec2, Vec2};
pub mod neighbors;

pub struct BinImage {
    data: Vec<u8>,
    height: u32,
    width: u32,
}

impl BinImage {
    /// Creates a new `BinImage` from the given height, width, and raw pixel data.
    ///
    /// # Arguments
    ///
    /// * `height` - The height of the image in pixels.
    /// * `width` - The width of the image in pixels.
    /// * `data` - A slice of bytes representing the raw pixel data. The length of this slice
    ///   must be at least `height * width`.
    ///
    /// # Panics
    ///
    /// This function will panic if the length of `data` is less than `height * width`.
    pub fn new(height: u32, width: u32, data: &[u8]) -> Self {
        assert!(
            data.len() >= (height * width) as usize,
            "data must not be smaller than image dimensions"
        );
        let compress_step = data.len() / (height * width) as usize;
        Self {
            data: data
                .chunks(8 * compress_step)
                .map(|chunk| {
                    chunk
                        .chunks(compress_step)
                        .map(|chunk| chunk.iter().any(|i| *i != 0))
                        .enumerate()
                        .map(|(index, bit)| u8::from(bit) << index)
                        .sum()
                })
                .collect(),
            height,
            width,
        }
    }

    /// Gets the pixel value at the given coordinate.
    ///
    /// # Arguments
    ///
    /// * `p` - A `UVec2` representing the coordinates of the pixel.
    ///
    /// # Returns
    ///
    /// Returns `true` if the pixel is "on" (1), and `false` if it is "off" (0) or out of bounds.
    pub fn get(&self, p: UVec2) -> bool {
        let (x, y) = (p.x, p.y);
        let index = y * self.width + x;
        if let Some(mut byte) = self
            .data
            .get((index / 8) as usize) // index of byte
            .copied()
        {
            byte >>= index % 8; // index of bit
            x <= self.width && byte & 1 > 0
        } else {
            false
        }
    }

    /// Gets the values of the neighboring pixels (8-connectivity) around the given coordinate.
    ///
    /// # Arguments
    ///
    /// * `p` - A `UVec2` representing the coordinates of the center pixel.
    ///
    /// # Returns
    ///
    /// An byte representing the state of the neighboring pixels.
    pub fn get_neighbors(&self, p: UVec2) -> u8 {
        let (x, y) = (p.x, p.y);
        let mut neighbors = 0;
        if y < u32::MAX && self.get(UVec2::new(x, y + 1)) {
            neighbors |= neighbors::NORTH;
        }
        if y > u32::MIN && self.get(UVec2::new(x, y - 1)) {
            neighbors |= neighbors::SOUTH;
        }
        if x < u32::MAX && self.get(UVec2::new(x + 1, y)) {
            neighbors |= neighbors::EAST;
        }
        if x > u32::MIN && self.get(UVec2::new(x - 1, y)) {
            neighbors |= neighbors::WEST;
        }
        if x < u32::MAX && y < u32::MAX && self.get(UVec2::new(x + 1, y + 1)) {
            neighbors |= neighbors::NORTHEAST;
        }
        if x > u32::MIN && y > u32::MIN && self.get(UVec2::new(x - 1, y - 1)) {
            neighbors |= neighbors::NORTHWEST;
        }
        if x < u32::MAX && y > u32::MIN && self.get(UVec2::new(x + 1, y - 1)) {
            neighbors |= neighbors::SOUTHEAST;
        }
        if x > u32::MIN && y < u32::MAX && self.get(UVec2::new(x - 1, y + 1)) {
            neighbors |= neighbors::SOUTHWEST;
        }
        neighbors
    }

    /// Translates a point in positive (x, y) coordinates to a coordinate system centered at (0, 0).
    ///
    /// # Arguments
    ///
    /// * `p` - A `Vec2` representing the point to translate.
    ///
    /// # Returns
    ///
    /// A new `Vec2` representing the translated coordinates
    fn translate_point(&self, p: Vec2) -> Vec2 {
        Vec2::new(
            p.x - ((self.width / 2) as f32 - 1.0),
            ((self.height / 2) as f32 - 1.0) - p.y,
        )
    }

    /// Translates an `Vec` of points in positive (x, y) coordinates to a coordinate system centered at (0, 0).
    ///
    /// # Arguments
    ///
    /// * `v` - An `Vec` of `Vec2` points to translate.
    ///
    /// # Returns
    ///
    /// A vector of `Vec2` representing the translated coordinates.
    pub fn translate(&self, v: Vec<Vec2>) -> Vec<Vec2> {
        v.into_iter().map(|p| self.translate_point(p)).collect()
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub const fn width(&self) -> u32 {
        self.width
    }
}