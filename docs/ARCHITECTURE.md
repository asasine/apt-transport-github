# Architecture

## APT Method Interface

http://www.fifi.org/doc/libapt-pkg-doc/method.html/ch2.html

All APT methods operate as a sub-process of the main APT process. The main process communicates with the method process through unix FDs stdin, stdout, and stderr.

### Messages

Messages are sent in RFC-822 format. Each is identified by a 3-digit Status Code.
