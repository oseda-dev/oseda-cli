<p align="center">
  <a href="https://github.com/oseda-dev/oseda-cli">
    <img src="https://github.com/oseda-dev/oseda-core/blob/main/frontend/public/OsedaLogoDark.png?raw=true" alt="Oseda Logo" width="350" height="370">
  </a>
</p>

<h3 align="center"><strong>Oseda-CLI</strong></h3>

<p align="center">
  CLI for <a href="https://oseda.net">oseda.net</a>
  <br>
</p>

The CLI tool designed to help you scaffold, manage, and deploy your OSEDA projects. It simplifies common development workflows, letting you focus on building your own presentations, ready to deploy to [oseda.net](https://oseda.net).

---

## Installation

To install `oseda-cli`, make sure you have **Cargo** and **NPM** installed.

### Prefered installation

```bash
curl -sL https://raw.githubusercontent.com/oseda-dev/oseda-cli/refs/heads/main/scripts/curl-install.sh | $SHELL
```

### Install from crates.io
```bash
cargo install oseda-cli
```
Then, make sure `~/.cargo/bin"` is in your path

#### **Zsh**
```
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
```
#### **Bash**
```
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
```
#### **Fish**
```
fish_add_path $HOME/.cargo/bin
```

---



---

## Usage

The `oseda` CLI provides several commands to manage your projects.
See our usage guide for more in depth usage.

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
