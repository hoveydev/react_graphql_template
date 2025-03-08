use colored::Colorize;
use std::{
  env::{self, current_dir},
  fs::DirEntry,
  io::{BufRead, BufReader},
  path::PathBuf,
  process::{self, Child, Command, Stdio},
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  thread,
};
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
        ln if ln.contains("Local:") => {
          let localhost_with_port = format!(
            "{}{}{}",
            "http://localhost:".bold().underline(),
            port.bold().underline(),
            "/".bold().underline()
          );
          let client_link = Link::new("", &localhost_with_port);
          println!("{} Client app started at {}", "[Vite]".blue(), client_link);
        }
        ln if ln.contains("hmr update") => println!("{} updated", "[Vite]".blue()),
        // below will only be needed for debugging until all basic scenarios are covered
        // println!("{} {}", "[Vite]".blue(), line.blue());
        _ => continue,
      }
    }

    // Check the running flag
    if !running_clone.load(Ordering::SeqCst) {
      println!("Stopping Vite...");
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
      if line.contains("CACError:") {
        eprintln!("{} {}", "[Vite Error]".red(), line.red());
      }
    }
  }
}

fn run_threaded_vite(port: &Arc<String>, entry: DirEntry) {
  let target_dir = "client";
  if entry.file_name() == target_dir && entry.path().is_dir() {
    let port_clone = Arc::clone(&port);
    println!("Found match: {entry:?}");
    env::set_current_dir(entry.path()).unwrap_or_else(|_| {
      eprintln!("{}", "Error: Could not change directory.".red());
      process::exit(1)
    });

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    // Spawn Vite in a separate thread
    let vite_handle = thread::spawn(move || {
      let mut child = Command::new("npx")
        .arg("vite")
        .arg("--port")
        .arg(&*port_clone)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| {
          eprintln!("{}", "Error: Failed to run Vite.".red());
          process::exit(1)
        });

      print_stdout(&mut child, running_clone, &port_clone);
      print_stderr(&mut child);
    });

    // First executed line
    println!("{}", "Starting Vite...".green());

    vite_handle.join().unwrap_or_else(|_| {
      eprintln!("{}", "Error: Vite thread panicked.".red());
      process::exit(1)
    });
  }
}

pub fn execute_client_commands(port: &str, path_arg: String) {
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
          run_threaded_vite(&port, entry);
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
    "Error:".red(),
    "Could not change directories".red()
  );
  println!("{no_dir_change_error}");
}
