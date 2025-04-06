use colored::Colorize;
use dialoguer::Input;
use regex::Regex;
use std::{
  env::{self, current_dir},
  fs::{self},
  path::{Path, PathBuf},
  process::{self, Command},
};
use walkdir::WalkDir;

fn find_variables_and_rename(project_slug: String) {
  println!("{}", "Found Project Slug!".green());
  create_new_git_root();
  let mut new_project_slug_name: Option<&String> = None;
  let variable_regex = Regex::new(r"\b_CHANGE_ME_\w+\b").unwrap();
  let file_content_matches = find_var_matches(&project_slug, &variable_regex);
  let (file_matches, dir_matches) = find_file_and_dir_matches(&project_slug);
  if file_content_matches.is_empty() && file_matches.is_empty() && dir_matches.is_empty() {
    println!(
      "{}{}",
      "Error: ".red().bold(),
      "No variables found.\nThis project may not be set up accurately for this script.\nPlease check all variables start with `_CHANGE_ME_`".red()
    );
    return;
  }
  println!(
    "{}",
    "Variables found. Please rename the following:".green()
  );
  let mut rename_map = std::collections::HashMap::new();
  for (_, var_name) in file_content_matches
    .iter()
    .chain(file_matches.iter())
    .chain(dir_matches.iter())
  {
    if !rename_map.contains_key(var_name) {
      let new_name = prompt_for_new_name(var_name);
      rename_map.insert(var_name.clone(), new_name);
    }
  }
  for (file_path, var_name) in file_content_matches {
    if let Some(new_name) = rename_map.get(&var_name) {
      rename_variable(&file_path, &var_name, new_name);
    }
  }
  for (file_path, var_name) in file_matches {
    if let Some(new_name) = rename_map.get(&var_name) {
      let new_path = file_path.replace(&var_name, new_name);
      match fs::rename(&file_path, &new_path) {
        Ok(()) => println!("√ Renamed file {var_name} to {new_name}"),
        Err(_) => eprintln!("Failed to rename file: {var_name} to {new_name}"),
      }
    }
  }
  let mut sorted_dir_matches = dir_matches;
  // below sorts by path length (deepest first)
  sorted_dir_matches.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
  for (dir_path, var_name) in sorted_dir_matches {
    if let Some(new_name) = rename_map.get(&var_name) {
      let new_path = dir_path.replace(&var_name, new_name);
      match fs::rename(&dir_path, &new_path) {
        Ok(()) => {
          if var_name == "CHANGE_ME_PROJECT_SLUG" {
            new_project_slug_name = Some(new_name);
          }
          println!("√ Renamed directory {var_name} to {new_name}")
        }
        Err(_) => eprintln!("Failed to rename directory {var_name} to {new_name}"),
      }
    }
  }
  println!("Renaming completed!");

  // install dependencies for both client and server
  install_dependencies(new_project_slug_name);
  generate_types(new_project_slug_name); // need to change because of project rename
}

fn create_new_git_root() {
  println!("Updating git root...");
  let mut child_one = Command::new("mv");
  child_one.arg(".git");
  child_one.arg("_CHANGE_ME_PROJECT_SLUG/.git");
  child_one.output().unwrap_or_else(|_| {
    eprintln!(
      "{} {}",
      "Error:".red().bold(),
      "Failed to move .git directory".red()
    );
    process::exit(1)
  });
  let mut child_two = Command::new("git");
  child_two.arg("config");
  child_two.arg("--local");
  child_two.arg("child.worktree");
  child_two.arg("../../");
  child_two.output().unwrap_or_else(|_| {
    eprintln!(
      "{} {}",
      "Error:".red().bold(),
      "Failed to reconfigure git".red()
    );
    process::exit(1)
  });

  println!("{}", "New git root created".green())
}

fn install_dependencies(new_project_slug: Option<&String>) {
  let new_project_slug = new_project_slug.unwrap_or_else(|| {
    eprintln!(
      "{} {}",
      "Error:".red().bold(),
      "Failed to get new project slug name".red()
    );
    process::exit(1)
  });
  println!("Installing client dependencies...");
  let cur_dir = current_dir().unwrap();
  env::set_current_dir(cur_dir.join(new_project_slug.to_string() + "/client")).unwrap();
  let mut child = Command::new("npm");
  child.arg("install");
  child.output().unwrap_or_else(|_| {
    eprintln!(
      "{} {}",
      "Error:".red().bold(),
      "Failed to install client dependencies".red()
    );
    process::exit(1)
  });

  println!("Installing sevrer dependencies...");
  env::set_current_dir(cur_dir.join(new_project_slug.to_string() + "/server")).unwrap();
  let mut child = Command::new("npm");
  child.arg("install");
  child.output().unwrap_or_else(|_| {
    eprintln!(
      "{} {}",
      "Error:".red().bold(),
      "Failed to install server dependencies".red()
    );
    process::exit(1)
  });
}

fn generate_types(new_project_slug: Option<&String>) {
  let new_project_slug = new_project_slug.unwrap_or_else(|| {
    eprintln!(
      "{} {}",
      "Error:".red().bold(),
      "Failed to get new project slug name".red()
    );
    process::exit(1)
  });
  let cur_dir = current_dir().unwrap();
  env::set_current_dir(cur_dir.join(new_project_slug.to_string() + "/server")).unwrap();
  let mut child = Command::new("npm");
  child.arg("run").arg("generate");
  child.output().unwrap_or_else(|_| {
    eprintln!(
      "{} {}",
      "Error:".red().bold(),
      "Failed to generate types".red()
    );
    process::exit(1)
  });
}

