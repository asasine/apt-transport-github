# apt-transport-github
Download GitHub Release assets directly from APT.

## Quickstart

Note: replace `${owner}` and `${repo}` with the GitHub repository's owner and repo name.

1. Add a new APT source to `/etc/apt/sources.list.d/${owner}_${repo}.sources`:
    ```sources.list(5)
    Types: deb
    URIs: github://${owner}/${repo}
    Suites: ./
    Trusted: yes
    Languages: none
    ```

1. Update packages:
    ```bash
    sudo apt update
    ```

1. List packages in this new repo:
    ```bash
    grep '^Package:' /var/lib/apt/lists/${owner}_${repo}_._Packages
    ```

1. Install:
    ```bash
    sudo apt install ${package}
    ```

## Example

For this repo, I make the file `/etc/apt/sources.list.d/asasine_apt-transport-github.sources`:

```sources.list(5)
Types: deb
URIs: github://asasine/apt-transport-github
Suites: ./
Trusted: yes
Languages: none
```

And list packages with:

```bash
grep '^Package:' /var/lib/apt/lists/asasine_apt-transport-github_._Packages
```

To update this package:

```bash
sudo apt install apt-transport-github
```

## Documentation
See [docs/](./docs/) for documentation on architecture and how to contribute.
