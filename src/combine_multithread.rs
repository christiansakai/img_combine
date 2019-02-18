use image;
use image::GenericImageView;

use std::path;
use std::time::SystemTime;
use std::thread;
use std::sync::{Arc, Mutex};

use crate::config::{Config, ImageConfig};
use crate::error::AppError;

type Image = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub fn combine(config: Config) -> Result<(), AppError> {
    let cell_width = config.width / config.cols;
    let cell_height = config.height / config.rows;

    let mut images_matrix: Vec<Vec<Option<Image>>> = Vec::new();
    let mut threads = Vec::new();

    for _ in 0..config.rows + 1 {
        let mut images_row = Vec::new();
        for _ in 0..config.cols + 1 {
            images_row.push(None);
        }

        images_matrix.push(images_row);
    }

    let load_and_resize = SystemTime::now();

    let images_matrix = Arc::new(Mutex::new(images_matrix));

    let image_jobs = separate(config.threads, &config.images);

    for image_job in image_jobs {
        let images_matrix = images_matrix.clone();
        let bg_color = config.background_color;

        let other_thread = thread::spawn(move || {
            for img in image_job {
                let img_path = img.path;
                let path = path::Path::new(&img_path);

                let load = SystemTime::now();

                let image_file = image::open(&path)
                    // TODO: Handle error
                    .unwrap();

                let load_finish = load.elapsed().unwrap();
                println!("Load: {} secs", load_finish.as_secs());

                let resizing = SystemTime::now();

                let resized_img = resize(
                    cell_width,
                    cell_height,
                    &image_file,
                    image::Rgba([
                        bg_color.r,
                        bg_color.g,
                        bg_color.b,
                        bg_color.a,
                    ]),
                );

                let resizing_finish = resizing.elapsed().unwrap();
                println!("Resize: {} secs", resizing_finish.as_secs());

                let mut images_matrix = images_matrix.lock()
                    // TODO: Handle error
                    .unwrap();

                images_matrix[img.row][img.col] = Some(resized_img);
            }
        });

        threads.push(other_thread);
    }

    for other_thread in threads {
        // TODO: Handle error
        other_thread.join().unwrap();
    }

    let load_and_resize_finish = load_and_resize.elapsed().unwrap();
    println!("Total load and resize: {} secs", load_and_resize_finish.as_secs());

    let combine = SystemTime::now();

    let mut combined: Image = image::ImageBuffer::new(config.width, config.height);
    for (x, y, pixel) in combined.enumerate_pixels_mut() {
        let img_row = y / cell_height; 
        let img_col = x / cell_width;

        let images_matrix = images_matrix.lock().unwrap();
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

    let combine_finish = combine.elapsed().unwrap();
    println!("Combining: {} secs", combine_finish.as_secs());

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

fn separate(group_num: u8, vector: &Vec<ImageConfig>) -> Vec<Vec<ImageConfig>> {
    let el_per_group = vector.len() / group_num as usize;

    let mut result: Vec<Vec<ImageConfig>> = Vec::new();

    for el in vector {
        match result.last_mut() {
            Some(ref mut vec) if vec.len() < el_per_group => {
                vec.push(el.clone());
            },
            _ => {
                let new_vec = vec![el.clone()];
                result.push(new_vec);
            },
        }
    }
    
    result
}
