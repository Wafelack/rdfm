# CHANGELOG

This is the RDFM changelog, all major changes will be depicted here.

# 0.1.0 | 12/02/2021

## Added

- `rdfm setup` to setup a dotfile environment.
- `rdfm add $src $dest` to add a dotfile.
- `rdfm proceed` to proceed all the linking.
- `rdfm remove $pattern` to remove all lines of `~/.dotfiles/dotfiles.rdfm` containing this pattern.
- `rdfm pull` to pull dotfiles from an external git repo.

# 0.1.1 | ?

## Fixed

- Message display in proceed (displaying `nok` in red instead of `ok` in green).