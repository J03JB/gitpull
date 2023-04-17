# Repos

Command line tool written in Rust as a learning project.  
Repos is a tool for pulling from a list of repos stored in `~/.repos`

## Usage

```bash
repos -a .              # Adds current directory to list.

repos -a /path/to/repo  # Adds repo to list.

repos -d "some_repo"    # Removes repo from the list.

repos -p "some_repo"    # Pull the specified repo in list.

repos --pull-all        # Pulls from all repos in list.

repos -l                # List repos stored in list.
```