pub fn setup_project(matches: &clap::ArgMatches) {
  // do we want to install dependencies as part of this as well? - probably
  let sub_command = matches
    .get_one::<String>("repo_root")
    .map_or_else(|| ".", |s| s.as_str())
    .to_string();
  match find_project_slug_dir(sub_command) {
    Ok(path) => {
      find_variables_and_rename(path);
    }
    Err(e) => eprintln!("{e}"),
  }
}

fn find_project_slug_dir(path_arg: String) -> Result<String, String> {
  if let Ok(current_dir) = current_dir() {
    let path_from_string = PathBuf::from(path_arg);
    let selected_dir = current_dir.join(path_from_string);
    println!(
      "Selected directory: {}",
      selected_dir.to_string_lossy().bold()
    );
    let target_dir = "_CHANGE_ME_PROJECT_SLUG";

    if let Ok(read_selected_dir) = selected_dir.read_dir() {
      for entry in read_selected_dir {
        if let Ok(entry) = entry {
          if entry.file_name() == target_dir && entry.path().is_dir() {
            return Ok(entry.path().to_string_lossy().to_string());
          }
        }
      }
    } else {
      return Err(format!(
        "{}{}{}{}",
        "Error: ".red().bold(),
        "Directory \"".red(),
        selected_dir.to_string_lossy().red(),
        "\" does not exist.".red(),
      ));
    }
    Err(format!(
      "{}{}{}{}",
      "Error: ".red().bold(),
      "Could not find a valid project directory to execute setup.\nPlease make sure ".red(),
      "_CHANGE_ME_PROJECT_SLUG".red().bold(),
      " exists in specified directory.".red()
    ))
  } else {
    Err(format!(
      "{}{}",
      "Error: ".red().bold(),
      "Could not determine current directory.".red()
    ))
  }
}

fn find_var_matches(repo_path: &str, regex: &Regex) -> Vec<(String, String)> {
  println!("{}", "Searching for variables in files...".green().bold());
  let mut matches = Vec::new();
  let excluded_substrings = vec![
    "/scripts/",
    "/scripts/target",
    "/old_scripts/",
    "/node_modules/",
  ];
  let repo_path_buf = PathBuf::from(repo_path);
  for entry in WalkDir::new(repo_path)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|entry| {
      let full_path = entry.path();
      let relative_path = full_path.strip_prefix(&repo_path_buf).unwrap_or(full_path);
      let relative_path_str = relative_path.to_string_lossy();
      !excluded_substrings
        .iter()
        .any(|substring| relative_path_str.contains(substring))
    })
  {
    if entry.file_type().is_file() {
      let file_path = entry.path().to_string_lossy().to_string();
      // let stripped_file_path = file_path.strip_prefix(&repo_path).expect("No path found");
      // println!("Searching: {stripped_file_path}"); // uncomment for debugging
      if let Ok(content) = fs::read_to_string(&file_path) {
        for capture in regex.captures_iter(&content) {
          if let Some(matched_var) = capture.get(0) {
            matches.push((file_path.clone(), matched_var.as_str().to_string()));
          }
        }
      }
    }
  }
  matches
}

fn find_file_and_dir_matches(repo_path: &str) -> (Vec<(String, String)>, Vec<(String, String)>) {
  println!(
    "{}",
    "Searching for variables in file and directory names..."
      .green()
      .bold()
  );
  let mut file_matches = Vec::new();
  let mut dir_matches = Vec::new();
  let prefix = "_CHANGE_ME_";
  let excluded_substrings = vec![
    "/scripts/",
    "/scripts/target",
    "/old_scripts/",
    "/node_modules/",
  ];
  let repo_path_buf = PathBuf::from(repo_path);
  for entry in WalkDir::new(repo_path)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|entry| {
      let full_path = entry.path();
      let relative_path = full_path.strip_prefix(&repo_path_buf).unwrap_or(full_path);
      let relative_path_str = relative_path.to_string_lossy();
      !excluded_substrings
        .iter()
        .any(|substring| relative_path_str.contains(substring))
    })
  {
    let path = entry.path().to_string_lossy().to_string();
    let name = entry.file_name().to_string_lossy().to_string();
    // println!("Searching: {name}"); // uncomment for debugging
    if name.contains(prefix) {
      if entry.file_type().is_file() {
        file_matches.push((path, name));
      } else if entry.file_type().is_dir() {
        dir_matches.push((path, name));
      }
    }
  }
  (file_matches, dir_matches)
}

fn prompt_for_new_name(old_name: &str) -> String {
  let stripped_name = old_name.trim_start_matches("_CHANGE_ME_");
  let name_no_extension = Path::new(stripped_name)
    .file_stem()
    .and_then(|os_str| os_str.to_str())
    .unwrap_or(stripped_name);
  let formatted_old_name = name_no_extension.replace("_", " ");
  Input::new()
    .with_prompt(format!("{}", formatted_old_name.blue().bold()))
    .interact_text()
    .unwrap()
}

fn rename_variable(file_path: &str, old_name: &str, new_name: &str) {
  if let Ok(content) = fs::read_to_string(file_path) {
    let new_content = content.replace(old_name, new_name);
    match fs::rename(&file_path, &new_content) {
      Ok(()) => println!("√ Renamed variable {old_name} to {new_name}"),
      Err(_) => eprintln!("Failed to rename variable {old_name} to {new_name}"),
    }
  }
}
