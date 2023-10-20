# Bump Version

A tool which updates files in your project with the current version number.

## Example

Given a file `README.md` which contains:

```text, file(path="README.md")
# Example Project Docs

The current version is `v2.3.16`
```

And a `bump-version.yml` file which contains:

```yaml, file(path="bump-version.yml")
locations:
  - type: string-pattern
    file: README.md
    pattern: The current version is `v{{version}}`
```

Running:

```shell, script()
bump-version bump 2.3.16
```

Will update `README.md` so that:

```shell, script()
cat README.md
```

Will output:

```text, verify()
# Example Project Docs

The current version is `v2.3.16`
```

## Getting Help

For help on available commands you can run:

```shell, script()
bump-version help
```

This will give you all the details you need:

```text, verify()
A tool which updates files in your project with the current version number.

Usage: bump-version <COMMAND>

Commands:
  bump  
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Current Version

To find out what version you are running, you can run:
```shell, script()
bump-version --version
```

```text, verify()
bump-version 0.1.0
```
