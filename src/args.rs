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
    /// The starting offset value to shift the gradient's starting position
    pub offset: f32,

    /// Linear gradient's starting color
    pub start_color: RGB<u8>,
    /// Linear gradient's ending color
    pub end_color: RGB<u8>,

    /// Disable ANSI colors
    pub no_color: bool,

    pub show_help: bool,
    pub show_version: bool,
}

impl Args {
    /// Initialize the struct with default values
    fn default() -> Self {
        Self {
            start_color: RGB(255, 0, 0),
            end_color: RGB(0, 0, 255),
            offset: 15.0,
            frequency: 1.0,
            spread: 15.0,
            ..Default::default()
        }
    }

    /// Parse the command-line arguments
    pub fn parse() -> Self {
        // Get the command line arguments
        let mut args = std::env::args().peekable();
        args.next(); // Consume the path to the executable

        let mut itself = Args::default();
        let mut options = HashMap::new();

        // Iterate over the arguments and parse them
        while let Some(arg) = args.next() {
            // If this starts with - or --, then this is a flag
            if arg.starts_with("--") || arg.starts_with("-") {
                let key = arg.trim_start_matches("-").to_string();

                // Peek forward one argument to determine the kind of flag...
                if let Some(value) = args.peek() {
                    // ... If the next argument is a flag too, then treat the current one as a boolean
                    if value.starts_with("-") {
                        options.insert(key, String::from("true"));
                    } else {
                        // ... otherwise, this argument is the value for the current flag.
                        options.insert(key, String::from(value));
                        args.next(); // Consume the value as it was recorded
                    }
                } else {
                    // If there is no next argument, then treat the current one as a boolean
                    options.insert(key, String::from("true"));
                }
                continue;
            }

            // ... otherwise, this is a positional argument
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

        if let Some(offset) = options.get("offset") {
            itself.offset = offset.parse().expect("Invalid offset value");
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

        if options.contains_key("no-color") || std::env::var("NO_COLOR").is_ok() {
            itself.no_color = true;
        }

        if options.contains_key("help") || options.contains_key("h") {
            itself.show_help = true;
        }

        if options.contains_key("version") || options.contains_key("v") {
            itself.show_version = true;
        }

        // Return self
        itself
    }

    pub fn help_message(&self) -> String {
        let name = env!("CARGO_PKG_NAME");
        let description = env!("CARGO_PKG_DESCRIPTION");

        let help_message = format!(
            r#"
Usage: {name} [FILES...] [OPTIONS]

{description}

Options:
    --mode <mode>           Set the gradient mode (rainbow, linear)

    --frequency <value>     Rainbow gradient's sine-wave frequency (Higher values cause faster change in the pattern)
    --spread <value>        Rainbow gradient's spread value

    --start-color <color>   Set the start color for the linear gradient
    --end-color <color>     Set the end color for the linear gradient

    --no-color              Disable ANSI colors

    --help, -h              Display the help message
    --version, -v           Display the version number

Examples:
    • {name} ./README.md ./src/main.rs      # Read from the given files
    • ls | {name}                           # Read from STDIN
    • {name} --mode linear --help           # Try out the various modes on this help message! 
        "#
        );

        help_message
    }

    pub fn version(&self) -> String {
        format!("v{}", env!("CARGO_PKG_VERSION"))
    }
}
