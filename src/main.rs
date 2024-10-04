// Modules
mod args;
mod colors;

// Library
use args::Args;
use colors::RGBColor;

/// ANSI Escape code to reset the styles
const ANSI_RESET: &str = "\x1b[0m";

/// The main entrypoint of the application
fn main() {
    // Parse the command line arguments
    let args = Args::parse();
    // Instantiate the application
    let app = App::from(args);
    // Run the command line application
    if let Err(e) = app.run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

struct App {
    args: Args,
}

impl App {
    /// Instantiate the application from the command-line arguments
    fn from(args: Args) -> Self {
        Self { args }
    }

    /// Run the command-line application
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.args.show_help {
            self.cat(std::io::Cursor::new(self.args.help_message()));
            return Ok(());
        }

        if self.args.show_version {
            self.print_line(self.args.version(), 0);
            return Ok(());
        }

        // If no files were specified in the cli arguments ...
        if self.args.files.len() == 0 {
            // ...Read from stdin
            let reader = std::io::stdin().lock();
            self.cat(reader)
        } else {
            // Otherwise, read the specified files
            for filepath in &self.args.files {
                let file = std::fs::File::open(filepath)?;
                let reader = std::io::BufReader::new(file);
                self.cat(reader)
            }
        };
        Ok(())
    }

    /// Cat out the contents read
    fn cat(&self, reader: impl std::io::BufRead) {
        for (idx, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                self.print_line(line, idx);
            }
        }
        // Reset ANSI style at the end
        if !self.args.no_color {
            print!("{}", ANSI_RESET);
        }
    }

    /// Style the characters and print-out the line
    fn print_line(&self, line: String, idx: usize) {
        // If the `NO_COLOR` flag or environment variable is set, then skip applying ANSI colors
        if self.args.no_color {
            println!("{}", line);
            return;
        }

        let length = line.chars().count();
        for (i, char) in line.chars().enumerate() {
            //  Determine the color to style the character
            let color = match self.args.mode {
                colors::GradientMode::Rainbow => colors::rainbow(
                    self.args.offset,
                    self.args.frequency,
                    self.args.spread,
                    i as f32 + idx as f32,
                ),
                colors::GradientMode::Linear => colors::interpolate_linear_gradient(
                    &self.args.start_color,
                    &self.args.end_color,
                    (i as f32) / (length - 1) as f32,
                ),
            };

            print!("{}{}", color.ansi_code(), char)
        }

        print!("\n"); // End the line with a new-line character
    }
}
