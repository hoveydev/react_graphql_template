import os
import re
import sys
import subprocess

# Define ANSI escape codes for colors
GREEN = "\033[32;1m"
RED = "\033[31m"
YELLOW = "\033[33m"
BOLD = "\033[1m"
RESET = "\033[0m"

ROOT_DIR = os.getcwd()
SERVER_DIR = os.getcwd()

def validate_root_dir():
  if os.path.basename(ROOT_DIR) != "react_graphql_template":
    print(f"\n{RED}Error: This script must be run from the 'react_graphql_template_test' directory.{RESET}")
    exit(1)

def is_using_gql() -> bool:
  while True:
    user_input = input(f"Will this project be utilizing a GQL server? ({GREEN}y{RESET}/n)")
    if not user_input:
      return True
    elif not re.match(r"^[yn]$", user_input):
      print(f"{RED}Error: Value can only be 'y' or 'n'! Please try again.{RESET}")
    else:
      return user_input == 'y'

# Find and collect placeholders
def find_placeholders(include_gql):
  print("Searching repository for placeholders...")
  placeholders = set()
  # Search file contents
  for root, _, files in os.walk(ROOT_DIR):
    for file in files:
      if file == ".DS_Store":
        continue
      with open(os.path.join(root, file), "r", encoding="utf-8", errors="ignore") as f:
        content = f.read()
        matches = re.findall(r"_CHANGE_ME_[A-Z]+(?:_[A-Z]+)*", content)
        placeholders.update(matches)
  # Search file/directory names
  for root, dirs, files in os.walk(ROOT_DIR):
    for name in dirs + files:
      if ".DS_Store" in name:
        continue
      matches = re.findall(r"_CHANGE_ME_[A-Z]+(?:_[A-Z]+)*", name)
      placeholders.update(matches)
  # Sort placeholders by length (descending) to avoid substring conflicts
  return sorted(placeholders, key=len, reverse=True)

def main():
  validate_root_dir()
  find_placeholders(is_using_gql)
  # continue above ^

# install server dependencies
def intall_server_dependencies():
    print("Installing server dependencies...")
    server_dir = "_CHANGE_ME_PROJECT_SLUG/server"
    try:
        subprocess.run(["npm", "install"], cwd=server_dir, capture_output=True, text=True)
        print(f"✅ {GREEN}Successfully installed server dependencies!{RESET}")
    except subprocess.CalledProcessError as e:
        print(f"{RED}Error installing server dependencies: {e.output.decode()}{RESET}")

### Eventually will neeed one for client ^^^
client_dir = "_CHANGE_ME_PROJECT_SLUG/client"

def format_placeholder(placeholder):
    readable = placeholder.replace("_CHANGE_ME_", "").replace("_", " ")
    return f"{GREEN}{readable}{RESET}"

def get_user_input(placeholder):
    while True:
        human_readable = format_placeholder(placeholder)
        user_input = input(f"Enter the value for {human_readable}: ")
        if not user_input:
            print(f"{RED}Error: Value cannot be empty! Please try again.{RESET}")
        elif not re.match(r"^[a-zA-Z]+$", user_input):
            print(f"{RED}Error: Value can only contain letters! Please try again.{RESET}")
        else:
          return user_input

# Prompt for user input (same as before)
def replace_placeholders():
    replacements = {}
    sorted_placeholders = find_placeholders()
    if len(sorted_placeholders) > 0:
        print(f"✅ {GREEN}Successfully found placeholders!{RESET}")
        for placeholder in sorted_placeholders:
            try:
                replacements[placeholder] = get_user_input(placeholder)
            except KeyboardInterrupt:
                print(f"\n{YELLOW}Script interrupted. Exiting...{RESET}")
                sys.exit(0)

        print("Replacing variables...")
        print(f"\n{GREEN}Please wait!{RESET} Exiting now could have unexpected results.")

        # Replace content in files
        for root, _, files in os.walk(ROOT_DIR):
            for file in files:
                path = os.path.join(root, file)
                with open(path, "r+", encoding="utf-8", errors="ignore") as f:
                    content = f.read()
                    for placeholder in sorted_placeholders:
                        content = content.replace(placeholder, replacements[placeholder])
                    f.seek(0)
                    f.write(content)
                    f.truncate()

        # Rename files and directories
        for root, dirs, files in os.walk(ROOT_DIR, topdown=False):
            for name in dirs + files:
                old_path = os.path.join(root, name)
                new_name = name
                for placeholder in sorted_placeholders:
                    new_name = new_name.replace(placeholder, replacements[placeholder])
                new_path = os.path.join(root, new_name)
                if old_path != new_path:
                    os.rename(old_path, new_path)
    else:
        print(f"{RED}No placeholders were found in this repository.{RESET}")
        print(f"Please make sure you are in the correct directory and all placeholders contain the prefix: {YELLOW}_CHANGE_ME_{RESET}")

    # Start Server
    print("Starting GraphQL server...")
    try:
        server = subprocess.run(["npm", "run", "build"], cwd=server_dir, capture_output=True, text=True)
        print(f"✅ {GREEN}Successfully started server!{RESET}")
        print(server.stdout)
    except subprocess.CalledProcessError as e:
        print(f"{RED}Error starting server: {e.output.decode()}{RESET}")

    print("✅ All replacements completed successfully!")

    print("Building and starting server...")
    subprocess.Popen(["python3", "./scripts/start-server.py"])
    # need similar code for client

if __name__ == "__main__":
  main()
