use libcli::config;

fn main() {
    let specs = [
        config::OptionSpec::new(
            '\0',
            "(unnamed)",
            "Input files",
            true,
            config::OptionPolicy::AtLeast(2), // 1st value is program name
        ),
        config::OptionSpec::new(
            'o',
            "output",
            "Searches recursive",
            false,
            config::OptionPolicy::Exact(1),
        ),
        config::OptionSpec::new(
            'v',
            "verbose",
            "Shows verbose output",
            false,
            config::OptionPolicy::Exact(0),
        ),
    ];

    let config = config::Config::new_env(&specs).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    // Check if verbose was specified, either as --verbose or -v
    let verbose: bool = match config.option("verbose") {
        Some(_) => true,
        None => false,
    };

    // Should always return Some since option was required, new_env should have failed if not included
    let files = match config.option("(unnamed)") {
        Some(v) => v,
        None => panic!("Didn't get input files"),
    };

    if verbose {
        println!("Reading files {:?}...", files);
    }

    // logic
}
