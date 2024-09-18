/// Command-Line Arguments
struct Args {
    /// A vector containing all the file-paths to read
    files: Vec<std::path::PathBuf>,
}

impl Args {
    /// Parse the command-line arguments
    fn parse() -> Self {
        // Get the command line arguments
        let mut args = std::env::args();
        args.next(); // Consume the path to the executable

        // Collect all valid file-paths
        let mut files = Vec::new();
        while let Some(arg) = args.next() {
            let path = std::path::PathBuf::from(arg);
            if std::path::Path::exists(&path) {
                files.push(path);
            }
        }

        // Return self
        Self { files }
    }
}

fn main() {
    // Parse the command line arguments
    let args = Args::parse();
    println!("{:#?}", args.files)
}
