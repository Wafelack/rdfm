RDFM
====

RDFM is a rusty dotfiles manager.

Installation
------------

### From source

You will need to have cargo, make and man installed.

```
$ git clone https://github.com/wafelack/rdfm.git
$ cd rdfm/
$ chmod +x ./configure
$ ./configure
$ make
# make install
```

### From package manager

`$ cargo install rdfm`


Configuration
-------------

* Run `rdfm setup` to setup the dotfiles environment.
* Edit `~/.config/.dotfiles/dotfiles.rdfm` and add dotfiles, with the following syntax rule:

```
/path/to/dotfile = relative/path/in/dotfiles/dir
```

TIP: You can add comments by starting the line with a `#`.

Usage
-----

### Setup the dotfiles environment

* `rdfm setup`

### Link dotfiles into the dotfiles folder

* `rdfm link`

### Install dotfiles from the dotfiles folder to their original location

* `rdfm install`
