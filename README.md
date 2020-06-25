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

### Ignoring options
The OptionPolicy provides a variant call Final which will collect all remaining arguments to the values of the option, regardless if there are more options

This can be useful if you want the user to enter a command as a last argument and not have the argument to that command affect yours, e.g;
    `myprogram -vo output.txt --exec grep search -r .`

If `exec` is policy `Final`, -r option won't be parsed as another argument but rather become a value of `exec` 

### Help and Usage
Detecting help option without failing on required option

The `FinalIgnore` OptionPolicy does the same as Finalize but will not Err on missing required options, this is useful for overriding options like `help` or `version`

This enables us to provide the help option without failing to parse due to missing required option

`args::Config::generate_usage(&specs, list_required, list_unrequired)` generates a usage string which can be printed

```
// Add this OptionSpec with the others with FinalizeIgnore policy
args::OptionSpec::new(
            'h',
            "help",
            "Display a help screen",
            false,
            args::OptionPolicy::FinalizeIgnore(),
        ),
```

Parse as normal and check if help was present
Note: When using FinalizeIgnore, required options may return none, which they don't usually because parse return Err

```
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
```

Using this style rather than an auto help feature when parsing is that you can add extra information the argument parses doesn't know or print to something else than console if for example in GUI application

This can also be used for version or similar