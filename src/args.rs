// Library
use crate::colors::{GradientMode, RGB};
use std::{collections::HashMap, str::FromStr};

/// Command-Line Arguments
#[derive(Default)]
pub struct Args {
    /// A vector containing all the file-paths to read
    pub files: Vec<std::path::PathBuf>,
    /// Operations Mode
    pub mode: GradientMode,
    /// Gradient's starting color
    pub start_color: RGB<u8>,
    /// Gradient's ending color
    pub end_color: RGB<u8>,
}

impl Args {
    /// Initialize the struct with default values
    fn default() -> Self {
        Self {
            start_color: RGB(255, 0, 0),
            end_color: RGB(0, 0, 255),
            ..Default::default()
        }
    }

    /// Parse the command-line arguments
    pub fn parse() -> Self {
        // Get the command line arguments
        let mut args = std::env::args();
        args.next(); // Consume the path to the executable

        let mut itself = Args::default();
        let mut options = HashMap::new();

        // Collect all valid file-paths
        while let Some(arg) = args.next() {
            if arg.starts_with("--") {
                let key = arg.trim_start_matches("--").to_string();
                if let Some(value) = args.next() {
                    options.insert(key, value);
                } else {
                    options.insert(key, String::from("true"));
                }
                continue;
            }

            let path = std::path::PathBuf::from(arg);
            if std::path::Path::exists(&path) {
                itself.files.push(path);
            }
        }

        // Parse options
        if let Some(start_color) = options.get("start-color") {
            if let Ok(val) = RGB::from_str(&start_color) {
                itself.start_color = val;
            }
        }

        if let Some(end_color) = options.get("end-color") {
            if let Ok(val) = RGB::from_str(&end_color) {
                itself.end_color = val;
            }
        }

        if let Some(mode) = options.get("mode") {
            itself.mode = mode.into();
        }

        // Return self
        itself
    }
}
