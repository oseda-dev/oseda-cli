# Command-Line Help for `oseda`

This document contains the help content for the `oseda` command-line program.

**Command Overview:**

* [`oseda`↴](#oseda)
* [`oseda init`↴](#oseda-init)
* [`oseda run`↴](#oseda-run)
* [`oseda check`↴](#oseda-check)
* [`oseda deploy`↴](#oseda-deploy)
* [`oseda fork`↴](#oseda-fork)
* [`oseda export`↴](#oseda-export)
* [`oseda import`↴](#oseda-import)

## `oseda`

oseda project scaffolding CLI

**Usage:** `oseda <COMMAND>`

###### **Subcommands:**

* `init` — Initialize a new Oseda project in the working directory
* `run` — Run the Oseda project in the working directory
* `check` — Check the Oseda project in the working directory for common errors
* `deploy` — Deploy your Oseda project to github to add to oseda.net
* `fork` — Fork the library repository to submit your course
* `export` — Export the Oseda project to a PDF file This will install the npm package `decktape` This relies on a chromium backend, as a result, it may take a while to run
* `import` — Import a PDF presentation and convert it into an Oseda project This is highly experimental and relies of generative AI `import` relies on several dependencies such as [TODO]



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



## `oseda export`

Export the Oseda project to a PDF file This will install the npm package `decktape` This relies on a chromium backend, as a result, it may take a while to run

**Usage:** `oseda export [OPTIONS]`

###### **Options:**

* `--output <OUTPUT>` — String name of the output PDF file

  Default value: `slides.pdf`
* `--port <PORT>` — Port the project runs on

  Default value: `3000`



## `oseda import`

Import a PDF presentation and convert it into an Oseda project This is highly experimental and relies of generative AI `import` relies on several dependencies such as [TODO]

**Usage:** `oseda import [OPTIONS] --input <INPUT>`

###### **Options:**

* `--input <INPUT>` — name of PDF to import
* `--output <OUTPUT>` — String name of the output PDF file

  Default value: `slides.pdf`
* `--port <PORT>` — Port the Docling Client runs on

  Default value: `8000`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
