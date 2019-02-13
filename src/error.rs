use std::error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    // Config related
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
    ParseBackgroundColorError,
    NeedImagesError,
    ParseImageError,
    InvalidRowsColsError,

    // Image resize and combine related
    LoadImageError,
    SaveImageError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NeedOutputError =>
                write!(f, "Need OUTPUT in config"),

            AppError::NeedRowsError =>
                write!(f, "Need ROWS in config"),

            AppError::ParseRowsError =>
                write!(f, "Failed to parse ROWS"),

            AppError::NeedColsError =>
                write!(f, "Need COL in config"),

            AppError::ParseColsError =>
                write!(f, "Failed to parse COLS"),

            AppError::NeedHeightError =>
                write!(f, "Need HEIGHT in config"),

            AppError::ParseHeightError =>
                write!(f, "Failed to parse HEIGHT"),

            AppError::NeedWidthError =>
                write!(f, "Need WIDTH in config"),

            AppError::ParseWidthError =>
                write!(f, "Failed to parse WIDTH"),

            AppError::NeedBackgroundColorError =>
                write!(f, "Need BACKGROUND_COLOR in config"),

            AppError::ParseBackgroundColorError =>
                write!(f, "BACKGROUND_COLOR needs to be in format <red,green,blue,alpha>"),

            AppError::NeedImagesError =>
                write!(f, "Need IMAGE in config"),

            AppError::ParseImageError =>
                write!(f, "IMAGE needs to be in format <row,col,path_to_image>"),

            AppError::InvalidRowsColsError =>
                write!(f, "ROWS or COLS cannot be lesser than 1"),

            AppError::LoadImageError =>
                write!(f, "Failed to load image"),

            AppError::SaveImageError =>
                write!(f, "Failed to save image"),
        }
    }
}

impl error::Error for AppError {
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

