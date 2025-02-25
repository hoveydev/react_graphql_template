import os
import sys
import subprocess

# Define ANSI escape codes for colors
GREEN = "\033[32;1m"
RED = "\033[31m"
YELLOW = "\033[33m"
RESET = "\033[0m"

ROOT_DIR = os.getcwd()

server_dir = "_CHANGE_ME_PROJECT_SLUG/server"

if os.path.basename(ROOT_DIR) != "react_graphql_template":
    print(f"\n{RED}Error: This script must be run from the 'react_graphql_template_test' directory.{RESET}")
    exit(1)

print("Starting GraphQL server...")
try:
    subprocess.run(["npm", "run", "start"], cwd=server_dir, text=True, check=True)
except subprocess.CalledProcessError as e:
    print(f"{RED}Error starting server: {e.output.decode()}{RESET}")
except KeyboardInterrupt:
    print(f"\n{YELLOW}Script interrupted. Exiting...{RESET}")
    sys.exit(0)
