// use tauri::{Manager, State};
use opencv::prelude::*;
use opencv::imgcodecs::*;
use opencv::imgproc::*;
use opencv::core::Vector;
use opencv::core::Mat;
use opencv::intensity_transform::*;
use base64::{encode};
// use std::convert::TryInto;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn process_threshold(file_data: Vec<u8> , threshold_value: f64) -> Result<String, String> {
    // 解码数据流
    let mat = Mat::from_slice(&file_data).map_err(|e| e.to_string())?;
    let  image = imdecode(&mat, IMREAD_COLOR).map_err(|e| e.to_string())?;

    // 转至灰度图
    let mut gray_image = Mat::default();
    cvt_color(&image, &mut gray_image, COLOR_BGR2GRAY, 0).expect("Failed to convert to grayscale");

    // 应用二值化
    let mut binary_image = Mat::default();
    threshold(&gray_image, &mut binary_image, threshold_value, 255.0, THRESH_BINARY).expect("Failed to apply threshold");

    // 加密数据流
    let mut encoded_image = opencv::core::Vector::<u8>::new();

    imencode(".png", &binary_image, &mut encoded_image, &Vector::<i32>::new()).expect("Failed to encode image");

    let base64_image = encode(&encoded_image);

    Ok(base64_image)
}

#[tauri::command]
fn process_gamma_correction(file_data: Vec<u8>, gamma_value: f32) -> Result<String, String> {
    // 解码数据流
    let mat = Mat::from_slice(&file_data).map_err(|e| e.to_string())?;
    let image = imdecode(&mat, IMREAD_COLOR).map_err(|e| e.to_string())?;

    // 创建目标图像
    let mut corrected_image = Mat::default();
    cvt_color(&image, &mut corrected_image, COLOR_BGR2RGB, 0).expect("Failed to convert to RGB");

    // 获取图像尺寸
    let rows = corrected_image.rows();
    let cols = corrected_image.cols();

    // 应用手动 gamma 校正
    for row in 0..rows {
        for col in 0..cols {
            if let (Ok(r_pixel), Ok(g_pixel), Ok(b_pixel)) = (
                corrected_image.at_3d::<u8>(row, col, 0),
                corrected_image.at_3d::<u8>(row, col, 1),
                corrected_image.at_3d::<u8>(row, col, 2)
            ) {
                let r = (*r_pixel as f32 / 255.0).powf(1.0 / gamma_value) * 255.0;
                let g = (*g_pixel as f32 / 255.0).powf(1.0 / gamma_value) * 255.0;
                let b = (*b_pixel as f32 / 255.0).powf(1.0 / gamma_value) * 255.0;

                corrected_image.at_3d_mut(row, col, 0).map(|p| *p = r as u8);
                corrected_image.at_3d_mut(row, col, 1).map(|p| *p = g as u8);
                corrected_image.at_3d_mut(row, col, 2).map(|p| *p = b as u8);
            }
        }
    }

    // 加密数据流
    let mut encoded_image = opencv::core::Vector::<u8>::new();
    imencode(".png", &corrected_image, &mut encoded_image, &Vector::<i32>::new()).expect("Failed to encode image");

    let base64_image = encode(&encoded_image);
    Ok(base64_image)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet , process_threshold ,process_gamma_correction])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



