//! # Argument parser
//!
//! Parses arguments from either the command line or supplied list
//! Takes a list of OptionSpecs and returns a Config containing the parsed data

use std::collections::{HashMap, HashSet};

/// Determines how the number of supplied values should match an argument
/// 0 val indicates a switch like argument

pub enum OptionPolicy {
    // The args's supplied values should match exactly
    Exact(usize),
    // The option's supplied values should be at least n
    AtLeast(usize),
    // The option's supplied values should be at most n
    AtMost(usize),
}

/// Specifies an option that can be given in the command line<br>
/// Later supplied to config::Config::new()<br>
/// abrev: The abreviation of name, e.g; 'r' or 'c'
/// name: The name/trigger for the option, e.g; "recursive" or "clean", an empty name specifies the first unnamed args<br>
/// desc: a short description printed with --help<br>
/// required: specifies if the option is required or optional<br>
/// policy: an enum containing the number of values and how they're enforced<br>
/// ## Example
/// ```
/// let spec = cli_utils::config::OptionSpec::new('r', "recursive", "Reads all files recursively in a directory", false,cli_utils::config::OptionPolicy::Exact(0));
/// ```
pub struct OptionSpec {
    abrev: char,
    name: &'static str,
    desc: &'static str,
    required: bool,
    policy: OptionPolicy,
}

impl OptionSpec {
    pub fn new(
        abrev: char,
        name: &'static str,
        desc: &'static str,
        required: bool,
        policy: OptionPolicy,
    ) -> Self {
        Self {
            abrev,
            name,
            desc,
            required,
            policy,
        }
    }

    // Consumes and checks supplied values with the option policy
    // Returns Ok(values) on success
    // Returns Err(reason) on failure
    fn enforce(&self, values: Vec<String>) -> Result<Vec<String>, String> {
        match self.policy {
            OptionPolicy::Exact(n) => {
                if values.len() != n {
                    return Err(format!(
                        "{} values supplied for option '{}', expected at most {}",
                        values.len(),
                        self.name,
                        n,
                    ));
                };
                Ok(values)
            }
            OptionPolicy::AtLeast(n) => {
                if values.len() < n {
                    return Err(format!(
                        "{} values supplied for option '{}', expected at least {}",
                        values.len(),
                        self.name,
                        n,
                    ));
                };
                Ok(values)
            }
            OptionPolicy::AtMost(n) => {
                if values.len() > n {
                    return Err(format!(
                        "{} values supplied for option '{}', expected at most {}",
                        values.len(),
                        self.name,
                        n,
                    ));
                };
                Ok(values)
            }
        }
    }
}

/// Specifies a configuration of parsed arguments
/// Each option which was given as a spec can be accessed by option(name)
/// This returns a Option<Vec<String>> containing the values of the option (if any)
/// # Parsing and creating a config
/// ```
///
/// ```
pub struct Config {
    // The path to the binary args[0]
    binary: String,
    parsed: HashMap<String, Vec<String>>,
}

impl Config {
    // Parses config from passed command line arguments
    pub fn new_env(specs: &[OptionSpec]) -> Result<Config, String> {
        Config::parse(std::env::args(), specs)
    }

    pub fn new(args: &[&str], specs: &[OptionSpec]) -> Result<Config, String> {
        Config::parse(args.iter().map(|arg| arg.to_string()), specs)
    }

    // Parses config from passed iterator
    fn parse<'a>(
        mut args: impl Iterator<Item = String>,
        specs: &[OptionSpec],
    ) -> Result<Config, String> {
        // Consume first argument
        let binary = args
            .next()
            .expect("Unable to retrieve binary location argument");

        // For quickly locating options
        let name_map: HashMap<&str, &OptionSpec> =
            specs.iter().map(|spec| (spec.name, spec)).collect();

        let abrev_map: HashMap<char, &OptionSpec> =
            specs.iter().map(|spec| (spec.abrev, spec)).collect();

        let mut parsed: HashMap<String, Vec<String>> = HashMap::new();

        // Tries to find a spec with an empty name, the unnamed spec
        // If some it will go by that ruling
        // If none, it will accept as many unnamed args as there are
        let mut current_spec: &OptionSpec = match name_map.get("") {
            Some(v) => v,
            None => return Err("No spec for unnamed arguments found".to_string()),
        };

        let mut values = Vec::new();

        for arg in args {
            // New full-name arg

            if arg.starts_with("-") {
                // Collect the last option values
                values = current_spec.enforce(values)?;

                parsed.insert(current_spec.name.to_string(), values);
                values = Vec::new();

                // Single full name argument
                if arg.starts_with("--") {
                    current_spec = match name_map.get(&arg[2..]) {
                        Some(spec) => spec,
                        None => return Err(format!("Invalid option {}", arg)),
                    };
                }
                // One or more abbreviated options
                else {
                    let options: Vec<_> = arg.chars().skip(1).collect();

                    // The values after a group of abbreviated options refer to the last option
                    for (index, option) in options.iter().enumerate() {
                        let spec = match abrev_map.get(&option) {
                            Some(spec) => spec,
                            None => return Err(format!("Invalid abbreviated option '{}'", option)),
                        };

                        // The last option is set to collect the values following
                        if index == options.len() - 1 {
                            current_spec = spec;
                            break;
                        }

                        parsed.insert(spec.name.to_string(), vec![]);
                    }
                }
                continue;
            }
            values.push(arg);
        }

        // Collect what remains
        {
            values = current_spec.enforce(values)?;

            parsed.insert(current_spec.name.to_string(), values);
        }

        // Check all required options where specified or Err
        for required in specs.iter().filter(|spec| spec.required) {
            if let None = parsed.get(required.name) {
                return Err(format!("Missing required option '{}'", required.name));
            }
        }

        Ok(Config {
            binary: binary,
            parsed,
        })
    }

    // Returns the value[s] given to named or unnamed [""] argument
    // Returns None if argument didn't exist
    pub fn option(&self, name: &str) -> Option<&Vec<String>> {
        self.parsed.get(name)
    }

    // Returns the path the program was run from
    pub fn binary(&self) -> &String {
        &self.binary
    }
}
