use image;
use image::GenericImageView;

use std::path;

use crate::config::Config;
use crate::error::AppError;

type Image = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub fn combine(config: Config) -> Result<(), AppError> {
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
        let path = path::Path::new(&img.path);

        let image_file = image::open(&path)
            .map_err(|_| AppError::LoadImageError)?;

        let resized_img = resize(
            cell_width,
            cell_height,
            &image_file,
            image::Rgba([
                config.background_color.r,
                config.background_color.g,
                config.background_color.b,
                config.background_color.a,
            ]),
        );

        images_matrix[img.row][img.col] = Some(resized_img);
    }

    let mut combined: Image = image::ImageBuffer::new(config.width, config.height);
    for (x, y, pixel) in combined.enumerate_pixels_mut() {
        let img_row = y / cell_height; 
        let img_col = x / cell_width;

        let img = &images_matrix[img_row as usize][img_col as usize];
        match img {
            Some(image) => {
                let img_pixel = image.get_pixel(x - (img_col * cell_width), y - (img_row * cell_height));
                *pixel = *img_pixel;
            }

            _ => {
                *pixel = image::Rgba([
                    config.background_color.r,
                    config.background_color.g,
                    config.background_color.b,
                    config.background_color.a,
                ])
            }
        }
    }

    combined.save(config.output)
        .map_err(|_| AppError::SaveImageError)?;

    Ok(())
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
