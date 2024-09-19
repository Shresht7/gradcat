// Library
use crate::colors::RGB;

/// Command-Line Arguments
#[derive(Default)]
pub struct Args {
    /// A vector containing all the file-paths to read
    pub files: Vec<std::path::PathBuf>,
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

        // Collect all valid file-paths
        while let Some(arg) = args.next() {
            let path = std::path::PathBuf::from(arg);
            if std::path::Path::exists(&path) {
                itself.files.push(path);
            }
        }

        // Return self
        itself
    }
}
