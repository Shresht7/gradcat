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

/// The main entrypoint of the application
fn main() {
    // Parse the command line arguments
    let args = Args::parse();
    // Run the command line application
    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

/// Run the command-line application
fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // If no files were specified in the cli arguments ...
    if args.files.len() == 0 {
        // ...Read from stdin
        let reader = std::io::stdin().lock();
        cat(reader)
    } else {
        // Otherwise, read the specified files
        for filepath in args.files {
            let file = std::fs::File::open(filepath)?;
            let reader = std::io::BufReader::new(file);
            cat(reader)
        }
    };
    Ok(())
}

/// Cat out the contents read
fn cat(reader: impl std::io::BufRead) {
    for line in reader.lines() {
        if let Ok(line) = line {
            print_line(line);
        }
    }
}

/// Style the characters and print-out the line
fn print_line(line: String) {
    for (_, char) in line.chars().enumerate() {
        print!("{}", char)
    }
    print!("\n"); // End the line with a new-line character
}
