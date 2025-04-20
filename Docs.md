oseda CLI - Presentation project scaffolding and validation tool

Usage:
  oseda <command> [options]

Commands:
  init           Generate a new oseda project
  run            Start local oseda dev server using Vite
  check          Validate presentation for deployment constraints
  deploy         Uhhhhh interop with user's github and make PR? idk this needs more planning

Global Options:
  -h, --help     Show help <i usaully hardcode thse options but I've seen cool CLIs have these are global options, so would like to do that>
  -v, --version  Show CLI version <semver from github action?>

Command Details:

  init [--presentation-only]
    Sets up a basic oseda project template:
      - Create oseda-config.js and vite.config.js
      - Installs dependencies (node, npm, vite, reveal, etc. if neeeded)

    <This should walk thru the vite cli to create a project, then prompt for config info, create that oseda-config.json file, write some stuff run scripts to the package.json, etc.>

    Options:
      --presentation-only Set up project in presentation only mode, containing only a reveal.js presentation instead of a whole website containing a reveal presentation

      <Should change index.html to only show the reveal prsentation>

  run
    Starts the Vite dev server using current project settings

  check
    Runs validation checks on the project:
      -  Max header image size
      - Minimum and maximum header dimensions
      - Total project size must be under some some?

  deploy
    Yeah idk yet lol
