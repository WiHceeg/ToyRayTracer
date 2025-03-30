#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    /// 默认空区间：min = +∞, max = -∞
    pub const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    /// 表示“全集”——所有实数：min = -∞, max = +∞
    pub const UNIVERSE: Self = Self {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    /// 创建一个新的区间
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// 紧紧包围两个区间的新区间
    pub fn new_from_merged(a: Interval, b: Interval) -> Interval {
        Interval {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    /// 返回区间的大小（区间长度）
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// 检查 x 是否落在区间内（包含边界）
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// 检查 x 是否严格在区间内部（不包含边界）
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// 对 x 进行夹紧：如果 x 小于 min 返回 min，
    /// 如果 x 大于 max 返回 max，否则返回 x 本身
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}
