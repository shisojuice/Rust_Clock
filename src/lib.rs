use std::io::{ Cursor};
use wasm_bindgen::prelude::*;
use image::{DynamicImage, ImageFormat, Rgba};
use base64::{engine::general_purpose, Engine as _};
use imageproc::drawing::{draw_filled_circle_mut, draw_line_segment_mut};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn clock_png(h : u32,m : u32,s : u32) -> String{
    let mut img = DynamicImage::new_rgba8(320,320);
    // 円の中心座標
    let center_x = img.width() / 2;
    let center_y = img.height() / 2;
    draw_filled_circle_mut(&mut img, (center_x as i32, center_y as i32), 155, Rgba([0, 0, 0, 255]));
    draw_filled_circle_mut(&mut img, (center_x as i32, center_y as i32), 153, Rgba([0, 0, 0, 0]));
    draw_filled_circle_mut(&mut img, (center_x as i32, center_y as i32), 5, Rgba([0, 0, 0, 255]));

    let s_angle = ((s as f64 / 60.0) * 360.0- 90.0).to_radians();
    draw_line_segment_mut(&mut img,(center_x as f32, center_y as f32),(center_x as f32 + 140.0 * s_angle.cos() as f32,center_y as f32 + 140.0 * s_angle.sin() as f32),Rgba([0, 0, 0, 255])); // 秒針

    let m_angle = (((m as f64 + s  as f64/ 60.0) / 60.0) * 360.0 - 90.0).to_radians();
    draw_line_segment_mut(&mut img,(center_x as f32, center_y as f32),(center_x as f32 + 120.0 * m_angle.cos() as f32,center_y as f32 + 120.0 * m_angle.sin() as f32),Rgba([0, 0, 0, 255])); // 分針

    let h_angle = (((h as f64 % 12.0 + m as f64 / 60.0 + s as f64 / 3600.0) / 12.0) * 360.0 - 90.0).to_radians();
    draw_line_segment_mut(&mut img,(center_x as f32, center_y as f32),(center_x as f32 + 80.0 * h_angle.cos() as f32,center_y as f32 + 80.0 * h_angle.sin() as f32),Rgba([0, 0, 0, 255])); // 時針

    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png).unwrap();
    let base64_string = general_purpose::STANDARD.encode(buffer.get_ref());
    // データURL形式で返す
    format!("data:image/png;base64,{}", base64_string)
}
