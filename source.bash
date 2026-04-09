#!/bin/bash
alias binstall='cargo build --release && sudo make /usr/lib/apt/methods/github'
alias update='sudo apt-get update -o Dir::Etc::sourcelist="sources.list.d/github_asasine_local-apt.sources" -o Dir::Etc::sourceparts="-" -o APT::Get::List-Cleanup="0" -o Debug::Acquire::Worker=true -o Debug::pkgAcquire::Worker=true'
alias reinstall='sudo apt-get install --reinstall local-apt -o Dir::Etc::sourcelist="sources.list.d/github_asasine_local-apt.sources" -o Dir::Etc::sourceparts="-" -o APT::Get::List-Cleanup="0" -o Debug::Acquire::Worker=true -o Debug::pkgAcquire::Worker=true'
