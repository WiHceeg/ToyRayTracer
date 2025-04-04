use crate::color::Color;
use crate::enums::NoiseType;
use crate::enums::Scene;

pub const SKY_GRADIENT: Color = Color::new(0.5, 0.7, 1.0);
pub const ENABLE_BVH: bool = true;

pub const TARGET_SCENE: Scene = Scene::FinalSceneLD;

// perlin noise 相关全局设置
pub const NOISE_TYPE: NoiseType = NoiseType::TurbulenceMarble;
pub const HERMITE_CUBIC_SMOOTHED: bool = true;
pub const TURBULENCE_DEPTH: usize = 7;


pub mod config_bouncing_spheres;
pub mod config_checkered_spheres;
pub mod config_cornell_box;
pub mod config_cornell_smoke;
pub mod config_earth;
pub mod config_final_scene;
pub mod config_perlin_spheres;
pub mod config_shapes;
pub mod config_simple_light;