#[macro_use]
extern crate clap;
use clap::App;
use minijinja::{Environment, Source};
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug, PartialEq)]
enum ContextFormat {
    Json,
    Yaml,
    Kv,
    Env,
    Unknown,
}

fn error_exit(msg: &str, code: exitcode::ExitCode) {
    eprintln!("temple: {}", msg);
    std::process::exit(code);
}

fn read_stdin() -> anyhow::Result<String> {
    let mut s = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut s)?;
    Ok(s)
}

fn parse_kv(s: &str) -> anyhow::Result<HashMap<String, String>> {
    let mut kv = HashMap::new();

    for line in s.lines() {
        if let Some((k, v)) = line.split_once("=") {
            kv.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    Ok(kv)
}

fn guess_context_format(path: &Path) -> ContextFormat {
    return match path
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase()
        .as_str()
    {
        "json" => ContextFormat::Json,
        "yaml" | "yml" => ContextFormat::Yaml,
        _ => ContextFormat::Unknown,
    };
}

fn main() {
    let cli = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli).get_matches();
    let mut source = Source::new();

    if let Some(templates) = matches.value_of("templates") {
        let exts = matches
            .value_of("extensions")
            .unwrap()
            .split(',')
            .map(|ext| ext.trim().strip_prefix('.').unwrap_or(ext).trim())
            .collect::<Vec<_>>();

        match source.load_from_path(templates, &exts) {
            Ok(_) => {}
            Err(e) => {
                return error_exit(&format!("{}", e), exitcode::IOERR);
            }
        };
    }

    let template = matches.value_of("TEMPLATE").unwrap();
    let template_name = format!("__temple_base__/{}", template);

    match fs::read_to_string(template) {
        Ok(content) => match source.add_template(&template_name, content) {
            Ok(_) => {}
            Err(e) => {
                return error_exit(&format!("{}", e), exitcode::DATAERR);
            }
        },
        Err(e) => {
            return error_exit(&format!("{}", e), exitcode::IOERR);
        }
    }

    let mut context_format = ContextFormat::Unknown;
    let context_content;

    if matches.is_present("env") {
        context_format = ContextFormat::Env;
        context_content = None;
    } else {
        if let Some(format) = matches.value_of("format") {
            match format.to_lowercase().as_str() {
                "json" => context_format = ContextFormat::Json,
                "yaml" | "yml" => context_format = ContextFormat::Yaml,
                "kv" => context_format = ContextFormat::Kv,
                _ => {}
            }
        }

        context_content = match matches.value_of("context") {
            Some("-") | None => match read_stdin() {
                Ok(content) => Some(content),
                Err(e) => {
                    let msg = format!("{}", e);
                    return error_exit(&msg, exitcode::IOERR);
                }
            },
            Some(context_file_path) => {
                let path = Path::new(context_file_path);

                if context_format == ContextFormat::Unknown {
                    context_format = guess_context_format(path);
                }

                match std::fs::read_to_string(path) {
                    Ok(content) => Some(content),
                    Err(e) => {
                        let msg = format!("{}: {}", context_file_path, e);
                        return error_exit(&msg, exitcode::IOERR);
                    }
                }
            }
        };
    }

    if context_format == ContextFormat::Unknown {
        return error_exit(
            "unknown or ambiguous context input format.  Try adding -F/--format=<format>",
            exitcode::USAGE,
        );
    }

    let mut env = Environment::new();

    if matches.is_present("no_auto_escape") {
        env.set_auto_escape_callback(|_| minijinja::AutoEscape::None);
    }

    env.set_source(source);

    let tmpl = env.get_template(&template_name).unwrap();
    let rendered;

    match (context_format, context_content) {
        (ContextFormat::Json, Some(context_content)) => {
            let context: serde_json::Value = match serde_json::from_str(&context_content) {
                Ok(context) => context,
                Err(e) => {
                    return error_exit(&format!("{}", e), exitcode::DATAERR);
                }
            };
            rendered = tmpl.render(context).unwrap();
        }
        (ContextFormat::Yaml, Some(context_content)) => {
            let context: serde_yaml::Value = match serde_yaml::from_str(&context_content) {
                Ok(context) => context,
                Err(e) => {
                    return error_exit(&format!("{}", e), exitcode::DATAERR);
                }
            };
            rendered = tmpl.render(context).unwrap();
        }
        (ContextFormat::Kv, Some(context_content)) => {
            let context = parse_kv(&context_content).unwrap();
            rendered = tmpl.render(context).unwrap();
        }
        (ContextFormat::Env, None) => {
            let mut context = HashMap::new();

            for (key, value) in std::env::vars() {
                context.insert(key, value);
            }

            rendered = tmpl.render(context).unwrap();
        }
        _ => {
            panic!("unknown context input format");
        }
    }

    match matches.value_of("output") {
        Some("-") | None => {
            print!("{}", rendered);
        }
        Some(output) => {
            let open_result;

            if matches.is_present("force") {
                open_result = OpenOptions::new().write(true).create(true).open(output);
            } else {
                open_result = OpenOptions::new().write(true).create_new(true).open(output);
            }

            match open_result {
                Ok(f) => {
                    write!(&f, "{}", &rendered).unwrap();
                }
                Err(e) => {
                    let msg = format!("{}: {}", output, e);
                    error_exit(&msg, exitcode::CANTCREAT);
                }
            }
        }
    }
}
