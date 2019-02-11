use image;
use image::GenericImageView;

use std::path;
use std::env;
use crate::config::Config;

type Image = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub fn combine(config: Config) {
    let cell_width = config.width / config.cols;
    let cell_height = config.height / config.rows;

    let mut images_matrix: Vec<Vec<Option<Image>>> = Vec::new();
    for _ in 0..config.rows + 1 {
        let mut images_row = Vec::new();
        for _ in 0..config.cols + 1 {
            images_row.push(None);
        }

        images_matrix.push(images_row);
    }

    for img in &config.images {
        // TODO: check error here
        let mut absolute_path = env::current_dir().unwrap();
        absolute_path.push(&img.path);

        let image_file = image::open(&absolute_path)
            .unwrap();

        println!("img.path {:?}", absolute_path);
        // TODO: usize vs u32
        let resized_img = resize(
            cell_width as u32,
            cell_height as u32,
            &image_file,
            // TODO: replace with background_color
            image::Rgba([100, 100, 100, 0]),
        );

        images_matrix[img.row][img.col] = Some(resized_img);
    }

    let mut combined: Image = image::ImageBuffer::new(config.width as u32, config.height as u32);
    for (x, y, pixel) in combined.enumerate_pixels_mut() {
        let img_row = (y / cell_height as u32); 
        let img_col = (x / cell_width as u32);

        let img = &images_matrix[img_row as usize][img_col as usize];
        match img {
            Some(image) => {
                let img_pixel = image.get_pixel(x - (img_col * cell_width as u32), y - (img_row * cell_height as u32));

                *pixel = *img_pixel;
            }

            _ => {
                // TODO: replace with background_color
                *pixel = image::Rgba([100, 100, 100, 0]);
            }
        }
    }

    combined.save(config.output).unwrap();
}

fn resize(
    width: u32,
    height: u32,
    image: &image::DynamicImage,
    pad: image::Rgba<u8>,
) -> Image {
    let resized_img = image.resize(width, height, image::FilterType::Nearest);
    let mut padded_img = image::ImageBuffer::new(width, height);

    let resized_img_dimensions = resized_img.dimensions();

    // Pad image vertically
    if resized_img_dimensions.0 == width {
        let y_offset = (height - resized_img_dimensions.1) / 2;

        for (x, y, pixel) in padded_img.enumerate_pixels_mut() {
            if y < y_offset || y >= (y_offset + resized_img_dimensions.1) {
                *pixel = pad;
            } else {
                *pixel = resized_img.get_pixel(x, y - y_offset);
            }
        }
    }

    // Pad image horizontally
    else if resized_img_dimensions.1 == height {
        let x_offset = (width - resized_img_dimensions.0) / 2;

        for (x, y, pixel) in padded_img.enumerate_pixels_mut() {
            if x < x_offset || x >= (x_offset + resized_img_dimensions.0) {
                *pixel = pad;
            } else {
                *pixel = resized_img.get_pixel(x - x_offset, y);
            }
        }
    }

    padded_img
}
