# brightness-cli

A program to read and control device brightness.

Supported:
- Windows
- Linux

# Usage

Increase brightness by 10%, e.g. from 70% to 80%:

```sh
brightness-cli set +10%
```

Decrease brightness by 10%:

```sh
brightness-cli set -10%
```

Set brightness to 40%, e.g. from 80% to 40%:

```sh
brightness-cli set 40%
```


# Command-Line Help for `brightness-cli`

This document contains the help content for the `brightness-cli` command-line program.

**Command Overview:**

* [`brightness-cli`↴](#brightness-cli)
* [`brightness-cli completion`↴](#brightness-cli-completion)
* [`brightness-cli set`↴](#brightness-cli-set)
* [`brightness-cli get`↴](#brightness-cli-get)
* [`brightness-cli list`↴](#brightness-cli-list)

## `brightness-cli`

Read and control device brightness

**Usage:** `brightness-cli <COMMAND>`

###### **Subcommands:**

* `completion` — Generate completions for a shell
* `set` — Set brightness of a device
* `get` — Get brightness of a device
* `list` — List each device and its brightness



## `brightness-cli completion`

Generate completions for a shell

**Usage:** `brightness-cli completion <SHELL>`

###### **Arguments:**

* `<SHELL>` — Shell to generation completions for

  Possible values: `bash`, `carapace`, `elvish`, `fig`, `fish`, `nushell`, `powershell`, `zsh`




## `brightness-cli set`

Set brightness of a device

**Usage:** `brightness-cli set [OPTIONS] [ACTION]`

###### **Arguments:**

* `<ACTION>` — Modify the percentage of brightness

   - Increase: +5%
   - Decrease: -5%
   - Set to specific: 40%
   - max or 100% sets to max brightness
   - min or 0% sets to min brightness

  Default value: `100%`

###### **Options:**

* `--device <DEVICE>` — Set brightness for this device (by default, sets the first device)
* `-s`, `--silent` — Silence output
* `--json` — Output JSON



## `brightness-cli get`

Get brightness of a device

**Usage:** `brightness-cli get [OPTIONS]`

###### **Options:**

* `--device <DEVICE>` — Get brightness of this device (by default, get the first device)
* `-s`, `--silent` — Silence output
* `--json` — Output JSON



## `brightness-cli list`

List each device and its brightness

**Usage:** `brightness-cli list [OPTIONS]`

###### **Options:**

* `--json` — Output JSON

# Installation

### Shell

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/nik-rev/brightness-cli/releases/latest/download/brightness-cli-installer.sh | sh
```

### Homebrew

```sh
brew install nik-rev/tap/brightness-cli
```

### PowerShell

```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/nik-rev/brightness-cli/releases/latest/download/brightness-cli-installer.ps1 | iex"
```

### Cargo

```sh
cargo install brightness-cli
```

### Nix

```sh
nix profile install github:nik-rev/brightness-cli/main
```
