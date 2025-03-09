use crate::client_commands::execute_client_commands;
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

fn start_client(matches: &clap::ArgMatches) -> Result<String, String> {
  let client_port = extract_client_port(matches);
  let target_directory = extract_directory(matches);
  match client_port {
    Ok(valid_port) => {
      execute_client_commands(&valid_port, target_directory);
      Ok(format!("Client script exited"))
    }
    Err(error) => Err(error),
  }
  // cd to client folder - I think we should be able to run this from anywhere within the SLUG
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

fn start_server(matches: &clap::ArgMatches) -> Result<String, String> {
  let server_port = extract_server_port(matches);
  let target_directory = extract_directory(matches);
  match server_port {
    Ok(valid_port) => {
      execute_server_commands(&valid_port, target_directory);
      Ok(format!("Server script exited"))
    }
    Err(error) => Err(error),
  }
}

pub fn start_application(matches: &clap::ArgMatches) {
  let server_started = start_server(matches);
  match server_started {
    Ok(success_message) => {
      let client_started = start_client(matches);
      println!("{success_message}");
      match client_started {
        Ok(success_message) => {
          println!("{success_message}")
        }
        Err(error_message) => {
          println!("{error_message}")
        }
      }
    }
    Err(error_message) => {
      println!("{error_message}")
    }
  }
}
