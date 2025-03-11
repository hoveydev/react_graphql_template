use clap::{Arg, Command};
mod setup;
use setup::setup_project;
mod start;
use start::start_application;
mod client_commands;
mod server_commands;

fn main() {
  let matches = cli().get_matches();
  match matches.subcommand() {
    Some(("start", sub_matches)) => {
      start_application(sub_matches);
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
        .arg(project_root()),
    )
    .subcommand(
      Command::new("start")
        .about("Start the project locally")
        .arg(project_root())
        .arg(destination_port_client())
        .arg(destination_port_server()),
    )
}

fn project_root() -> Arg {
  Arg::new("repo_root")
    .required(false)
    .default_value(".")
    .help("the directory in which you would like to execute the command")
}

fn destination_port_client() -> Arg {
  Arg::new("client_port")
    .required(false)
    .default_value("3000")
    .hide_default_value(false)
    .help("The port your client app will run on")
    .long("client-port")
    .short('c')
}

fn destination_port_server() -> Arg {
  Arg::new("server_port")
    .required(false)
    .default_value("4000")
    .hide_default_value(false)
    .help("The port your server app will run on")
    .long("server-port")
    .short('s')
}
