use tauri::{Manager, State};
use opencv::prelude::*;
use opencv::imgcodecs::*;
use opencv::imgproc::*;
use opencv::core::Vector;
use opencv::core::Mat;
use base64::{encode};
use std::convert::TryInto;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn process_threshold(file_data: Vec<u8>) -> Result<String, String> {
    // Decode the image data
    let mat = Mat::from_slice(&file_data).map_err(|e| e.to_string())?;
    let  image = imdecode(&mat, IMREAD_COLOR).map_err(|e| e.to_string())?;

    // Convert to grayscale
    let mut gray_image = Mat::default();
    cvt_color(&image, &mut gray_image, COLOR_BGR2GRAY, 0).expect("Failed to convert to grayscale");

    // Apply thresholding to binarize the image
    let mut binary_image = Mat::default();
    threshold(&gray_image, &mut binary_image, 128.0, 255.0, THRESH_BINARY).expect("Failed to apply threshold");

    // Encode the processed image to base64
    let mut encoded_image = opencv::core::Vector::<u8>::new();

    imencode(".png", &binary_image, &mut encoded_image, &Vector::<i32>::new()).expect("Failed to encode image");

    let base64_image = encode(&encoded_image);

    Ok(base64_image)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet , process_threshold])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



