
use std::{env,path::PathBuf};
use image::{ImageBuffer, Pixel,RgbImage};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <dir> <out-file>", args[0]);
        std::process::exit(2);
    }
    let dir = PathBuf::from(&args[1]);
    let out_file = &args[2];
    let dir = std::fs::read_dir(dir).unwrap();
    let imgs: Vec<RgbImage> = dir
        .map(|path| image::open(path.unwrap().path()).unwrap().to_rgb8())
        .collect();

    // Ensure all images are the same size
    let (width, height) = imgs[0].dimensions();
    for img in &imgs {
        assert_eq!(img.dimensions(), (width, height));
    }

    // Create a new image buffer to store the averaged image
    let mut avg_img: RgbImage = ImageBuffer::new(width, height);

    // Iterate over each pixel position
    for x in 0..width {
        for y in 0..height {
            let mut sum_r = 0u32;
            let mut sum_g = 0u32;
            let mut sum_b = 0u32;

            // Sum the color values for each image at this pixel
            for img in &imgs {
                let pixel = img.get_pixel(x, y).channels();
                sum_r += pixel[0] as u32;
                sum_g += pixel[1] as u32;
                sum_b += pixel[2] as u32;
            }

            // Calculate the average for each color channel
            let avg_r = (sum_r / imgs.len() as u32) as u8;
            let avg_g = (sum_g / imgs.len() as u32) as u8;
            let avg_b = (sum_b / imgs.len() as u32) as u8;

            // Set the pixel in the averaged image
            avg_img.put_pixel(x, y, image::Rgb([avg_r, avg_g, avg_b]));
        }
    }

    // Save the averaged image
    avg_img.save(format!("{out_file}.png")).unwrap();
    // // Read the JPG image from the file
    // let img = image::open(&file).unwrap();
    // println!("Dimensions: {:?}", img.dimensions());
    // let grey_image = img.to_luma8();
    // let detection = canny::<ImageBuffer<Luma<u8>, Vec<u8>>>(grey_image, 1.2, 0.2, 0.01);

    // detection
    //     .as_image()
    //     .save(format!("./edges.{:?}", file.file_name()))
    //     .unwrap();
    // // Convert the image to grayscale
    // let grayscale_img = grayscale(&img);

    // // Detect circles in the grayscale image
    // let circles = hough_circles(grayscale_img, 5.0, 10.0, 100.0, 100.0, 50.0);

    // // Process the detected circles as needed
    // for circle in circles {
    //     // Access the center coordinates of the circle
    //     let (x, y) = circle;

    //     // Process the circle further based on your requirements
    // }
}

// // use std::fs::File;
// // use std::io::prelude::*;
// // use std::io::BufWriter;

// use rawloader::RawImage;

// fn main() {
// let args: Vec<_> = env::args().collect();
// if args.len() != 2 {
//     println!("Usage: {} <file>", args[0]);
//     std::process::exit(2);
// }
// let file = &args[1];
//     let RawImage {
//         make,
//         model,
//         clean_make,
//         clean_model,
//         width,
//         height,
//         cpp,
//         wb_coeffs,
//         whitelevels,
//         blacklevels,
//         xyz_to_cam,
//         cfa,
//         crops,
//         orientation,
//         ..
//     } = rawloader::decode_file(file).unwrap();

//     println!(r#"Image:
// {file}
// make: {make}
// model: {model}
// clean_make: {clean_make}
// clean_model: {clean_model}
// width: {width}
// height: {height}
// cpp: {cpp}
// wb_coeffs: {wb_coeffs:#?}
// whitelevels: {whitelevels:#?}
// blacklevels: {blacklevels:#?}
// xyz_to_cam: {xyz_to_cam:#?}
// cfa: {cfa:#?}
// crops: {crops:#?}
// orientation: {orientation:#?}
// "#);

//     // // Write out the image as a grayscale PPM
//     // let mut f = BufWriter::new(File::create(format!("{}.ppm": {// let mut f = BufWriter::new(File::create(format!("{}.ppm"file)).unwrap());}
//     // let preamble = format!("P6 {} {} {}\n", image.width, image.height, 65535).into_bytes();
//     // f.write_all(&preamble).unwrap();
//     // if let rawloader::RawImageData::Integer(data) = image.data {
//     //   for pix in data {
//     //     // Do an extremely crude "demosaic" by setting R=G=B
//     //     let pixhigh = (pix>>8) as u8;
//     //     let pixlow  = (pix&0x0f) as u8;
//     //     f.write_all(&[pixhigh, pixlow, pixhigh, pixlow, pixhigh, pixlow]).unwrap()
//     //   }
//     // } else {
//     //   eprintln!("Don't know how to process non-integer raw files");
//     // }
// }
