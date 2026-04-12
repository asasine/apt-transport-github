# Contributing

Welcome and thanks for taking an interest on contributing!

Before opening any pull requests, please [open an issue.](https://github.com/asasine/apt-transport-github/issues/new/choose)

## Development commands

Some common commands can be found in the [justfile](../justfile). Others of use:

```bash
# list common dev commands
just

# run unit tests
cargo test

# build and install the binary directly
cargo build --release && sudo make /usr/lib/apt/methods/github
# or
just binstall

# build the debian binary package via its container image
# outputs to target/debian/
./build.sh

# generate man pages
./debial/rules man
# and view them
man -l target/man/apt-transport-github.1
```
