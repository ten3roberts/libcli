#[cfg(test)]
mod tests {
    use libcli::config;

    #[test]
    fn parse_0() {
        let specs = [
            config::OptionSpec::new(
                '\0',
                "(unnamed)",
                "Unnamed arguments",
                true,
                config::OptionPolicy::AtLeast(1),
            ),
            config::OptionSpec::new(
                'r',
                "recursive",
                "Searches recursive",
                false,
                config::OptionPolicy::Exact(0),
            ),
            config::OptionSpec::new(
                'o',
                "output",
                "Specifies output file",
                true,
                config::OptionPolicy::Exact(1),
            ),
            config::OptionSpec::new(
                'v',
                "verbose",
                "Shows verbose output",
                false,
                config::OptionPolicy::Exact(1),
            ),
            config::OptionSpec::new(
                'n',
                "number",
                "The number of iterations to perform",
                false,
                config::OptionPolicy::Exact(1),
            ),
        ];

        let args = [
            "./test",
            "myfile.txt",
            "--output",
            "output.txt",
            "-rvn",
            "3",
        ];

        println!(
            "Usage:\n{}",
            config::Config::generate_usage(&specs, false, true)
        );

        let config = config::Config::new(&args[..], &specs).unwrap_or_else(|err| panic!(err));

        assert_eq!(
            *config
                .option("output")
                .expect("Didn't parse --output option"),
            vec!["output.txt".to_string()],
        );
        assert_eq!(
            *config
                .option("number")
                .expect("Didn't parse --number option"),
            vec!["3".to_string()],
        );
    }

    #[test]
    #[should_panic]
    fn parse_too_few() {
        let specs = [config::OptionSpec::new(
            '\0',
            "(unnamed)",
            "Files to search",
            true,
            config::OptionPolicy::AtLeast(2),
        )];

        let args = ["./test"];

        config::Config::new(&args[..], &specs).unwrap_or_else(|err| panic!(err));
    }

    #[test]
    #[should_panic]
    fn parse_too_many() {
        let specs = [config::OptionSpec::new(
            '\0',
            "(unnamed)",
            "File to search",
            true,
            config::OptionPolicy::AtMost(1),
        )];

        let args = ["./test", "file1", "file2"];
        config::Config::new(&args[..], &specs).unwrap_or_else(|err| panic!(err));
    }

    #[test]
    #[should_panic]
    fn parse_dup() {
        let specs = [
            config::OptionSpec::new(
                '\0',
                "(unnamed)",
                "Unnamed",
                true,
                config::OptionPolicy::AtLeast(1),
            ),
            config::OptionSpec::new(
                'o',
                "output",
                "Specifies the output file",
                true,
                config::OptionPolicy::Exact(1),
            ),
        ];

        let args = ["./test", "-o", "file1", "-o", "file2"];
        config::Config::new(&args[..], &specs).unwrap_or_else(|err| panic!(err));
    }

    #[test]
    fn parse_dup_switch() {
        let specs = [
            config::OptionSpec::new(
                '\0',
                "(unnamed)",
                "Unnamed",
                true,
                config::OptionPolicy::AtLeast(1),
            ),
            config::OptionSpec::new(
                'o',
                "output",
                "Specifies the output file",
                true,
                config::OptionPolicy::Exact(1),
            ),
            config::OptionSpec::new(
                'v',
                "verbose",
                "Show verbose output",
                true,
                config::OptionPolicy::Exact(0),
            ),
        ];

        let args = ["./test", "-vo", "file1", "-v"];
        config::Config::new(&args[..], &specs).unwrap_or_else(|err| panic!(err));
    }
}
