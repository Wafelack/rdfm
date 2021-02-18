```
                    d8888b.   d88888b.   8888888b  8888b  d8888
                    88  `8D    88   `8D   88'       88'YbdP`88 
                    88oobY'    88    88   88ooo     88  88  88 
                    88`8b      88    88   88        88      88 
                    88 `88.    88   .8D   88        88      88 
                   d88b  d88  Y88888D'   d88b      d88b    d88b
                        
                            The Rusty DotFiles Manager.
                            Copyleft (ɔ) 2021, Wafelack
```

# Introduction

RDFM is the Rusty DotFiles Manager, based on a homemade linking system
and configured by a simple file ; it is intended for everyone who needs
an easy and reliable way to manage and share dotfiles.

# Roadmap

- [x] Dotfiles adding.
- [x] Dotfiles linking.
- [x] Setup command.
- [x] Dotfiles pulling.
- [x] `XDG_CONFIG_HOME` support.
- [x] Default target name.
- [ ] Reverse linking.


# Installation

- To install RDFM you can either:

1. **Recommended** [Build the project from source](#build-from-source)
2. Run `cargo install rdfm`
3. Run `bruh install rdfm`
4. Download the latest binary from [the releases page](https://github.com/wafelack/rdfm/releases)

# Getting Started

In this guide, the `$FOLDER` variable will refer to your `$XDG_CONFIG_HOME` folder if it is set, or `$HOME` if it is not.

## Setting up rdfm

RDFM can be set up with the command `rdfm setup`, that will create a `$FOLDER/.dotfiles/` directory and a `$FOLDER/.dotfiles/dotfiles.rdfm` file.

## Adding dotfiles

You can add dotfiles to the `$FOLDER/.dotfiles/dotfiles.rdfm` file with the `rdfm add $src $dest` command
that can be used as following:

```bash
$ rdfm add ~/.vim/ vim/.vim
Successfully added `/home/wafelack/.vim/` to dotfiles as `vim/.vim`
$ cat ~/.dotfiles/dotfiles.rdfm
# This file is created by rdfm and is not intended for manual editing.
/home/wafelack/.vim/->/home/wafelack/.dotfiles/vim/.vim
$
```

## Removing dotfiles

To remove dotfiles from `$FOLDER/.dotfiles/dotfiles.rdfm`, you can use the `rdfm remove $pattern` command
that will remove every line containing one or more occurences of `$pattern`.

## Proceeding linking

To proceed linking and copy your dotfiles to the `$FOLDER/.dotfiles` folder, you'll use the `rdfm proceed` command.

## Pulling dotfiles

To pull dotfiles from an external repo, you can use `rdfm pull $repo` as following:

```bash
$ rdfm pull https://github.com/wafelack/dotfiles
Successfully pulled `https://github.com/wafelack/dotfiles` into `~/.dotfiles`
$
```

# Contributing

- There are a few rules that contributors may respect:
  1. Format your code using `cargo fmt`.
  2. Follow the commit rule, [gitmoji.dev](https://gitmoji.dev) or at least [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/).
  3. Use [semantic versioning](https://semver.org)
  4. All submited code has to be working.

# Legal

RDFM is distributed under the GNU Affero General Public License version 3.0 (AGPL-3.0) as described in the [LICENSE](./LICENSE) file.

# Build from source

- Clone the project: `git clone git@github.com:Wafelack/rdfm.git`.
- `cd` in `rdfm/` and run `cargo build --release`.
- Add the produced binary somewhere in your $PATH.
