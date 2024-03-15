use clap::{App, Arg, SubCommand};

/// Parses the command-line arguments using Clap.
/// Sets up the structure of the command-line interface, including subcommands and their options.
/// 
/// Returns:
/// - A clap::ArgMatches instance containing the parsed arguments.
pub fn parse_arguments() -> clap::ArgMatches {
    App::new("Manifest Checker")
        .version("0.1.0")
        .author("Usman Mehmood <usmanmehmood55@gmail.com>")
        .about("Verifies files in a directory against a checksum manifest or generates a manifest from a directory")
        // Require at least one subcommand to be used
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("verify")
            .about("Verifies the directory against a manifest")
            .arg(Arg::with_name("manifest")
                .short('m')
                .long("manifest")
                .value_name("FILE")
                .help("Sets the path to the manifest file")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("directory")
                .short('d')
                .long("directory")
                .value_name("DIR")
                .help("Sets the input directory path")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name("generate")
            .about("Generates a manifest from the directory")
            .arg(Arg::with_name("directory")
                .short('d')
                .long("directory")
                .value_name("DIR")
                .help("Sets the input directory path")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Sets the output manifest file path")
                .takes_value(true)
                .required(true)))
        .get_matches()
}
