use crate::color::Color;
use crate::enums::NoiseType;
use crate::enums::Scene;

pub const SKY_GRADIENT: Color = Color::new(0.5, 0.7, 1.0);
pub const ENABLE_BVH: bool = true;

pub const TARGET_SCENE: Scene = Scene::FinalSceneHD;

// perlin noise 相关全局设置
pub const NOISE_TYPE: NoiseType = NoiseType::TurbulenceMarble;
pub const HERMITE_CUBIC_SMOOTHED: bool = true;
pub const TURBULENCE_DEPTH: usize = 7;
