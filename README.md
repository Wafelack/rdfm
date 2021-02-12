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

Welcome to RDFM.

RDFM is the Rusty DotFiles Manager, it permits an efficient and easy dotfiles sharing.

It has only been tested on a GNU system but it should work on every UNIX based OS.

**Dont forget to read the LICENSE file** - RDFM is distributed under the GNU Affero General Public License (AGPL).

The informations about latest changes can be found in the CHANGELOG.md file.

# Installation

- To install RDFM you can either:

1. Run `cargo install rdfm`
2. Download the latest binary from [the releases page](https://github.com/wafelack/rdfm/releases)
3. [Build the project from source](#build-from-source)

# Getting Started

- To setup a dotfiles environment, run `rdfm setup`.
- To get help about RDFM commands, run `rdfm help`.

# Contributing

- There are a few rules that contributors may respect:
  1. Format your code using `cargo fmt`.
  2. Follow the commit rule, [gitmoji.dev](https://gitmoji.dev) or at least [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/).
  3. Use [semantic versioning](https://semver.org)
  4. All submited code has to be working.

# Build from source

- Run `wget https://github.com/upx/upx/archive/$VERSION.tar.gz`.
- Extract it via `tar -xzf $VERSION.tar.gz`.
- `cd` in the produced directory and run `cargo build --release`.
- Add the produced binary somewhere in your $PATH.
