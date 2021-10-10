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

    if let Some(templates_path) = matches.value_of("templates") {
        let extensions = matches
            .value_of("extensions")
            .unwrap()
            .split(',')
            .map(|extension| {
                extension
                    .trim()
                    .strip_prefix('.')
                    .unwrap_or(extension)
                    .trim()
            })
            .collect::<Vec<_>>();
        source
            .load_from_path(templates_path, &extensions)
            .map_err(|e| error_exit(&format!("{}", e), exitcode::IOERR))
            .unwrap();
    }

    let template_path = matches.value_of("TEMPLATE").unwrap();
    let template_name = format!("__temple_base__/{}", template_path);

    match fs::read_to_string(template_path) {
        Ok(template_content) => {
            source
                .add_template(&template_name, template_content)
                .map_err(|e| {
                    error_exit(&format!("{}", e), exitcode::DATAERR);
                })
                .unwrap();
        }
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
            "unknown or ambiguous context input format. Try adding -F/--format=<format>",
            exitcode::USAGE,
        );
    }

    let mut env = Environment::new();

    if matches.is_present("no_auto_escape") {
        env.set_auto_escape_callback(|_| minijinja::AutoEscape::None);
    }

    env.set_source(source);

    let template = env.get_template(&template_name).unwrap();
    let rendered;

    match (context_format, context_content) {
        (ContextFormat::Json, Some(context_content)) => {
            let context: serde_json::Value = match serde_json::from_str(&context_content) {
                Ok(context) => context,
                Err(e) => {
                    return error_exit(&format!("{}", e), exitcode::DATAERR);
                }
            };
            rendered = template.render(context).unwrap();
        }
        (ContextFormat::Yaml, Some(context_content)) => {
            let context: serde_yaml::Value = match serde_yaml::from_str(&context_content) {
                Ok(context) => context,
                Err(e) => {
                    return error_exit(&format!("{}", e), exitcode::DATAERR);
                }
            };
            rendered = template.render(context).unwrap();
        }
        (ContextFormat::Env, None) => {
            let mut context = HashMap::new();

            for (key, value) in std::env::vars() {
                context.insert(key, value);
            }

            rendered = template.render(context).unwrap();
        }
        _ => {
            // This shouldn't be possible if clap is validating the CLI correctly.
            unreachable!()
        }
    }

    match matches.value_of("output") {
        Some("-") | None => {
            print!("{}", rendered);
        }
        Some(output_file_path) => {
            let open_output_file_result;

            if matches.is_present("force") {
                open_output_file_result = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(output_file_path);
            } else {
                open_output_file_result = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(output_file_path);
            }

            match open_output_file_result {
                Ok(file) => {
                    write!(&file, "{}", &rendered).unwrap();
                }
                Err(e) => {
                    let msg = format!("{}: {}", output_file_path, e);
                    error_exit(&msg, exitcode::CANTCREAT);
                }
            }
        }
    }
}
