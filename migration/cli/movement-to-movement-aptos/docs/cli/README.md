# Command-Line Help for `movement-to-movement-aptos`

This document contains the help content for the `movement-to-movement-aptos` command-line program.

**Command Overview:**

* [`movement-to-movement-aptos`↴](#movement-to-movement-aptos)
* [`movement-to-movement-aptos markdown`↴](#movement-to-movement-aptos-markdown)
* [`movement-to-movement-aptos markdown generate`↴](#movement-to-movement-aptos-markdown-generate)
* [`movement-to-movement-aptos markdown file`↴](#movement-to-movement-aptos-markdown-file)
* [`movement-to-movement-aptos markdown print`↴](#movement-to-movement-aptos-markdown-print)
* [`movement-to-movement-aptos markdown workspace`↴](#movement-to-movement-aptos-markdown-workspace)
* [`movement-to-movement-aptos migrate`↴](#movement-to-movement-aptos-migrate)

## `movement-to-movement-aptos`

The `movement-to-aptos` CLI

**Usage:** `movement-to-movement-aptos [COMMAND]`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: migration/cli/movement-to-movement-aptos/src/cli/mod.rs

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `migrate` — Migrate from Movement to MovementAptos



## `movement-to-movement-aptos markdown`

Generates markdown for the CLI

**Usage:** `movement-to-movement-aptos markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `movement-to-movement-aptos markdown generate`

Generate and update the documentation

**Usage:** `movement-to-movement-aptos markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `movement-to-movement-aptos markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `movement-to-movement-aptos markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `movement-to-movement-aptos markdown print`

Print the documentation in the shell

**Usage:** `movement-to-movement-aptos markdown print`



## `movement-to-movement-aptos markdown workspace`

Generate the documentation for the workspace

**Usage:** `movement-to-movement-aptos markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `movement-to-movement-aptos migrate`

Migrate from Movement to MovementAptos

**Usage:** `movement-to-movement-aptos migrate --movement-state-db-path <MOVEMENT_STATE_DB_PATH> --movement-aptos-state-db-path <MOVEMENT_APTOS_STATE_DB_PATH>`

###### **Options:**

* `--movement-state-db-path <MOVEMENT_STATE_DB_PATH>` — The path to the input Movement state database
* `--movement-aptos-state-db-path <MOVEMENT_APTOS_STATE_DB_PATH>` — The path to the output MovementAptos state database



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
