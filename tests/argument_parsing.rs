#[cfg(test)]
mod tests {
    use cli_utils::config;

    #[test]
    fn parse_0() {
        let specs = [
            config::OptionSpec::new(
                '\0',
                "",
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
                false,
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

        let args = ["./test myfile.txt", "--output", "output.txt", "-rvn", "3"];

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
}
