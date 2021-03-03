extern crate qrcode_generator;

use qrcode_generator::*;

use crate::bp::builder::Builder;

pub trait QrGenerator {
    fn add_qr_code(&mut self, x: f32, y: f32, val: &str);
}

impl QrGenerator for Builder {
    fn add_qr_code(&mut self, x: f32, y: f32, val: &str) {
        let val = qrcode_generator::to_matrix_from_str(val, QrCodeEcc::High).unwrap();
        for (ix, v) in val.iter().enumerate() {
            for (iy, b) in v.iter().enumerate() {
                if *b {
                    self.add("stone-wall", x + ix as f32, y + iy as f32);
                }
            }
        }
    }
}
