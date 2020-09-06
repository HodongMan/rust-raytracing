use std::cmp;
use super::vec3::Vec3;


#[derive(Debug, Copy, Clone)]
pub struct Ray
{
    origin: Vec3,
    direction: Vec3,
    time: f32
}

impl Default for Ray
{
    #[inline]
    fn default() -> Self
    {
        Self
        {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 1.0, 0.0),
            time: 0.0
        }
    }
}