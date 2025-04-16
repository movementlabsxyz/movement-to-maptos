# Command-Line Help for `mtma-executor-util`

This document contains the help content for the `mtma-executor-util` command-line program.

**Command Overview:**

* [`mtma-executor-util`↴](#mtma-executor-util)
* [`mtma-executor-util markdown`↴](#mtma-executor-util-markdown)
* [`mtma-executor-util markdown generate`↴](#mtma-executor-util-markdown-generate)
* [`mtma-executor-util markdown file`↴](#mtma-executor-util-markdown-file)
* [`mtma-executor-util markdown print`↴](#mtma-executor-util-markdown-print)
* [`mtma-executor-util markdown workspace`↴](#mtma-executor-util-markdown-workspace)

## `mtma-executor-util`

The `mtma-executor-util` CLI

**Usage:** `mtma-executor-util [COMMAND]`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI



## `mtma-executor-util markdown`

Generates markdown for the CLI

**Usage:** `mtma-executor-util markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-executor-util markdown generate`

Generate and update the documentation

**Usage:** `mtma-executor-util markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-executor-util markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-executor-util markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-executor-util markdown print`

Print the documentation in the shell

**Usage:** `mtma-executor-util markdown print`



## `mtma-executor-util markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-executor-util markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
