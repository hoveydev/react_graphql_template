use std::{
  collections::HashMap,
  env::{self, current_dir},
  fs::{self, DirEntry},
  io::{BufRead, BufReader},
  path::PathBuf,
  process::{self, Child, Command, Stdio},
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  thread,
};

use colored::Colorize;
use dotenvy::Result;
use terminal_link::Link;

fn print_stdout(child: &mut Child, running_clone: Arc<AtomicBool>, port: &Arc<String>) {
  let stdout = child.stdout.take().unwrap_or_else(|| {
    eprintln!("{}", "Error: Could not get stdout from process.".red());
    process::exit(1)
  });
  let stdout_reader = BufReader::new(stdout);

  for line in stdout_reader.lines() {
    if let Ok(line) = line {
      match line {
        ln if ln.contains("Server listening at:") => {
          let localhost_with_port = format!(
            "{}{}{}",
            "http://localhost:".bold().underline(),
            port.bold().underline(),
            "/".bold().underline()
          );
          let server_link = Link::new("", &localhost_with_port);
          println!(
            "{} Server app started at {}",
            "[GraphQL]".blue(),
            server_link
          );
        }
        ln if ln.contains("restarting due to changes") => {
          println!("{} Server restarting due to changes", "[GraphQL]".blue())
        }
        // below will only be needed for debugging until all basic scenarios are covered
        // ln => {
        //   println!("{} {}", "[GraphQL]".blue(), ln.blue())
        // },
        _ => continue,
      }
    }

    // Check the running flag
    if !running_clone.load(Ordering::SeqCst) {
      println!("Stopping GraphQL...");
      break;
    }
  }
}

fn print_stderr(child: &mut Child) {
  let stderr = child.stderr.take().unwrap_or_else(|| {
    eprintln!("{}", "Error: Could not get stderr from process.".red());
    process::exit(1)
  });
  let stderr_reader = BufReader::new(stderr);
  for line in stderr_reader.lines() {
    if let Ok(line) = line {
      if line.contains("Error:") {
        eprintln!("{} {}", "[GraphQL Error]".red(), line.red());
      }
      if line.contains("app crashed") {
        eprintln!("{} {}", "[GraphQL Error]".red(), line.red());
      }
    }
  }
}

fn update_env_file(port: &str) -> Result<()> {
  let env_vars: Vec<(String, String)> = dotenvy::from_filename_iter(".env")
    .unwrap_or_else(|_| {
      eprintln!("Could not find .env file");
      process::exit(1)
    })
    .map(|result| {
      result.map_err(|_| {
        eprintln!("Error parsing .env");
        process::exit(1)
      })
    })
    .collect::<Result<Vec<_>>>()
    .unwrap_or_else(|_| {
      eprintln!("Could not collect all entries");
      process::exit(1)
    });

  let mut env_map: HashMap<String, String> = env_vars.into_iter().collect();
  env_map.insert("PORT".to_string(), port.to_string());

  let mut new_env_content = String::new();
  for (key, value) in env_map {
    new_env_content.push_str(&format!("{}={}\n", key, value));
  }

  fs::write(".env", new_env_content).unwrap_or_else(|_| {
    eprintln!("Could not write new content to file.");
    process::exit(1)
  });

  Ok(())
}

fn run_threaded_graphql(port: &Arc<String>, entry: DirEntry) {
  let target_dir = "server";
  if entry.file_name() == target_dir && entry.path().is_dir() {
    let port_clone = Arc::clone(&port);
    println!("Found match: {entry:?}");
    env::set_current_dir(entry.path()).unwrap_or_else(|_| {
      eprintln!("{}", "Error: Could not change directory.".red());
      process::exit(1)
    });
    update_env_file(&port_clone).unwrap_or_else(|_| {
      eprintln!("Could not update .env");
      process::exit(1)
    });

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    // Spawn GraphQL in a separate thread
    let graphql_handle = thread::spawn(move || {
      let mut child = Command::new("npm")
        .arg("run")
        .arg("dev")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| {
          eprintln!(
            "{} {}",
            "Error:".red().bold(),
            "Failed to run GraphQL server.".red()
          );
          process::exit(1)
        });

      print_stdout(&mut child, running_clone, &port_clone);
      print_stderr(&mut child);
    });

    // First executed line
    println!("{}", "Starting GraphQL...".green());

    graphql_handle.join().unwrap_or_else(|_| {
      eprintln!(
        "{} {}",
        "Error:".red().bold(),
        "GraphQL thread panicked.".red()
      );
      process::exit(1)
    });
  }
}

pub fn execute_server_commands(port: &str, path_arg: String) {
  let port = Arc::new(port.to_string());

  if let Ok(current_dir) = current_dir() {
    let path_from_string = PathBuf::from(path_arg);
    let selected_dir = current_dir.join(path_from_string);
    println!(
      "Selected directory: {}",
      selected_dir.to_string_lossy().bold()
    );

    if let Ok(read_selected_dir) = selected_dir.read_dir() {
      for entry in read_selected_dir {
        if let Ok(entry) = entry {
          run_threaded_graphql(&port, entry);
        }
      }
    } else {
      let invalid_dir_error = format!(
        "{} {}{}{}",
        "Error:".red().bold(),
        "Directory \"".red(),
        selected_dir.to_string_lossy().red(),
        "\" does not exist.".red(),
      );
      println!("{invalid_dir_error}")
    }
  }
  let no_dir_change_error = format!(
    "{} {}",
    "Server Error:".red().bold(),
    "Please see above".red()
  );
  println!("{no_dir_change_error}");
}
