# OSEDA CLI

The CLI tool designed to help you scaffold, manage, and deploy your OSEDA projects. It simplifies common development workflows, letting you focus on building your own presentations, ready to deploy to oseda.net.

---

## Installation

To install `oseda-cli`, make sure you have **Rust** and **Cargo** installed. Then, run:

```bash
cargo install oseda-cli
```

---

## Usage

The `oseda` CLI provides several commands to manage your projects.

### Global Options

* `--version`: Prints version information.
* `--help`: Prints help message.

---

### Commands

#### `oseda init` - Initialize a new OSEDA project

Sets up a new OSEDA project in the current directory.

```bash
oseda init
```

**Example:**

```bash
oseda init
# (follow prompts for project setup)
```

**Output:**

```
Successfully initialized oseda project
```

#### `oseda run` - Run your OSEDA project

Executes your OSEDA project.

```bash
oseda run
```

**Example:**

```bash
oseda run
```

**Output:**

```
Successfully ran oseda project
```

#### `oseda check` - Check your OSEDA project for issues

This command performs all checks on your OSEDA project to identify potential issues.
Passing this command is the MVP for getting a presentation approved for deployment.

```bash
oseda check [OPTIONS]
```

Options:
    --port <PORT>: Specifies the port to use for the check. Default value is 3000.

Example:

```bash
oseda check --port 8080
```

Output:

```
Successfully checked oseda project
```


#### `oseda deploy` - Deploy your OSEDA project

This command handles the deployment of your OSEDA project.
You *must* be signed into GitHub for deployment to work.
See deployment instructions for more details.

```bash
oseda deploy [FORK_URL]
```

**FORK_URL:**
URL of your fork of https://github.com/oseda-dev/oseda-lib


**Example:**

```bash
oseda deploy https://github.com/ReeseHatfield/oseda-lib
```

**Output:**

```
Successfully deployed oseda project
See deployment instructions...
```

---

## Error Handling

If any command encounters an error, the CLI prints a descriptive message.

**Example (Init Error):**

```
Could not initialize project with error: DirectoryNameMismatch(Project name does not match directory)
```

---

## Contributing

Feel free to submit a PR to this, or any other repository

---

## License

This project is licensed under the MIT License.

---

## Author

[Reese Hatfield](https://github.com/ReeseHatfield)
