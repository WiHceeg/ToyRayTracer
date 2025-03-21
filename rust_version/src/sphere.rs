use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::point3::Point3;
use crate::ray::Ray;

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius.max(0.),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let b = -2.0 * r.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-b - discriminant.sqrt()) / (2. * a);
        if root <= ray_tmin || root >= ray_tmax {
            root = (-b + discriminant.sqrt()) / (2. * a);
            if root <= ray_tmin || root >= ray_tmax {
                return None;
            }
        }

        let p = r.at(root);

        let mut rec = HitRecord::default();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }
}
