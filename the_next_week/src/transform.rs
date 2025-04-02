use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::point3::Point3;
use crate::ray::Ray;

use glam::DVec3;

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: DVec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: DVec3) -> Self {
        let bbox = object.bounding_box().translate(offset);
        Self {
            object: object,
            offset: offset,
            bbox: bbox,
        }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_r = Ray::new_with_time(r.origin() - self.offset, r.direction(), r.time());
        if let Some(mut rec) = self.object.hit(&offset_r, ray_t) {
            rec.p += self.offset;
            Some(rec)
        } else {
            None
        }
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let bbox = object.bounding_box().rotate_y(angle);
        Self {
            object: object,
            sin_theta: radians.sin(),
            cos_theta: radians.cos(),
            bbox: bbox,
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let origin = r.origin();
        let direction = r.direction();
        // 反向旋转 theta
        let new_origin = DVec3::new(
            self.cos_theta * origin.x - self.sin_theta * origin.z,
            origin.y,
            self.sin_theta * origin.x + self.cos_theta * origin.z,
        );
        let new_direction = DVec3::new(
            self.cos_theta * direction.x - self.sin_theta * direction.z,
            direction.y,
            self.sin_theta * direction.x + self.cos_theta * direction.z,
        );
        let rotated_ray = Ray::new_with_time(new_origin, new_direction, r.time());
        if let Some(mut rec) = self.object.hit(&rotated_ray, ray_t) {
            rec.p = Point3::new(
                self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
                rec.p.y,
                -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
            );
            rec.unit_normal = DVec3::new(
                self.cos_theta * rec.unit_normal.x + self.sin_theta * rec.unit_normal.z,
                rec.unit_normal.y,
                -self.sin_theta * rec.unit_normal.x + self.cos_theta * rec.unit_normal.z,
            );
            Some(rec)
        } else {
            None
        }
    }
}
