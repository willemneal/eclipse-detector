//! Port of https://github.com/opencv/opencv/blob/4.7.0/samples/cpp/tutorial_code/ImgTrans/HoughCircle_Demo.cpp

use rayon::prelude::*;
use std::env::args;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

use opencv::core::{find_file, Size2i, BORDER_DEFAULT};
use opencv::imgcodecs::{imread, imwrite, IMREAD_COLOR};
use opencv::imgproc::{
    cvt_color_def, gaussian_blur, hough_circles, COLOR_BGR2GRAY, HOUGH_GRADIENT,
};
use opencv::prelude::*;
use opencv::types::VectorOfVec3f;
use opencv::Result;

opencv::not_opencv_branch_4! {
     use opencv::core::LINE_8;
}
// Initial and max values of the parameters of interest
const CANNY_THRESHOLD_INIT_VAL: i32 = 10;
const ACCUMULATOR_THRESHOLD_INIT_VAL: i32 = 10;

fn find_cicle(
    src_gray: &Mat,
    _src_display: &Mat,
    canny_threshold: i32,
    accumulator_threshold: i32,
) -> Result<VectorOfVec3f> {
    // Will hold the results of the detection
    let mut circles = VectorOfVec3f::new();

    // Runs the actual detection
    hough_circles(
        &src_gray,
        &mut circles,
        HOUGH_GRADIENT,
        1.0,
        (src_gray.rows() / 8).into(),
        canny_threshold.into(),
        accumulator_threshold.into(),
        0,
        0,
    )?;
    Ok(circles)
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <input_dir> <output_dir>", args[0]);
        std::process::exit(2);
    }
    let input_dir = PathBuf::from(&args[1]);
    let out_dir = PathBuf::from(&args[2]);
    fs::read_dir(input_dir)
        .unwrap()
        .map(|p| p.unwrap().path())
        .collect::<Vec<_>>()
        .par_iter()
        .for_each(|p| {
            let out_name = out_dir.join(p.file_name().unwrap());

            if out_name.exists() {
                return;
            }
            if let Err(e) = read_image(p, &out_name) {
                eprintln!("failed to process image:{:?}\n{e}", p.file_name().unwrap());
            }
        });
    Ok(())
    // read_image(&image_path, &out_dir)
}

fn read_image(img_name: &Path, out_name: &Path) -> Result<()> {
    let src = imread(
        &find_file(&img_name.to_string_lossy(), true, false)?,
        IMREAD_COLOR,
    )?;
    if src.empty() {
        eprintln!("Invalid input image");
        println!(
            "Usage: {} <path_to_input_image>",
            args().next().unwrap_or_else(|| "program".to_string())
        );
        return Ok(());
    }

    // Convert it to gray
    let mut src_gray = Mat::default();
    cvt_color_def(&src, &mut src_gray, COLOR_BGR2GRAY)?;

    // Reduce the noise so we avoid false circle detection
    let mut src_gray_blur = Mat::default();
    gaussian_blur(
        &src_gray,
        &mut src_gray_blur,
        Size2i::new(9, 9),
        2.0,
        2.0,
        BORDER_DEFAULT,
    )?;

    // Declare and initialize both parameters that are subjects to change
    let canny_threshold: Arc<AtomicI32> = Arc::new(AtomicI32::new(CANNY_THRESHOLD_INIT_VAL));
    let accumulator_threshold: Arc<AtomicI32> =
        Arc::new(AtomicI32::new(ACCUMULATOR_THRESHOLD_INIT_VAL));

    // // Create the main window, and attach the trackbars
    // named_window(WINDOW_NAME, WINDOW_AUTOSIZE)?;

    // create_trackbar(
    //     CANNY_THRESHOLD_TRACKBAR_NAME,
    //     WINDOW_NAME,
    //     None,
    //     MAX_CANNY_THRESHOLD,
    //     Some(Box::new({
    //         let canny_threshold = canny_threshold.clone();
    //         move |val| {
    //             canny_threshold.as_ref().store(val, Ordering::SeqCst);
    //         }
    //     })),
    // )?;

    // create_trackbar(
    //     ACCUMULATOR_THRESHOLD_TRACKBAR_NAME,
    //     WINDOW_NAME,
    //     None,
    //     MAX_ACCUMULATOR_THRESHOLD,
    //     Some(Box::new({
    //         let accumulator_threshold = accumulator_threshold.clone();
    //         move |val| {
    //             accumulator_threshold.as_ref().store(val, Ordering::SeqCst);
    //         }
    //     })),
    // )?;

    // Infinite loop to display
    // and refresh the content of the output image
    // until the user presses q or Q
    // let mut key: char = ' ';
    // while key.to_ascii_lowercase() != 'q' {
    // Those parameters cannot be = 0, so we must check here
    let canny_threshold_val = canny_threshold.fetch_max(1, Ordering::SeqCst);
    let accumulator_threshold_val = accumulator_threshold.fetch_max(1, Ordering::SeqCst);

    // Runs the detection, and update the display
    let cirlce = find_cicle(
        &src_gray_blur,
        &src,
        canny_threshold_val,
        accumulator_threshold_val,
    )?
    .get(0)
    .expect("no circle found");
    let (x, y, r) = (cirlce[0], cirlce[1], cirlce[2]);
    let xmin = (x - r * 2.0) as i32;
    let ymin = (y - r * 2.0) as i32;
    let height = 1000;
    let width = 1000;
    let cropped_image = Mat::roi(
        &src,
        opencv::core::Rect {
            x: xmin,
            y: ymin,
            width,
            height,
        },
    )
    .unwrap();
    let params = opencv::types::VectorOfi32::new();
    imwrite(&out_name.to_string_lossy(), &cropped_image, &params).expect("faield to write image");
    //     // Get user key
    //     key = wait_key(10).unwrap() as u8 as char;
    // }

    Ok(())
}
