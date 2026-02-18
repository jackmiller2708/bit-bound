/// A sprite backed by 2bpp planar tile data.
///
/// Dimensions are in pixels and must be multiples of 8 (tile-aligned).
/// Data is stored as sequential 8×8 tiles in row-major order,
/// with each tile occupying 16 bytes (GameBoy-style 2bpp planar).
pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub tiles_x: usize,
    pub tiles_y: usize,
    pub data: &'static [u8],
}
