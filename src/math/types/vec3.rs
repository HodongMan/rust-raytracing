use super::vec2::Vec2;
use rand::Rng;

#[cfg(target_feature = "sse2")]
pub mod sse2
{

}

pub mod scalar
{
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Vec3(pub f32, pub f32, pub f32)

    impl Vec3
    {
        #[inline]
        pub fn new(x: f32, y: f32, z: f32) -> Self
        {
            Self(x, y, z)
        }

    }
}