# Architecture

This [APT transport method](#apt-method-interface) synthesizes a [Debian package archive](https://wiki.debian.org/DebianRepository/Format#Overview) (APT repository) based on metadata contained in a GitHub Release. During an `apt update`, a [`Packages` index file](#packages-index-file) that lists out available binary packages is synthesized, and a later `apt install` or `apt upgrade` can download those packages. The configured source list uses the [flat repository format](https://wiki.debian.org/DebianRepository/Format#Flat_Repository_Format) as opposed to `distribution [component1]` format.

## APT Method Interface

An [APT Method](http://www.fifi.org/doc/libapt-pkg-doc/method.html/ch2.html) is a sub-process of the main APT process. The main process communicates with the method through the unix FD `stdin` to send messages for downloading various files that belong to the package archive. The method communicates back to APT with the `stdout` FD, and uses `stderr` for emergency error reporting.

### Messages

Messages are sent in RFC-822 format. Each is identified by a 3-digit Status Code.

#### 600 URI Acquire

APT will request new URIs through the `600 URI Acquire` message. This includes packages, as specified in the repository indices, and the [package index](https://wiki.debian.org/DebianRepository/Format#A.22Packages.22_Indices) itself.

##### `Packages` index file

APT will request for the method to obtain the `Packages` file from the repository. This file must follow the [Debian Repository format](https://wiki.debian.org/DebianRepository/Format#A.22Packages.22_Indices).

Since this file isn't provided by GitHub Releases automatically, the transport method will synthesize one from information contained in the GitHub Release. This file is then stored in APT's variable state directory and parsed by it to determine packages available to install.

Each stanza in the `Packages` file has a `Filename` key, which this method sets to the download URL of that asset, which allows the method to fetch the asset when requested by APT.

```
https://api.github.com/repos/{owner}/{repo}/releases/assets/{asset_id}
```

The `{asset_id}` is found in the response to various [Releases REST API requests](https://docs.github.com/en/rest/releases/releases?apiVersion=2026-03-10). Each asset that has a `content_type` of `application/x-debian-package` is included.
