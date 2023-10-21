# Versioned Files

A tool which updates files in your project with the current version number.

> ⚠️ **CURRENT STATUS** ⚠️
>
> This project is in the early development stage. It is currently not functional or useful.
> Please check back soon.

## Example

Given a file `README.md` which contains:

```text, file(path="README.md")
# Example Project Docs

The current version is `v2.3.16`
```

And a `versioned-files.yml` file which contains:

```yaml, file(path="versioned-files.yml")
locations:
  - type: string-pattern
    file: README.md
    pattern: The current version is `v{{version}}`
```

Running:

```shell, script(expected_exit_code=0)
versioned-files update 2.3.16
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
versioned-files help
```

This will give you all the details you need:

```text, verify()
A tool which updates files in your project with the current version number.

Usage: versioned-files <COMMAND>

Commands:
  update  
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Current Version

To find out what version you are running, you can run:
```shell, script()
versioned-files --version
```

```text, verify()
versioned-files 0.1.0
```

## Configuration

When running the `update` command, a configuration file is required.
By default, `versioned-files` looks for a file named `versioned-files.yml` in the current directory.

### Missing Configuration

If you run `versioned-files update` without a configuration file present like this:

```shell, script(expected_exit_code=1)
rm -f versioned-files.yml # TODO extract section into another doc and remove this
versioned-files update 1.2.3
```

Then you will see the following error:

```text, verify(stream=stderr)
ERROR: No versioned-files.yml file found.
```
