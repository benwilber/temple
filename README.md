# `temple`

A commandline program that renders template files with structured context inputs.  It is most often used to transform JSON data from a web API to a presentation format such as HTML.

In addition to JSON, `temple` can also read context inputs as YAML, and from the environment.  It is sometimes employed as a powerful alternative to tools like `envsubst`, which are used to render configuration files from parameters given as environment variables in 12-factor/Herokuish type environments.

Templates are rendered using [MiniJinja](https://github.com/mitsuhiko/minijinja), a port of the [Jinja2](https://jinja2docs.readthedocs.io/en/stable/) Python templating library for Rust.  `temple` supports any of the templating features that MiniJinja supports.  Visit those docs for more comprehensive examples and documentation about the templating language.

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
Ben Wilber <benwilber@pm.me>
Commandline template renderer

USAGE:
    temple [FLAGS] [OPTIONS] <TEMPLATE>

FLAGS:
    -E, --env               Reads context input from the environment
    -f, --force             Overwrites output files if they already exist.  By default, the program will not overwite files
                            that already exist
    -h, --help              Prints help information
    -n, --no-auto-escape    Disables template auto-escaping.  When auto-escaping is on, which is the default, special
                            characters in context input values will be escaped when rendering template files that end with
                            .htm, .html, or .xml.
    -V, --version           Prints version information

OPTIONS:
    -c, --context <FILE>             The context input file.  If FILE is a single dash ("-"), or absent, reads from the
                                     standard input
                                      [default: -]
    -e, --extensions <EXTENSIONS>    The list of file extensions that are considered to be templates.  Hidden files are
                                     always ignored. Separate multiple file extensions with a comma (",")
                                      [env: TEMPLE_TEMPLATE_EXTENSIONS=]  [default: htm,html,txt]
    -F, --format <FORMAT>            The context input format.  The program will try to discover the context input format
                                     automatically, but if that doesn't work, or if it yields unexpected results, then this
                                     option can be given explicitly.  It is most often required when reading context input
                                     from the standard input
                                      [env: TEMPLE_CONTEXT_FORMAT=]  [possible values: json, yaml]
    -o, --output <FILE>              The rendered output file.  If FILE is a single dash ("-"), or absent, writes to the
                                     standard output
                                      [default: -]
    -t, --templates <DIRECTORY>      The directory to search for additional templates when using "{% extends ... %}" or "{%
                                     include ... %}" template tags
                                      [env: TEMPLE_TEMPLATES=]

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
 ✓ json-file
 ✓ yaml-file
 ✓ json-empty
 ✓ yaml-empty
 ✓ invalid-empty
 ✓ invalid-json-malformed
 ✓ invalid-yaml-malformed
 ✓ extends
 ✓ include
 ✓ auto-escape
 ✓ no-auto-escape
 ✓ options-from-env-templates
 ✓ options-from-env-context-format
 ✓ output-file
 ✓ output-file-exists

18 tests, 0 failures
```

# Author

`temple` is written by Ben Wilber <benwilber@gmail.com>

# Acknowledgments

* [MiniJinja](https://github.com/mitsuhiko/minijinja)

# Reporting bugs
Report bugs in the [bug tracker](https://github.com/benwilber/temple/issues)

# Copyright
[Apache](LICENSE)
