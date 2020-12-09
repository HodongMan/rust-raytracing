use super::math::Vec4;

pub struct Surface
{
    width: usize,
    height: usize,
    pixels: Vec<Vec4>,
}

impl Surface
{
    pub fn new(width: usize, height: usize) -> Self
    {
        debug_assert!(0 < width);
        debug_assert!(0 < height);

        let mut pixels: Vec<Vec4> = Vec::new();
        pixels.resize(width * height, Vec4::new(0.0, 0.0, 0.0, 1.0));

        Self::from(width, height, pixels)
    }

    pub fn from(width: usize, height: usize, pixels: Vec<Vec4>) -> Self
    {
        debug_assert_eq!(pixels.len(), width * height);

         Self
        {
            width,
            height,
            pixels
        }
    }

    pub fn get_width(&self) -> usize
    {
        self.width
    }

    pub fn get_height(&self) -> usize
    {
        self.height
    }

    pub fn get_pixel_count(&self) -> usize
    {
        self.width *  self.height
    }

    pub fn get_data_size(&self) -> usize
    {
        self.get_pixel_count() * std::mem::size_of::<Vec4>()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec4)
    {
        let index           = y * self.width + x;
        self.pixels[index]  = color
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Vec4
    {
        let index           = y * self.width + x;
        self.pixels[index]
    }

    pub fn as_pixel_slice(&self) -> &[Vec4]
    {
        self.pixels.as_slice()
    }
}