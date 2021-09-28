# Name
`temple` - renders template files with structured context inputs


# Synopsis
```sh
$ curl -s https://sunspot.io/time.json | temple -f json datetime.html
```
### Context input:

```
{
  "datetime": "2021-09-28T04:12:56+0000"
}
```

### Template:

```
<pre>
  {{ datetime }}
</pre>
```

### Rendered:

```
<pre>
  2021-09-28T04:12:56+0000
</pre>
```

## Usage

```sh
$ temple --help
temple 0.1.0
Ben Wilber <benwilber@gmail.com>
Template renderer

USAGE:
    temple [FLAGS] [OPTIONS] <TEMPLATE>

FLAGS:
    -e, --env              Reads context input from the environment
    -h, --help             Prints help information
    -n, --no-autoescape    Disables template autoescaping.  When autoescaping is on, which is the default, all special
                           characters in context input values will be escaped when rendering template files that end
                           with .html, .htm, or .xml.
    -V, --version          Prints version information

OPTIONS:
    -c, --context <FILE>           The context input file.  If FILE is a single dash ("-"), or absent, reads from the
                                   standard input
                                    [default: -]
    -f, --format <FORMAT>          The context input format
                                    [possible values: json, yaml, kv]
    -o, --output <FILE>            The rendered output file.  The file must not already exist.  If FILE is a single dash
                                   ("-"), or absent, writes to the standard output
                                    [default: -]
    -t, --templates <DIRECTORY>    The directory to search for additional templates for use with "{% extends ... %}" or
                                   "{% include ... %}" template tags

ARGS:
    <TEMPLATE>    The template file to render with the given context input
```

# Author

`temple` is written by Ben Wilber <benwilber@gmail.com>

# Acknowledgments

* [minijinja](https://github.com/mitsuhiko/minijinja)
	* This makes the whole thing work
* [rlua](https://github.com/amethyst/rlua)
	* This makes all the extra stuff work

# Reporting bugs
Report bugs in the bug tracker at <https://github.com/benwilber/temple/issues> or by email to <benwilber+temple@gmail.com>.

# Copyright
[Apache](LICENSE)
