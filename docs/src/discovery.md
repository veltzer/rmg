# Project Discovery

RMG discovers git repositories by searching for directories that contain a `.git` subdirectory.

## Discovery modes

There are three ways RMG finds projects, checked in this order:

### 1. Explicit folders (`--folders`)

When `--folders` is provided, only those directories are considered. Non-git directories are silently skipped.

```bash
rmg --folders /path/to/repoA,/path/to/repoB status
```

### 2. No-glob mode (`--no-glob`)

When `--no-glob` is set, RMG scans immediate subdirectories of the current directory:

```bash
rmg --no-glob list-projects
```

### 3. Glob-based discovery (default)

By default, RMG uses the glob pattern `*/*` to find projects two levels deep (e.g., `org/repo`). If no projects are found with `*/*`, it automatically falls back to `*` to handle the common case where immediate subdirectories are git repos.

```bash
# Works from ~/git/veltzer (repos are at */*)
cd ~/git
rmg list-projects

# Also works from ~/git/veltzer (repos are at *)
cd ~/git/veltzer
rmg list-projects
```

A custom glob can be provided:

```bash
rmg --glob "python-*" list-projects
rmg --glob "org/team-*" list-projects
```

## Sorting

By default, discovered projects are sorted alphabetically. Use `--no-sort` to preserve the filesystem discovery order.

## How it works

1. Collect candidate paths using the selected mode
2. Filter to directories that contain `.git/`
3. Sort alphabetically (unless `--no-sort`)
4. Pass the list to the selected subcommand's runner
