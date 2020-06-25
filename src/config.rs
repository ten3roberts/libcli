//! # Argument parser
//! Parses and generates configuration from supplied arguments and option specifications
//! Can also generate usage strings

use std::collections::HashMap;

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
/// name: The name/trigger for the option, e.g; "recursive" or "clean", an a name of "(unnamed)" specifies the first unnamed arguments before any option is given<br>
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
                        "{} values supplied for option '{}', expected exactly {}",
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

impl std::fmt::Display for OptionSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "    -{}, --{} {}\n        {}\n\n",
            self.abrev,
            self.name,
            if self.required { "[required]" } else { "" },
            self.desc,
        )
    }
}

/// Specifies a configuration of parsed arguments
/// Each option which was given as a spec can be accessed by option(name)
/// This returns a Option<Vec<String>> containing the values of the option (if any)
pub struct Config {
    parsed: HashMap<&'static str, Vec<String>>,
}

/// Parses and generates configuration from supplied arguments and option specifications
/// Can also generate usage strings
impl Config {
    /// Same as Config::new but uses the arguments passed to the program (env::args)
    /// The program path, first argument, is included in the unnamed args<br>
    /// Note, the spec isn't stored with config<br>
    pub fn new_env(specs: &[OptionSpec]) -> Result<Config, String> {
        Config::parse(std::env::args(), specs)
    }
    /// Parses config from custom supplied arguments<br>
    /// Specs is a list containing specifications for the available options a use can supply<br>
    /// Returns Err(msg) if a spec doesn't match what is specified<br>
    /// The arguments before any option are specified with the (unnamed)<br>
    /// The values for the options can be accessed with the option(name) method<br>
    /// Parsing will fail if an option with policy other than Exact(0) is used twice
    /// Note, the spec isn't stored with config<br>
    pub fn new(args: &[&str], specs: &[OptionSpec]) -> Result<Config, String> {
        Config::parse(args.iter().map(|arg| arg.to_string()), specs)
    }

    /// Generates a usage string from supplied specs
    // Through a combination of list_required and list_unrequired you can configure it to only show required options and vice versa
    pub fn generate_usage(
        specs: &[OptionSpec],
        list_required: bool,
        list_unrequired: bool,
    ) -> String {
        let mut required_string = String::new();
        let mut unrequired_string = String::new();
        if list_required {
            required_string = specs
                .iter()
                .filter(|spec| spec.required)
                .map(|spec| spec.to_string())
                .collect();
        }
        if list_unrequired {
            unrequired_string = specs
                .iter()
                .filter(|spec| !spec.required)
                .map(|spec| spec.to_string())
                .collect();
        }

        return required_string + &unrequired_string;
    }

    // Parses config from passed iterator
    fn parse<'a>(
        args: impl Iterator<Item = String>,
        specs: &[OptionSpec],
    ) -> Result<Config, String> {
        // For quickly locating options
        let name_map: HashMap<&str, &OptionSpec> =
            specs.iter().map(|spec| (spec.name, spec)).collect();

        let abrev_map: HashMap<char, &OptionSpec> =
            specs.iter().map(|spec| (spec.abrev, spec)).collect();

        let mut parsed: HashMap<&'static str, Vec<String>> = HashMap::new();

        // Tries to find a spec with an empty name, the unnamed spec
        // If some it will go by that ruling
        // If none, it will accept as many unnamed args as there are
        let mut current_spec: &OptionSpec = match name_map.get("(unnamed)") {
            Some(v) => v,
            None => return Err("No specification for unnamed arguments found".to_string()),
        };

        let mut values = Vec::new();

        for arg in args {
            // New full-name arg

            if arg.starts_with("-") {
                // Collect the last option values
                values = current_spec.enforce(values)?;

                Self::insert_non_duplicate(&mut parsed, current_spec, values)?;

                values = Vec::new();

                // Single full name argument
                if arg.starts_with("--") {
                    current_spec = match name_map.get(&arg[2..]) {
                        Some(spec) => {
                            if let Some(_) = parsed.get(spec.name) {
                                return Err(format!("Duplicate option '{}'", spec.name));
                            }
                            spec
                        }
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

                        if let Some(_) = parsed.get(spec.name) {
                            return Err(format!("Duplicate option '{}'", spec.name));
                        }
                        Self::insert_non_duplicate(&mut parsed, spec, vec![])?;
                    }
                }
                continue;
            }
            values.push(arg);
        }

        // Collect what remains
        values = current_spec.enforce(values)?;

        Self::insert_non_duplicate(&mut parsed, current_spec, values)?;
        // Check all required options where specified or Err
        for required in specs.iter().filter(|spec| spec.required) {
            if let None = parsed.get(required.name) {
                return Err(format!("Missing required option '{}'", required.name));
            }
        }

        Ok(Config { parsed })
    }

    // Checks if option is already present before inserting and return Err
    // If spec required Exact(0) it won't return Err
    fn insert_non_duplicate(
        map: &mut HashMap<&str, Vec<String>>,
        spec: &OptionSpec,
        values: Vec<String>,
    ) -> Result<(), String> {
        match spec.policy {
            OptionPolicy::Exact(0) => (),
            _ => {
                if let Some(_) = map.get(spec.name) {
                    return Err(format!("Duplicate option '{}'", spec.name));
                }
            }
        }

        map.insert(spec.name, values);
        Ok(())
    }

    // Returns the value[s] given to named or unnamed argument
    // Returns None if argument didn't exist
    pub fn option(&self, name: &str) -> Option<&Vec<String>> {
        self.parsed.get(name)
    }
}
