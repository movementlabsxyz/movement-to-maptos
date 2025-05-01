# Command-Line Help for `mtma-dev`

This document contains the help content for the `mtma-dev` command-line program.

**Command Overview:**

* [`mtma-dev`↴](#mtma-dev)
* [`mtma-dev markdown`↴](#mtma-dev-markdown)
* [`mtma-dev markdown generate`↴](#mtma-dev-markdown-generate)
* [`mtma-dev markdown file`↴](#mtma-dev-markdown-file)
* [`mtma-dev markdown print`↴](#mtma-dev-markdown-print)
* [`mtma-dev markdown workspace`↴](#mtma-dev-markdown-workspace)
* [`mtma-dev track`↴](#mtma-dev-track)
* [`mtma-dev track dev`↴](#mtma-dev-track-dev)
* [`mtma-dev track dev markdown`↴](#mtma-dev-track-dev-markdown)
* [`mtma-dev track dev markdown generate`↴](#mtma-dev-track-dev-markdown-generate)
* [`mtma-dev track dev markdown file`↴](#mtma-dev-track-dev-markdown-file)
* [`mtma-dev track dev markdown print`↴](#mtma-dev-track-dev-markdown-print)
* [`mtma-dev track dev markdown workspace`↴](#mtma-dev-track-dev-markdown-workspace)
* [`mtma-dev track dev track`↴](#mtma-dev-track-dev-track)
* [`mtma-dev track dev track where`↴](#mtma-dev-track-dev-track-where)
* [`mtma-dev track dev track using`↴](#mtma-dev-track-dev-track-using)
* [`mtma-dev track prod`↴](#mtma-dev-track-prod)
* [`mtma-dev track prod markdown`↴](#mtma-dev-track-prod-markdown)
* [`mtma-dev track prod markdown generate`↴](#mtma-dev-track-prod-markdown-generate)
* [`mtma-dev track prod markdown file`↴](#mtma-dev-track-prod-markdown-file)
* [`mtma-dev track prod markdown print`↴](#mtma-dev-track-prod-markdown-print)
* [`mtma-dev track prod markdown workspace`↴](#mtma-dev-track-prod-markdown-workspace)
* [`mtma-dev track prod track`↴](#mtma-dev-track-prod-track)
* [`mtma-dev track prod track where`↴](#mtma-dev-track-prod-track-where)
* [`mtma-dev track prod track using`↴](#mtma-dev-track-prod-track-using)

## `mtma-dev`

The `movement-to-aptos` CLI

**Usage:** `mtma-dev [COMMAND]`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `track` — Check the state for given migration criteria. Migrate from Movement to MovementAptos. Track the state of the node for real-time comparison



## `mtma-dev markdown`

Generates markdown for the CLI

**Usage:** `mtma-dev markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-dev markdown generate`

Generate and update the documentation

**Usage:** `mtma-dev markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-dev markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-dev markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-dev markdown print`

Print the documentation in the shell

**Usage:** `mtma-dev markdown print`



## `mtma-dev markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-dev markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-dev track`

Check the state for given migration criteria. Migrate from Movement to MovementAptos. Track the state of the node for real-time comparison

**Usage:** `mtma-dev track <COMMAND>`

###### **Subcommands:**

* `dev` — Runs the null migration
* `prod` — Runs the replay migration



## `mtma-dev track dev`

Runs the null migration

**Usage:** `mtma-dev track dev <COMMAND>`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `track` — Track the state of the node



## `mtma-dev track dev markdown`

Generates markdown for the CLI

**Usage:** `mtma-dev track dev markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-dev track dev markdown generate`

Generate and update the documentation

**Usage:** `mtma-dev track dev markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-dev track dev markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-dev track dev markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-dev track dev markdown print`

Print the documentation in the shell

**Usage:** `mtma-dev track dev markdown print`



## `mtma-dev track dev markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-dev track dev markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-dev track dev track`

Track the state of the node

**Usage:** `mtma-dev track dev track <COMMAND>`

###### **Subcommands:**

* `where` — Run track with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run track with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev track dev track where`

Run track with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev track dev track where`



## `mtma-dev track dev track using`

Run track with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev track dev track using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-dev track prod`

Runs the replay migration

**Usage:** `mtma-dev track prod <COMMAND>`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `track` — Track the state of the node



## `mtma-dev track prod markdown`

Generates markdown for the CLI

**Usage:** `mtma-dev track prod markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-dev track prod markdown generate`

Generate and update the documentation

**Usage:** `mtma-dev track prod markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-dev track prod markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-dev track prod markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-dev track prod markdown print`

Print the documentation in the shell

**Usage:** `mtma-dev track prod markdown print`



## `mtma-dev track prod markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-dev track prod markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-dev track prod track`

Track the state of the node

**Usage:** `mtma-dev track prod track <COMMAND>`

###### **Subcommands:**

* `where` — Run track with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run track with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev track prod track where`

Run track with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev track prod track where`



## `mtma-dev track prod track using`

Run track with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev track prod track using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
