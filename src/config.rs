use std::collections::HashMap;

use crate::error::AppError;

#[derive(Debug)]
pub struct Config {
    pub output: String,
    pub rows: u32,
    pub cols: u32,
    pub height: u32,
    pub width: u32,
    pub background_color: Rgba,
    pub images: Vec<ImageConfig>,
    pub threads: u8,
}

impl Config {
    pub fn new(config_str: &str) -> Result<Config, AppError> {
        let str_split = config_str.trim().split('\n');
        let mut map = HashMap::new();
        let mut images = Vec::new();

        for line in str_split {
            let key_val = line
                .split('=')
                .map(|el| el.trim())
                .collect::<Vec<&str>>();

            if key_val[0] == "IMAGE" {
                let image = ImageConfig::new(key_val[1])?;
                images.push(image);
            } else {
                let key = key_val[0].to_lowercase();
                let val = key_val[1].to_owned();

                map.insert(key, val);
            }
        }

        let output = map.get("output")
            .ok_or(AppError::NeedOutputError)?;

        let rows = map.get("rows")
            .ok_or(AppError::NeedRowsError)?
            .parse()
            .map_err(|_| AppError::ParseRowsError)?;

        let cols = map.get("cols")
            .ok_or(AppError::NeedColsError)?
            .parse()
            .map_err(|_| AppError::ParseColsError)?;

        if rows < 1 || cols < 1 {
            return Err(AppError::InvalidRowsColsError);
        }

        let height = map.get("height")
            .ok_or(AppError::NeedHeightError)?
            .parse()
            .map_err(|_| AppError::ParseHeightError)?;

        let width = map.get("width")
            .ok_or(AppError::NeedWidthError)?
            .parse()
            .map_err(|_| AppError::ParseWidthError)?;

        let channels: Vec<&str> = map.get("background_color")
            .ok_or(AppError::NeedBackgroundColorError)?
            .split(',')
            .collect();

        let mut background_color: Vec<u8> = Vec::new();
        for channel_str in channels {
            let channel = channel_str
                .parse()
                .map_err(|_| AppError::ParseBackgroundColorError)?;

            background_color.push(channel);
        }

        let bgcolor_rgba = Rgba {
            r: background_color[0],
            g: background_color[1],
            b: background_color[2],
            a: background_color[3],
        };

        if images.len() < 1 {
            return Err(AppError::NeedImagesError);
        }

        let threads = map.get("threads")
            .unwrap_or(&String::from("1"))
            .parse()
            .unwrap();

        Ok(Config {
            output: output.to_string(),
            rows,
            cols,
            height,
            width,
            background_color: bgcolor_rgba,
            images,
            threads,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ImageConfig {
    pub row: usize,
    pub col: usize,
    pub path: String,
}

impl ImageConfig {
    fn new(image_str: &str) -> Result<ImageConfig, AppError> {
        let image_str_split: Vec<&str> = image_str
            .split(',')
            .collect();

        let row = image_str_split.get(0);
        let col = image_str_split.get(1);
        let path = image_str_split.get(2);

        match (row, col, path) {
            (Some(row), Some(col), Some(path)) => {
                let row = row.parse();
                let col = col.parse();

                match (row, col) {
                    (Ok(row), Ok(col)) => {
                        Ok(ImageConfig {
                            row,
                            col,
                            path: (*path).to_owned(),
                        })
                    },
                    _ => Err(AppError::ParseImageError),
                }
            },
            _ => Err(AppError::ParseImageError),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}
