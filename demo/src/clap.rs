use clap::Command;
use clap::{arg, Arg};

// Clap sub command syntax defintions
pub fn syntax() -> Command {
    // strip out usage
    const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";
    // strip out name/version
    const APPLET_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    Command::new("xxx")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("Command")
        .subcommand_help_heading("Commands")
        .help_template(PARSER_TEMPLATE)
        .subcommand(
            Command::new("quit")
                .visible_aliases(["exit", "q"])
                .about("Quit demo")
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("dir")
                .about("Directory list of current directory")
                .arg(arg!([filter]))
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("dark")
                .about("Set register value")
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(Command::new("light").help_template(APPLET_TEMPLATE))
        .subcommand(
            Command::new("about")
                .about("explanation of commands")
                .arg(arg!([topic]))
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("cd")
                .about("change current dir")
                .arg(Arg::new("directory").required(true))
                .arg_required_else_help(true)
                .help_template(APPLET_TEMPLATE),
        )
}
