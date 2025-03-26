use std::sync::Arc;

use crate::hittable::Hittable;
use crate::aabb::Aabb;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::hit_record::HitRecord;

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
        if object_span == 1 {
            BvhNode {
                left: objects[start].clone(),
                right: objects[start].clone(),
                bbox: bbox,
            }
        } else if object_span == 2 {
            BvhNode {
                left: objects[start].clone(),
                right: objects[start + 1].clone(),
                bbox: bbox,
            }
        } else {
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
                bbox: bbox,
            }
        }

    }
}

impl Hittable for BvhNode {

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }
        let left_hit = self.left.hit(r, ray_t);
        // 根据左子节点的结果调整右子节点的时间区间
        let right_t = match left_hit {
            Some(ref hit) => Interval::new(ray_t.min, hit.t),
            None => ray_t,
        };
        // 计算右子节点的命中结果（在调整后的时间区间内）
        let right_hit = self.right.hit(r, right_t);
        // 合并结果：右子节点命中优先（因时间更早），否则返回左子节点结果
        right_hit.or(left_hit)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
    
}