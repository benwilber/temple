# `temple`

A fast commandline program that renders template files with structured context inputs.  It is most often used to transform JSON data from a web API to a presentation format such as HTML.

In addition to JSON, `temple` can also read context inputs as YAML, and from the environment.  It is sometimes employed as a powerful alternative to tools like `envsubst`, which are used to render configuration files from parameters given as environment variables in 12-factor/Herokuish type environments.

Templates are rendered using [MiniJinja](https://github.com/mitsuhiko/minijinja), a port of the [Jinja2](https://jinja2docs.readthedocs.io/en/stable/) Python templating library for Rust.  `temple` supports any of the templating features that MiniJinja supports.  Visit those docs for more comprehensive examples and documentation about the templating language.

# Installing

Pre-compiled binaries are available in the [releases section](https://github.com/benwilber/temple/releases) for most common platforms.

## Homebrew
The program can be installed from Homebrew with:

```sh
$ brew tap benwilber/temple https://github.com/benwilber/temple
$ brew install temple
$ temple --version
temple 0.3.0
```

## Source
The program can be compiled from sources using standard Rust/Cargo toolchain.

```sh
$ cargo build --release
$ ./target/release/temple --version
temple 0.3.0
```

# Examples

Check out the [examples](examples) directory.

This example renders the USGS Polar Bears [JSON dataset](https://polar-bears.vercel.app/polar-bears/USGS_WC_eartags_output_files_2009-2011-Status) as an HTML table.

```sh
$ curl -s https://polar-bears.vercel.app/polar-bears/USGS_WC_eartags_output_files_2009-2011-Status.json | \
  temple \
  --format=json \
  --templates=examples/polarbears/templates \
  examples/polarbears/templates/main.html
```

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <title>
      Polar Bears
    </title>
  </head>
  <body>    
    <h1>Polar Bears</h1>
    <table>
      <thead>
        <tr>
          <th>rowid</th>
          <th>DeployID</th>
          <th>Ptt</th>
          <th>Received</th>
          <th>LocationQuality</th>
          <th>Latitude</th>
          <th>Longitude</th>
          <th>Transmits</th>
          <th>BattVoltage</th>
          <th>TransmitCurrent</th>
          <th>Temperature</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>1</td>
          <td>20992</td>
          <td>80435</td>
          <td>12&#x2f;9&#x2f;2009 2:49</td>
          <td>1</td>
          <td>47.67599869</td>
          <td>-122.1329956</td>
          <td>128</td>
          <td>3.36</td>
          <td>0.312</td>
          <td>21.63</td>
        </tr>
        <!-- [...] -->
      </tbody>
    </table>
  </body>
</html>
```

## Usage

```sh
$ temple --help
```

```
temple 0.3.0
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

# FAQ

## Does `temple` have a configuration file?

No.

However, some common configuration options can be provided as environment variables instead of on the commandline.  For example, if you have a templates directory containing Markdown files (`.md`), and you normally only deal with JSON context data, then you can export the following options in your shell profile script:

```sh
export TEMPLE_TEMPLATES=/path/to/templates
export TEMPLE_TEMPLATE_EXTENSIONS=md
export TEMPLE_CONTEXT_FORMAT=json
```

**NOTE**

Options given on the commandline will always override the same options given in the environment.

# Tests
```sh
$ make test
cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests (target/debug/deps/temple-3a6cdad72a8d2255)

running 17 tests
test tests::invalid_yaml_malformed ... ok
test tests::invalid_json_malformed ... ok
test tests::extends ... ok
test tests::include ... ok
test tests::auto_escape ... ok
test tests::invalid_empty ... ok
test tests::env ... ok
test tests::json_empty ... ok
test tests::options_from_env_context_format ... ok
test tests::no_auto_escape ... ok
test tests::options_from_env_templates ... ok
test tests::json_file ... ok
test tests::json_stdin ... ok
test tests::yaml_empty ... ok
test tests::yaml_file ... ok
test tests::yaml_stdin ... ok
test tests::output_file ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```

# Author

`temple` is written by Ben Wilber <benwilber@pm.me>

# Acknowledgments

* [MiniJinja](https://github.com/mitsuhiko/minijinja) - The template engine
* [ripgrep](https://github.com/BurntSushi/ripgrep) - The Github workflows

# Reporting bugs
Report bugs in the [bug tracker](https://github.com/benwilber/temple/issues)

# Copyright
[Apache](LICENSE)
