# libcli
A library for Rust containing utilities to help develop command line based programs

More features are coming

## Parsing command line arguments
The module `config` contains the ability to parse command line arguments

Allows you to supply an OptionSpec list that specifies
* The name of the option 
* Abbreviated single letter name that can be combined, e.g -rv
* Description that can be used to generate usage messages
* Required or nonrequired option
* Accepted number of values, exactly, at least and at most specified number

Parsing of arguments either from std::env::args or custom list

After creating an OptionSpec list, pass them to either Config::new or Config::new_env

### Examples
```
 let specs = [
        args::OptionSpec::new(
            '\0',
            "(unnamed)",
            "Input files",
            true,
            args::OptionPolicy::AtLeast(2), // 1st value is program name
        ),
        args::OptionSpec::new(
            'o',
            "output",
            "Searches recursive",
            false,
            args::OptionPolicy::Exact(1),
        ),
        args::OptionSpec::new(
            'v',
            "verbose",
            "Shows verbose output",
            false,
            args::OptionPolicy::Exact(0),
        ),
    ];

    let config = args::Config::new_env(&specs).unwrap_or_else(|err| {
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
```