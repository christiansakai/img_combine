use std::collections::HashMap;
use std::fmt;
use std::error;

#[derive(Debug)]
pub struct Config {
    pub output: String,
    pub rows: usize,
    pub cols: usize,
    pub height: usize,
    pub width: usize,
    pub background_color: String,
    pub images: Vec<ImageConfig>,
}

impl Config {
    pub fn new(config_str: &str) -> Result<Config, ConfigError> {
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
            .ok_or(ConfigError::NeedOutputError)?;

        let rows = map.get("rows")
            .ok_or(ConfigError::NeedRowsError)?
            .parse::<usize>()
            .map_err(|_| ConfigError::ParseRowsError)?;

        let cols = map.get("cols")
            .ok_or(ConfigError::NeedColsError)?
            .parse::<usize>()
            .map_err(|_| ConfigError::ParseColsError)?;

        if rows < 1 || cols < 1 {
            return Err(ConfigError::InvalidRowsColsError);
        }

        let height = map.get("height")
            .ok_or(ConfigError::NeedHeightError)?
            .parse::<usize>()
            .map_err(|_| ConfigError::ParseHeightError)?;

        let width = map.get("width")
            .ok_or(ConfigError::NeedWidthError)?
            .parse::<usize>()
            .map_err(|_| ConfigError::ParseWidthError)?;

        let background_color = map.get("background_color")
            .ok_or(ConfigError::NeedBackgroundColorError)?;

        if images.len() < 1 {
            return Err(ConfigError::NeedImagesError);
        }

        Ok(Config {
            output: output.to_string(),
            rows,
            cols,
            height,
            width,
            background_color: background_color.to_string(),
            images,
        })
    }
}

#[derive(Debug)]
pub enum ConfigError {
    NeedOutputError,
    NeedRowsError,
    ParseRowsError,
    NeedColsError,
    ParseColsError,
    NeedHeightError,
    ParseHeightError,
    NeedWidthError,
    ParseWidthError,
    NeedBackgroundColorError,
    NeedImagesError,
    ParseImageError,
    InvalidRowsColsError,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::NeedOutputError => write!(f, "Need OUTPUT in config"),
            ConfigError::NeedRowsError => write!(f, "Need ROWS in config"),
            ConfigError::ParseRowsError => write!(f, "Failed to parse ROWS"),
            ConfigError::NeedColsError => write!(f, "Need COL in config"),
            ConfigError::ParseColsError => write!(f, "Failed to parse COLS"),
            ConfigError::NeedHeightError => write!(f, "Need HEIGHT in config"),
            ConfigError::ParseHeightError => write!(f, "Failed to parse HEIGHT"),
            ConfigError::NeedWidthError => write!(f, "Need WIDTH in config"),
            ConfigError::ParseWidthError => write!(f, "Failed to parse WIDTH"),
            ConfigError::NeedBackgroundColorError => write!(f, "Need BACKGROUND_COLOR in config"),
            ConfigError::NeedImagesError => write!(f, "Need IMAGE in config"),
            ConfigError::ParseImageError => write!(f, "IMAGE needs to be in format <row,col,path_to_image>"),
            ConfigError::InvalidRowsColsError => write!(f, "ROWS or COLS cannot be lesser than 1"),
        }
    }
}

impl error::Error for ConfigError {
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug)]
pub struct ImageConfig {
    pub row: usize,
    pub col: usize,
    pub path: String,
}

impl ImageConfig {
    fn new(image_str: &str) -> Result<ImageConfig, ConfigError> {
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
                    _ => Err(ConfigError::ParseImageError),
                }
            },
            _ => Err(ConfigError::ParseImageError),
        }
    }
}



