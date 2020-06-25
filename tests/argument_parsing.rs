#[cfg(test)]
mod tests {
    use libcli::args;

    #[test]
    fn parse_0() {
        let specs = [
            args::OptionSpec::new(
                '\0',
                "(unnamed)",
                "Unnamed arguments",
                true,
                args::OptionPolicy::AtLeast(1),
            ),
            args::OptionSpec::new(
                'r',
                "recursive",
                "Searches recursive",
                false,
                args::OptionPolicy::Exact(0),
            ),
            args::OptionSpec::new(
                'o',
                "output",
                "Specifies output file",
                true,
                args::OptionPolicy::Exact(1),
            ),
            args::OptionSpec::new(
                'v',
                "verbose",
                "Shows verbose output",
                false,
                args::OptionPolicy::Exact(1),
            ),
            args::OptionSpec::new(
                'n',
                "number",
                "The number of iterations to perform",
                false,
                args::OptionPolicy::Exact(1),
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
            args::Config::generate_usage(&specs, false, true)
        );

        let config = args::Config::new(&args[..], &specs).unwrap_or_else(|err| panic!(err));

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
        let specs = [args::OptionSpec::new(
            '\0',
            "(unnamed)",
            "Files to search",
            true,
            args::OptionPolicy::AtLeast(2),
        )];

        let args = ["./test"];

        args::Config::new(&args[..], &specs).unwrap_or_else(|err| panic!(err));
    }

    #[test]
    #[should_panic]
    fn parse_too_many() {
        let specs = [args::OptionSpec::new(
            '\0',
            "(unnamed)",
            "File to search",
            true,
            args::OptionPolicy::AtMost(1),
        )];

        let args = ["./test", "file1", "file2"];
        args::Config::new(&args[..], &specs).unwrap_or_else(|err| panic!(err));
    }

    #[test]
    #[should_panic]
    fn parse_dup() {
        let specs = [
            args::OptionSpec::new(
                '\0',
                "(unnamed)",
                "Unnamed",
                true,
                args::OptionPolicy::AtLeast(1),
            ),
            args::OptionSpec::new(
                'o',
                "output",
                "Specifies the output file",
                true,
                args::OptionPolicy::Exact(1),
            ),
        ];

        let args = ["./test", "-o", "file1", "-o", "file2"];
        args::Config::new(&args[..], &specs).unwrap_or_else(|err| panic!(err));
    }

    #[test]
    fn parse_dup_switch() {
        let specs = [
            args::OptionSpec::new(
                '\0',
                "(unnamed)",
                "Unnamed",
                true,
                args::OptionPolicy::AtLeast(1),
            ),
            args::OptionSpec::new(
                'o',
                "output",
                "Specifies the output file",
                true,
                args::OptionPolicy::Exact(1),
            ),
            args::OptionSpec::new(
                'v',
                "verbose",
                "Show verbose output",
                true,
                args::OptionPolicy::Exact(0),
            ),
        ];

        let args = ["./test", "-vo", "file1", "-v"];
        args::Config::new(&args[..], &specs).unwrap_or_else(|err| panic!(err));
    }
}
