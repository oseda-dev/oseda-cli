# OSEDA CLI

The CLI tool designed to help you scaffold, manage, and deploy your OSEDA projects. It simplifies common development workflows, letting you focus on building your own presentations, ready to deploy to [oseda.net](https://oseda.net).

---

## Installation

To install `oseda-cli`, make sure you have **Cargo** installed. Then, run:

`bash
cargo install oseda-cli
`

---

## Usage

The `oseda` CLI provides several commands to manage your projects.

### Global Options

- `--version`: Prints version information
- `--help`: Prints help message

---

### Commands

#### `oseda init` — Initialize a new OSEDA project

Sets up a new OSEDA project in the current directory with a basic Reveal.js setup, Vite config, and markdown-based slide deck.

`bash
oseda init
`

**Example:**

`bash
oseda init
# (follow prompts for project setup)
`

**Output:**

`
Successfully initialized oseda project
`

---

#### `oseda run` — Run your OSEDA project locally

Builds and s your OSEDA presentation using Vite and Serve.

`bash
oseda run
`

**Example:**

`bash
oseda run
`

**Output:**

`
Successfully ran oseda project
`

---

#### `oseda check` — Check your OSEDA project for issues

Performs a series of checks to validate your project before deployment. This includes verifying your config file, git credentials, directory structure, and whether the presentation returns a valid response locally.

Passing this check is the minimum bar for deployment approval by a moderator

`bash
oseda check [OPTIONS]
`

**Options:**

- `--port <PORT>`: Specifies the port to check. Default is `3000`.

**Example:**

`bash
oseda check --port 8080
`

**Output:**

`
Successfully checked oseda project
`

---

#### `oseda deploy` — Deploy your OSEDA project

Clones your fork of the deployment repo, places your project inside the `courses/` directory, and pushes changes to your GitHub fork. You do not need to manage this direcotory and this is automatically cleaned up

You *must* be signed into GitHub and have `user.name` and `user.email` configured for this to work.

`bash
oseda deploy [FORK_URL]
`

**FORK_URL:**

The URL of your fork of https://github.com/oseda-dev/oseda-lib

**Example:**

`bash
oseda deploy https://github.com/ReeseHatfield/oseda-lib
`

**Output:**

`
Successfully deployed oseda project
See deployment instructions...
`

---

## Project Structure

OSEDA projects are structured like this:

```
oseda-project/
├── oseda-config.json     # project metadata
├── slides/               # markdown files
├── src/                  # reveal.js entrypoint
├── css/                  # custom styling
├── index.html            # reveal.js HTML wrapper
├── vite.config.js        # Vite build setup
└── package.json          # npm dependencies
```

Your `oseda-config.json` must match the project folder name and contain correct git author info to pass checks.

---

## Requirements

- Linux/macOS (uses `lsof`, `kill`, `serve`, etc.)
- [Node.js + npm](https://nodejs.org/)
- Git (with `user.name` and `user.email` configured)
- Internet access for `npm` and `git` commands

---

## Error Handling

If any command encounters an error, the CLI prints a descriptive message and exits with code 1.

**Example (Init Error):**

`
Could not initialize project with error: DirectoryNameMismatch("Project name does not match directory")
`

---

## Contributing

Feel free to submit a PR to this, or any other Oseda repository.

---

## License

This project is licensed under the MIT License.

---

## Author

[Reese Hatfield](https://github.com/ReeseHatfield)
