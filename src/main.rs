use libcli::args;

fn main() {
    let specs = [
        args::OptionSpec::new(
            '\0',
            "(unnamed)",
            "Input files",
            true,
            args::OptionPolicy::AtLeast(1), // 1st value is program name
        ),
        args::OptionSpec::new(
            'o',
            "output",
            "Searches recursive",
            true,
            args::OptionPolicy::Exact(1),
        ),
        args::OptionSpec::new(
            'v',
            "verbose",
            "Shows verbose output",
            false,
            args::OptionPolicy::Exact(0),
        ),
        args::OptionSpec::new(
            'h',
            "help",
            "Display a help screen",
            false,
            args::OptionPolicy::FinalizeIgnore(),
        ),
    ];

    let config = args::Config::new_env(&specs).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    if let Some(_) = config.option("help") {
        println!(
            "Myprogram\n{}",
            args::Config::generate_usage(&specs, true, true)
        );
        return;
    }

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
