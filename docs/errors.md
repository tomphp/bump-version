# A Catalogue of Errors

## Update Command

### Unknown Location Type

### String Pattern
```yaml, file(path="versioned-files.yml")
locations:
  - !this-type-does-not-exist
    foo: bar
```
```shell, script(expected_exit_code=1)
versioned-files update 1.2.3
```
```shell, verify(stream=stderr)
ERROR: Invalid config file schema: locations: unknown variant `this-type-does-not-exist`, expected `string-pattern` at line 2 column 3
```

#### Location file does not exist

```yaml, file(path="versioned-files.yml")
locations:
  - !string-pattern
    file: this-file-does-not-exist.txt
    pattern: The current version is `v{{version}}`
```
```shell, script(expected_exit_code=1)
versioned-files update 1.2.3
```
```shell, verify()
Updating this-file-does-not-exist.txt...failed
ERROR: Failed to update this-file-does-not-exist.txt: File does not exist
```

#### Pattern does not contain `{{version}}` placeholder

```text, file(path="file.txt")
This is the context of file.txt
```
```yaml, file(path="versioned-files.yml")
locations:
  - !string-pattern
    file: file.txt
    pattern: This pattern does not contain a placeholder
```
```shell, script(expected_exit_code=1)
versioned-files update 1.2.3
```
```shell, verify()
Updating file.txt...failed
ERROR: String pattern "This pattern does not contain a placeholder" does not contain the required {{version}} placeholder
```
