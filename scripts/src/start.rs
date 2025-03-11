use colored::Colorize;

use crate::server_commands::execute_server_commands;

fn extract_directory(matches: &clap::ArgMatches) -> String {
  let directory = matches
    .get_one::<String>("repo_root")
    .map_or_else(|| ".", |s| s.as_str())
    .to_string();
  directory
}

fn is_valid_port(port: &str) -> bool {
  let is_valid_length = port.len() == 4;
  let is_valid_number = port.parse::<f64>().is_ok();
  return is_valid_length && is_valid_number;
}

fn extract_client_port(matches: &clap::ArgMatches) -> Result<String, String> {
  let port = matches
    .get_one::<String>("client_port")
    .map_or_else(|| "client_port", |s| s.as_str())
    .to_string();
  if !is_valid_port(&port) {
    return Err(format!("Value \"{}\" is not a valid port", port).to_string());
  }
  Ok(port)
}

fn extract_server_port(matches: &clap::ArgMatches) -> Result<String, String> {
  let port = matches
    .get_one::<String>("server_port")
    .map_or_else(|| "server_port", |s| s.as_str())
    .to_string();
  if !is_valid_port(&port) {
    return Err(format!("Value \"{}\" is not a valid port", port).to_string());
  }
  Ok(port)
}

pub fn start_application(matches: &clap::ArgMatches) {
  let server_port = extract_server_port(matches);
  let client_port = extract_client_port(matches);
  let target_directory = extract_directory(matches);
  match server_port {
    Ok(server_port) => match client_port {
      Ok(client_port) => {
        execute_server_commands(&server_port, &client_port, target_directory);
        println!("Server script exited")
      }
      // client error
      Err(error) => println!("{} {}", "Error:".red().bold(), error.red()),
    },
    // server error
    Err(error) => println!("{} {}", "Error:".red().bold(), error.red()),
  }
}
