use image;
use image::GenericImageView;
use image::FilterType;
use image::ImageBuffer;
use image::Rgba;
use image::DynamicImage;

fn main() {
    let width = 1200;
    let height = 600;
    let rows = 2;
    let cols = 3;

    let cell_width = width / cols;
    let cell_height = height / rows;

    let cat = image::open("./sample_images/cat.jpg").unwrap();
    let cat2 = image::open("./sample_images/cat2.jpg").unwrap();
    let cat3 = image::open("./sample_images/cat3.jpg").unwrap();
    let dog = image::open("./sample_images/dog.jpg").unwrap();
    let dog2 = image::open("./sample_images/dog2.jpg").unwrap();
    let dog3 = image::open("./sample_images/dog3.jpg").unwrap();

    let catbuf = resize(cell_width, cell_height, &cat, Rgba([100, 100, 100, 0]));
    let cat2buf = resize(cell_width, cell_height, &cat2, Rgba([100, 100, 100, 0]));
    let cat3buf = resize(cell_width, cell_height, &cat3, Rgba([100, 100, 100, 0]));
    let dogbuf = resize(cell_width, cell_height, &dog, Rgba([100, 100, 100, 0]));
    let dog2buf = resize(cell_width, cell_height, &dog2, Rgba([100, 100, 100, 0]));
    let dog3buf = resize(cell_width, cell_height, &dog3, Rgba([100, 100, 100, 0]));

    let imgs = vec![
        vec![catbuf, cat2buf, cat3buf],
        vec![dogbuf, dog2buf, dog3buf],
    ];

    let mut combobuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for (x, y, pixel) in combobuf.enumerate_pixels_mut() {
        let img_row = (y / cell_height) as u32; 
        let img_col = (x / cell_width) as u32;

        let img = &imgs[img_row as usize][img_col as usize];
        let img_pixel = img.get_pixel(x - (img_col * cell_width), y - (img_row * cell_height));

        *pixel = *img_pixel;
    }

    combobuf.save("./output.jpg").unwrap();
}

fn resize(
    width: u32,
    height: u32,
    image: &DynamicImage,
    pad: Rgba<u8>,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let resized_img = image.resize(width, height, FilterType::Nearest);
    let mut padded_img = ImageBuffer::new(width, height);

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
