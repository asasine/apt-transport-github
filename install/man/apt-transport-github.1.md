% APT-TRANSPORT-GITHUB(1) apt-transport-github | APT Transport Methods
% Adam Sasine
% March 2026

# NAME

apt-transport-github - APT transport for downloading packages from GitHub releases

# SYNOPSIS

DEB822 format:

```
Types: deb
URIs: github://owner/repo
Suites: suite
```

One-line format:

```
deb github://owner/repo suite
```

# DESCRIPTION

**apt-transport-github** is an APT transport method that allows APT to fetch
Debian packages directly from GitHub Release assets. It translates standard APT
repository requests into GitHub Releases API calls, enabling users to manage
packages hosted on GitHub with familiar APT workflows.

When APT encounters a sources list entry using the **github://** scheme, this
transport resolves the requested packages against assets attached to GitHub
releases in the specified repository.

# SOURCES LIST FORMAT

The transport supports both the one-line and DEB822 **sources.list**(5) formats.

DEB822 format:

```
Types: deb
URIs: github://owner/repo
Suites: suite
```

One-line format:

```
deb github://owner/repo suite
```

*owner*
:   The GitHub user or organization that owns the repository.

*repo*
:   The name of the GitHub repository containing the releases.

*suite*
:   The suite/distribution name (e.g., **stable**, **bookworm**).

# EXAMPLES

Install packages from a public GitHub repository (deb822 format):

```
Types: deb
URIs: github://exampleuser/mypackages
Suites: stable
```

The same in one-line format:

```
deb github://exampleuser/mypackages stable
```

# SEE ALSO

**sources.list**(5)

# BUGS

Report bugs at **https://github.com/asasine/apt-transport-github/issues**.
