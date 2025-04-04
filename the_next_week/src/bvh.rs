use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(mut list: HittableList) -> BvhNode {
        let sz = list.objects.len();
        BvhNode::build(&mut list.objects, 0, sz)
    }

    fn build(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> BvhNode {
        let mut bbox = Aabb::EMPTY;
        for i in start..end {
            bbox = Aabb::new_from_merged(bbox, objects[i].bounding_box());
        }
        let axis = bbox.longest_axis();
        let object_span = end - start;

        match object_span {
            1 => BvhNode {
                left: objects[start].clone(),
                right: objects[start].clone(),
                bbox,
            },
            2 => BvhNode {
                left: objects[start].clone(),
                right: objects[start + 1].clone(),
                bbox,
            },
            _ => {
                let comparator = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
                    let a_axis_min = a.bounding_box()[axis].min;
                    let b_axis_min = b.bounding_box()[axis].min;
                    a_axis_min.partial_cmp(&b_axis_min).unwrap()
                };
                objects[start..end].sort_by(comparator);
                let mid = start + object_span / 2;
                BvhNode {
                    left: Arc::new(BvhNode::build(objects, start, mid)),
                    right: Arc::new(BvhNode::build(objects, mid, end)),
                    bbox,
                }
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }
        // 先检查左子树
        match self.left.hit(r, ray_t) {
            Some(left_rec) => {
                // 如果左子树命中，用更窄的区间检查右子树
                let right_t = Interval::new(ray_t.min, left_rec.t);
                self.right.hit(r, right_t).or(Some(left_rec))
            }
            None => {
                // 左子树未命中，正常检查右子树
                self.right.hit(r, ray_t)
            }
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
