<p align="center">
    <img src="assets/concom.png" height=400 width="auto" alt="concom banner"/>
</p>

# concom

> Interactive CLI for [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/), with [Gitmoji](https://gitmoji.dev/) support.

concom *(**con**ventional **com**mits)* is a Rust-based interactive CLI wizard for generating & committing conventional commit messages.

This tool uses up-to-date data sets directly from the [Gitmoji](https://github.com/carloscuesta/gitmoji/releases) and [Commitizen](https://github.com/commitizen/conventional-commit-types/releases) projects.

> [!TIP]
> After installing, [alias it](#git-alias) to `git cc` for a better experience.

## Demo

![](assets/concom.gif)

## Installation

### 🐧 Linux / 🍎 MacOS

Install prebuilt binaries via shell script:

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/agarleanu/concom/releases/latest/download/concom-installer.sh | sh
```

### 🪟 Windows

Install prebuilt binaries via powershell script:

```pwsh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/agarleanu/concom/releases/latest/download/concom-installer.ps1 | iex"
```

### npm


Run the CLI directly with:

```sh
npx @concom/cli
```

## Usage

The prebuilt binaries can be run directly by invoking `concom` in your shell.

```sh
concom
```

### Git Alias

You can set up a `git cc` alias to invoke `concom` directly from git.

```sh
git config --global alias.cc '!concom'

git cc # will run concom
```

Once set, you can run `git cc` from any repository instead of `concom`.

### Updating

Update to the latest version by invoking the following:

```sh
concom-update
```
