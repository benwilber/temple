# `temple`

A commandline program that renders template files with structured context inputs.  It is most often used to transform JSON data from a web API to a presentation format such as HTML.  It's a basic, yet flexible tool that can be extended with [Lua](https://www.lua.org/) to perform any kind of advanced templating task from the commandline or a shell pipeline.

In addition to JSON, `temple` can also read context inputs as YAML, simple `KEY=value`, as well as from the environment.  It is sometimes employed as a powerful alternative to tools like `envsubst`, which are used to render configuration files from parameters given as environment variables in 12-factor/Heroku-ish type environments.

Templates are rendered using [minijinja](https://github.com/mitsuhiko/minijinja), a port of the [Jinja2](https://jinja2docs.readthedocs.io/en/stable/) Python templating library for Rust.  `temple` supports any of the templating features that `minijinja` supports.  Visit those docs for more comprehensive examples and documentation about the templating language.

`temple` supports loading custom template filters from Lua scripts.  See the examples in [Lua scripting](#lua-scripting) for more information.


# Basic examples
```sh
$ curl -s https://sunspot.io/time.json | temple -F json datetime.html
```
#### Context input - `time.json`

```json
{
  "datetime": "2021-09-28T19:58:25+0000"
}
```

#### Template - `datetime.html`

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <title>Date/Time</title>
  </head>
  <body>
    <pre>{{ datetime }}</pre>.
  </body>
</html>
```

#### Rendered

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <title>Date/Time</title>
  </head>
  <body>
    <pre>2021-09-28T19:58:25+0000</pre>.
  </body>
</html>
```

## Usage

```sh
$ temple --help
temple 0.1.0
Ben Wilber <benwilber@gmail.com>
A commandline program that renders template files with structured context inputs

USAGE:
    temple [FLAGS] [OPTIONS] <TEMPLATE>

FLAGS:
    -e, --env              Reads context input from the environment
    -f, --force            Overwrites output files if they already exist.  By default, the program will not overwite
                           files that already exist
    -h, --help             Prints help information
    -n, --no-autoescape    Disables template autoescaping.  When autoescaping is on, which is the default, special
                           characters in context input values will be escaped when rendering template files that end
                           with .html, .htm, or .xml.
    -V, --version          Prints version information

OPTIONS:
    -c, --context <FILE>           The context input file.  If FILE is a single dash ("-"), or absent, reads from the
                                   standard input
                                    [default: -]
    -F, --format <FORMAT>          The context input format.  The program will normally try to discover the context
                                   input format automatically. But if that doesn't work, or yields unexpected results,
                                   then this option can be given explicitly.  This is most often required when reading
                                   context input from the standard input
                                    [possible values: json, yaml, kv]
    -l, --load <PATH>              The Lua file or directory to load custom scripts.  If PATH is a directory, then loads
                                   the file "init.lua" located at the top-level
    -o, --output <FILE>            The rendered output file.  If FILE is a single dash ("-"), or absent, writes to the
                                   standard output
                                    [default: -]
    -t, --templates <DIRECTORY>    The directory to search for additional templates for use with "{% extends ... %}" or
                                   "{% include ... %}" template tags

ARGS:
    <TEMPLATE>    The template file to render with the given context input
```

# Tests
Testing uses the [BATS](https://github.com/sstephenson/bats) testing framework.  The tests can be run like:

```sh
$ make testcli
cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
bats tests/tests.bats
 ✓ env
 ✓ json-stdin
 ✓ yaml-stdin
 ✓ kv-stdin
 ✓ json-file
 ✓ yaml-file
 ✓ kv-file
 ✓ json-empty
 ✓ yaml-empty
 ✓ kv-empty
 ✓ invalid-empty
 ✓ invalid-json-malformed
 ✓ invalid-yaml-malformed
 ✓ extends
 ✓ include
 ✓ autoescape
 ✓ no-autoescape

17 tests, 0 failures
```

# Author

`temple` is written by Ben Wilber <benwilber@gmail.com>

# Acknowledgments

* [minijinja](https://github.com/mitsuhiko/minijinja)
* [rlua](https://github.com/amethyst/rlua)

# Reporting bugs
Report bugs in the [bug tracker](https://github.com/benwilber/temple/issues).

# Copyright
[Apache](LICENSE)
