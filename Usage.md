# Command-Line Help for `oseda`

This document contains the help content for the `oseda` command-line program.

**Command Overview:**

* [`oseda`↴](#oseda)
* [`oseda init`↴](#oseda-init)
* [`oseda run`↴](#oseda-run)
* [`oseda check`↴](#oseda-check)
* [`oseda deploy`↴](#oseda-deploy)
* [`oseda fork`↴](#oseda-fork)

## `oseda`

oseda project scafolding CLI

**Usage:** `oseda <COMMAND>`

###### **Subcommands:**

* `init` — Initialize a new Oseda project in the working directory
* `run` — Run the Oseda project in the working directory
* `check` — Check the Oseda project in the working directory for common errors
* `deploy` — Deploy your Oseda project to github to add to oseda.net
* `fork` — Fork the library repository to submit your course



## `oseda init`

Initialize a new Oseda project in the working directory

**Usage:** `oseda init [OPTIONS]`

###### **Options:**

* `--title <TITLE>`
* `--tags <TAGS>`
* `--color <COLOR>`
* `--template <TEMPLATE>`



## `oseda run`

Run the Oseda project in the working directory

**Usage:** `oseda run`



## `oseda check`

Check the Oseda project in the working directory for common errors

**Usage:** `oseda check [OPTIONS]`

###### **Options:**

* `--port <PORT>` — Port to check for the Oseda project on This is only useful if you have changed the default port that Oseda projects run on my default (3000)

  Default value: `3000`



## `oseda deploy`

Deploy your Oseda project to github to add to oseda.net

**Usage:** `oseda deploy <FORK_URL>`

###### **Arguments:**

* `<FORK_URL>`



## `oseda fork`

Fork the library repository to submit your course

**Usage:** `oseda fork`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
