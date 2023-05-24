# Repos

Command line tool written in Rust as a learning project.  
A program to manage and pull from a list of git repositories.

## Usage

```bash
repos -a .              # Adds current directory to list.

repos -a /path/to/repo  # Adds repo to list.

repos -r "some_repo"    # Removes repo from the list.

repos -p "some_repo"    # Pull the specified repo in list.

repos --pull-all        # Pulls from all repos in list.
repos -x                # Pulls from all repos in list.

repos -l                # List repos stored in list.
```
