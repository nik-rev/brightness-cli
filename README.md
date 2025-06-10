# lumina

A program to read and control device brightness.

Supported:
- Windows
- Linux

# Command-Line Help for `lumina`

This document contains the help content for the `lumina` command-line program.

**Command Overview:**

* [`lumina`↴](#lumina)
* [`lumina completion`↴](#lumina-completion)
* [`lumina set`↴](#lumina-set)
* [`lumina get`↴](#lumina-get)
* [`lumina list`↴](#lumina-list)

## `lumina`

Read and control device brightness

**Usage:** `lumina <COMMAND>`

###### **Subcommands:**

* `completion` — Generate completions for a shell
* `set` — Set brightness of a device
* `get` — Get brightness of a device
* `list` — List each device and its brightness



## `lumina completion`

Generate completions for a shell

**Usage:** `lumina completion <SHELL>`

###### **Arguments:**

* `<SHELL>` — Shell to generation completions for

  Possible values: `bash`, `carapace`, `elvish`, `fig`, `fish`, `nushell`, `powershell`, `zsh`




## `lumina set`

Set brightness of a device

**Usage:** `lumina set [OPTIONS] [ACTION]`

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



## `lumina get`

Get brightness of a device

**Usage:** `lumina get [OPTIONS]`

###### **Options:**

* `--device <DEVICE>` — Get brightness of this device (by default, get the first device)
* `-s`, `--silent` — Silence output
* `--json` — Output JSON



## `lumina list`

List each device and its brightness

**Usage:** `lumina list [OPTIONS]`

###### **Options:**

* `--json` — Output JSON

# Installation

### Shell

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/nik-rev/lumina/releases/latest/download/lumina-installer.sh | sh
```

### Homebrew

```sh
brew install nik-rev/tap/lumina
```

### PowerShell

```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/nik-rev/lumina/releases/latest/download/lumina-installer.ps1 | iex"
```

### Cargo

```sh
cargo install lumina
```

### Nix

```sh
nix profile install github:nik-rev/lumina/main
```
