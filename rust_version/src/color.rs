use std::io::Write;

use crate::constant::INTENSITY;

pub type Color = glam::DVec3;

pub trait ColorExt {

    fn write_color<W: Write>(&self, writer: &mut W) -> std::io::Result<()>;
}

impl ColorExt for Color {
    fn write_color<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {

        let r = self.x;
        let g = self.y;
        let b = self.z;

        // 将 [0,1] 范围内的值转换为 [0,255] 范围内的整数
        let r_byte = (256. * INTENSITY.clamp(r)) as u8;
        let g_byte = (256. * INTENSITY.clamp(g)) as u8;
        let b_byte = (256. * INTENSITY.clamp(b)) as u8;

        // 写入文件
        writeln!(writer, "{} {} {}", r_byte, g_byte, b_byte)
    }
}