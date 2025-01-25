# `license-picker` is a simple command-line tool to generate a license file

Supports all the licenses listed in the `licenses/` directory. Each supported
license is included in the compiled binary itself, and defined in source-code,
not from the files in `licenses/`.

## CLI Definition

```
USAGE: license-picker [OPTION]... LICENSE...
Choose one or more LICENSEs for your project.

ARGUMENTS:
  LICENSE...
    License(s) to generate in the current directory. The default file name
    for a single license choice is `LICENSE`. Multiple licenses can be
    chosen, and each will be named to indicate what license it is. For example,
    choosing the MIT and Apache 2.0 licenses will create the files `LICENSE-MIT`
    and `LICENSE-APACHE`.

FLAGS:
  -h, --help          Display this help message and exit
  -V, --version       Display version information and exit
  -p, --print         Prints the license to standard output
  -l, --list          List license options and exit
  -c, --check         Check if the given license specifier(s) is valid

OPTIONS:
  -e, --extension EXT Sets the extension for the output file (default: None)
  -n, --name  NAME    The full name for the license(s) (default: None)
  -y, --year  YEAR    The year for the license(s) (default: None)
  -m, --email EMAIL   The email for the license(s) (default: None)
  -j, --project PROJ  The project for the license(s) (default: None)
  -u, --url URL       The project url for the license(s) (default: None)
```
