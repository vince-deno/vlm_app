use std::{fmt::Error, path::PathBuf, time::Instant};

/// The basic CLI trait.
pub trait VLMCli {
    /// Runs the CLI application.
    fn run(&self) -> Result<(), Error>;
    /// Parses command-line arguments.
    fn parse_args(&self) -> Result<(), Error>;
    /// Displays help or usage information.
    fn help(&self);

    /// Create an instance using a pattern.
    fn pattern(pattern: String) -> Self
    where
        Self: Sized;
    /// Create an instance from a file path.
    fn file(file: PathBuf) -> Self
    where
        Self: Sized;
    /// Set the number of lines (or items) to process.
    fn count(&self, count: usize);
    /// Process a given path.
    fn path(&self, path: PathBuf);
    /// Toggle JSON mode.
    fn json(&self, json: bool);
}


// --- Helper traits with default implementations

pub trait DefaultVLMCli: Default {
    fn run_default(&self) -> Result<(), std::fmt::Error> {
        let start = Instant::now();
        println!("Running CLI application (default)...");
        self.parse_args_default()?;
        println!("Execution time: {:?}", start.elapsed());
        Ok(())
    }
    fn parse_args_default(&self) -> Result<(), std::fmt::Error> {
        println!("Parsing command-line arguments (default)...");
        Ok(())
    }
    fn help_default(&self) {
        println!("Usage: vlmcli [options] <command>");
    }
    fn pattern_default(pattern: String) -> Self
    where
        Self: Sized,
    {
        println!("Applying pattern (default): {}", pattern);
        Self::default()
    }
    fn file_default(file: PathBuf) -> Self
    where
        Self: Sized,
    {
        println!("Processing file (default): {:?}", file);
        Self::default()
    }
    fn count_default(&self, count: usize) -> Self {
        println!("Processing {} items (default)", count);
        Self::default()
    }
    fn path_default(&self, path: PathBuf) -> Self {
        println!("Processing path (default): {:?}", path);
        Self::default()
    }
    fn json_default(&self, json: bool) -> Self {
        if json {
            let json_output = serde_json::json!({
                "status": "active",
                "mode": "json"
            });
            match serde_json::to_string_pretty(&json_output) {
                Ok(json_str) => println!("JSON output (default):\n{}", json_str),
                Err(e) => eprintln!("JSON serialization error: {}", e),
            }
        } else {
            println!("JSON mode: disabled (default)");
        }
        Self::default()
    }
}