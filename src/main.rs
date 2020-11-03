use std::io::{self};

use clap::{App, Arg};

use init_project::{consts, suggestions};

/// Get arguments
pub fn args() -> clap::ArgMatches<'static> {
    let res = App::new("init_project")
        .version(consts::VERSION.unwrap_or("Uknown"))
        .author(consts::AUTHOR)
        .about(consts::ABOUT)
        .arg(
            Arg::with_name("generate")
                .short("g")
                .long("generate")
                .takes_value(false)
                .required(true)
                .help("Initialize available projects"),
        )
        .get_matches();
    res
}

fn main() -> io::Result<()> {
    let _args = args();

    let project = suggestions::Project::new();

    project.display().choose().generate_project();

    Ok(())
}
