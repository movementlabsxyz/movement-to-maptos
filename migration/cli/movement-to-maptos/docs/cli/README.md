# Command-Line Help for `movement-to-maptos`

This document contains the help content for the `movement-to-maptos` command-line program.

**Command Overview:**

* [`movement-to-maptos`↴](#movement-to-maptos)
* [`movement-to-maptos markdown`↴](#movement-to-maptos-markdown)
* [`movement-to-maptos markdown generate`↴](#movement-to-maptos-markdown-generate)
* [`movement-to-maptos markdown file`↴](#movement-to-maptos-markdown-file)
* [`movement-to-maptos markdown print`↴](#movement-to-maptos-markdown-print)
* [`movement-to-maptos markdown workspace`↴](#movement-to-maptos-markdown-workspace)
* [`movement-to-maptos migrate`↴](#movement-to-maptos-migrate)

## `movement-to-maptos`

The `movement-to-aptos` CLI

**Usage:** `movement-to-maptos [COMMAND]`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: migration/cli/movement-to-maptos/src/cli/mod.rs

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `migrate` — Migrate from Movement to Maptos



## `movement-to-maptos markdown`

Generates markdown for the CLI

**Usage:** `movement-to-maptos markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `movement-to-maptos markdown generate`

Generate and update the documentation

**Usage:** `movement-to-maptos markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `movement-to-maptos markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `movement-to-maptos markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `movement-to-maptos markdown print`

Print the documentation in the shell

**Usage:** `movement-to-maptos markdown print`



## `movement-to-maptos markdown workspace`

Generate the documentation for the workspace

**Usage:** `movement-to-maptos markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `movement-to-maptos migrate`

Migrate from Movement to Maptos

**Usage:** `movement-to-maptos migrate --movement-state-db-path <MOVEMENT_STATE_DB_PATH> --maptos-state-db-path <MAPTOS_STATE_DB_PATH>`

###### **Options:**

* `--movement-state-db-path <MOVEMENT_STATE_DB_PATH>` — The path to the input Movement state database
* `--maptos-state-db-path <MAPTOS_STATE_DB_PATH>` — The path to the output Maptos state database



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
