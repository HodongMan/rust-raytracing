use std::cmp;
use std::ops;

pub struct Vec2
{
    pub x: f32,
    pub y: f32
}

impl Vec2
{
    #[inline]
    pub fn new(x: f32, y: f32) -> Self
    {
        Self
        {
            x,
            y
        }
    }

    #[inline]
    pub fn random_direction() -> Self
    {
        let azimuth = rand::random::<f32>() * 2.0 * std::f32::consts::PI;

        Self
        {
            x: azimuth.cos(),
            y: azimuth.sin()
        }
    }

    #[inline]
    pub fn dot(self, other: Self) -> f32
    {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn cross(self, other: Self) -> f32
    {
        self.x * other.y - self.y * other.x
    }

    #[inline]
    pub fn length_sqrt(self) -> f32
    {
        self.dot(self)
    }

    #[inline]
    pub fn length(self) -> f32
    {
        self.length_sqrt().sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Self
    {
        self / self.length()
    }

    #[inline]
    pub fn is_unit(self) -> bool
    {
        (self.length_sqr() - 1.0).abs() < 0.0001
    }
}