repo := 'asasine/local-apt'
source_file := replace(repo, '/', '_') + ".sources"

# List available just commands
default:
    just --list

# Build and install the binary directly to /usr/lib/apt/methods/github
binstall:
    cargo build --release
    sudo make /usr/lib/apt/methods/github

# Update the package list for a specific package repository.
update:
    sudo apt-get update -o Dir::Etc::sourcelist="sources.list.d/{{source_file}}" -o Dir::Etc::sourceparts="-" -o APT::Get::List-Cleanup="0" -o Debug::Acquire::Worker=true -o Debug::pkgAcquire::Worker=true
