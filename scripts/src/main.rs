use clap::{Arg, Command};
mod setup;
use setup::setup_project;

// these functions should probably move
fn extract_client_port(matches: &clap::ArgMatches) {
  let sub_command = matches
    .get_one::<String>("client_port")
    .map_or_else(|| "client_port", |s| s.as_str())
    .to_string();
  println!("{sub_command}")
}

fn extract_server_port(matches: &clap::ArgMatches) {
  let sub_command = matches
    .get_one::<String>("server_port")
    .map_or_else(|| "server_port", |s| s.as_str())
    .to_string();
  println!("{sub_command}")
}
// see above

fn main() {
  let matches = cli().get_matches();
  match matches.subcommand() {
    Some(("start", sub_matches)) => {
      extract_client_port(sub_matches);
      extract_server_port(sub_matches);
    }
    Some(("setup", sub_matches)) => {
      setup_project(sub_matches);
    }
    _ => unreachable!(),
  }
}

pub fn cli() -> Command {
  Command::new("rgql")
    .about("A utility for setting up React-GraphQL projects")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
      Command::new("setup")
        .about("Setup a brand new project with boilerplate code")
        .arg(project_name()),
    )
    .subcommand(
      Command::new("start")
        .about("Start the project locally")
        .arg(destination_port_client())
        .arg(destination_port_server()),
    )
}

fn project_name() -> Arg {
  Arg::new("repo_root")
    .required(false)
    .default_value(".")
    .help("the name of your new project")
}

fn destination_port_client() -> Arg {
  Arg::new("client_port")
    .required(false)
    .default_value("3000")
    .hide_default_value(false)
    .help("The port your client app will run on")
    .long("client-port")
}

fn destination_port_server() -> Arg {
  Arg::new("server_port")
    .required(false)
    .default_value("8080")
    .hide_default_value(false)
    .help("The port your server app will run on")
    .long("server-port")
}
