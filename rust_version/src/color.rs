use std::io::Write;

use crate::constant::INTENSITY;

pub type Color = glam::DVec3;

pub trait ColorExt {
    fn write_color<W: Write>(&self, writer: &mut W) -> std::io::Result<()>;
}

impl ColorExt for Color {
    fn write_color<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;

        // Apply a linear to gamma transform for gamma 2
        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);

        // 将 [0,1] 范围内的值转换为 [0,255] 范围内的整数
        let r_byte = (256. * INTENSITY.clamp(r)) as u8;
        let g_byte = (256. * INTENSITY.clamp(g)) as u8;
        let b_byte = (256. * INTENSITY.clamp(b)) as u8;

        // 写入文件
        writeln!(writer, "{} {} {}", r_byte, g_byte, b_byte)
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. {
        linear_component.sqrt()
    } else {
        0.
    }
}
