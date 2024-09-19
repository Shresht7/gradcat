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

    /// Rainbow gradient's sine-wave frequency (Higher values cause faster change in the pattern)
    pub frequency: f32,
    /// Rainbow gradient's spread value
    pub spread: f32,

    /// Gradient's starting color
    pub start_color: RGB<u8>,
    /// Gradient's ending color
    pub end_color: RGB<u8>,

    pub show_help: bool,
    pub show_version: bool,
}

impl Args {
    /// Initialize the struct with default values
    fn default() -> Self {
        Self {
            start_color: RGB(255, 0, 0),
            end_color: RGB(0, 0, 255),
            frequency: 1.0,
            spread: 3.0,
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

        if let Some(mode) = options.get("mode") {
            itself.mode = mode.into();
        }

        if let Some(frequency) = options.get("frequency") {
            itself.frequency = frequency.parse().expect("Invalid frequency value");
        }

        if let Some(spread) = options.get("spread") {
            itself.spread = spread.parse().expect("Invalid spread value");
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

        if options.contains_key("help") {
            itself.show_help = true;
        }

        if options.contains_key("version") {
            itself.show_version = true;
        }

        // Return self
        itself
    }

    pub fn help_message(&self) -> String {
        let mut help_message = String::new();

        help_message.push_str("Usage: gradcat [FILES...] [OPTIONS]\n");
        help_message.push_str("\n");
        help_message.push_str("Options: \n");
        help_message.push_str("  --mode <mode>\tSet the gradient mode (rainbow, linear)\n");
        help_message.push_str("\n");
        help_message
        .push_str("  --frequency <#color>\tRainbow gradient's sine-wave frequency (Higher values cause faster change in the pattern)\n");
        help_message.push_str("  --spread <#color>\tRainbow gradient's spread value\n");
        help_message.push_str("\n");
        help_message
            .push_str("  --start-color <#color>\tSet the start color for the linear gradient\n");
        help_message
            .push_str("  --end-color <#color>\tSet the end color for the linear gradient\n");
        help_message.push_str("\n");
        help_message.push_str("\n");
        help_message.push_str("  --help\tDisplay this help message\n");
        help_message.push_str("  --version\tDisplay the version number\n");

        help_message
    }

    pub fn version(&self) -> String {
        format!("v{}", "0.1.0")
    }
}
