use serde::{ Serialize, Deserialize };
use crate::math::Ray;
use crate::math::Intersection;

use super::camera;
use super::objects::Object;


#[derive(Default, Serialize, Deserialize)]
pub struct Scene
{
    pub camera: camera::Parameters,
    objects: Vec<Object>
}


impl Scene
{
    pub fn new() -> Self
    {
        Self::default()
    }
}