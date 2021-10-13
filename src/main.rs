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
    let lua = mlua::Lua::new();
    lua.load(include_str!("temple.lua")).exec().unwrap();
    let globals = lua.globals();
    let temple: mlua::Table = globals.get("temple").unwrap();
    let filters: mlua::Table = temple.get("_filters").unwrap();
    let concat2: mlua::Function = filters.get("concat2").unwrap();

    env.add_filter(
        "concat2",
        |_env: &Environment, s1: String, s2: String| -> anyhow::Result<String, minijinja::Error> {
            let res: String = concat2.call::<(String, String), String>((s1, s2)).unwrap();
            Ok(res)
        },
    );

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

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
mod tests {
    use shlex;
    use std::ffi::OsString;
    use std::fs;
    use std::path::Path;
    use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};

    // Run a commandline and return a triple of (exit_code, stdout, stderr)
    fn run_cmd(
        cmd: &str,
        stdin: Option<&str>,
        env: Option<Vec<(OsString, OsString)>>,
    ) -> anyhow::Result<(Option<String>, Option<String>, ExitStatus)> {
        let mut p = Popen::create(
            &shlex::split(cmd).unwrap(),
            PopenConfig {
                stdout: Redirection::Pipe,
                stderr: Redirection::Pipe,
                stdin: if stdin.is_some() {
                    Redirection::Pipe
                } else {
                    Redirection::None
                },
                env: env,
                ..Default::default()
            },
        )?;
        let (out, err) = p.communicate(stdin)?;
        let ret = p.wait().unwrap();
        Ok((out, err, ret))
    }

    #[test]
    fn env() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --env tests/templates/simple.txt",
            None,
            Some(vec![("FOO".into(), "bar".into())]),
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("bar".to_string()));
        assert_eq!(err, Some("".to_string()));
    }

    #[test]
    fn json_stdin() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --format=json tests/templates/simple.txt",
            Some(&fs::read_to_string("tests/contexts/simple.json").unwrap()),
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("bar".to_string()));
        assert_eq!(err, Some("".to_string()));

        let (out, err, ret) = run_cmd(
            "target/debug/temple -F json tests/templates/simple.txt",
            Some(&fs::read_to_string("tests/contexts/simple.json").unwrap()),
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("bar".to_string()));
        assert_eq!(err, Some("".to_string()));
    }

    #[test]
    fn yaml_stdin() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --format=yaml tests/templates/simple.txt",
            Some(&fs::read_to_string("tests/contexts/simple.yml").unwrap()),
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("bar".to_string()));
        assert_eq!(err, Some("".to_string()));

        let (out, err, ret) = run_cmd(
            "target/debug/temple -F yaml tests/templates/simple.txt",
            Some(&fs::read_to_string("tests/contexts/simple.yml").unwrap()),
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("bar".to_string()));
        assert_eq!(err, Some("".to_string()));
    }

    #[test]
    fn json_file() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --context=tests/contexts/simple.json tests/templates/simple.txt",
            None,
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("bar".to_string()));
        assert_eq!(err, Some("".to_string()));

        let (out, err, ret) = run_cmd(
            "target/debug/temple -c tests/contexts/simple.json -F json tests/templates/simple.txt",
            None,
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("bar".to_string()));
        assert_eq!(err, Some("".to_string()));
    }

    #[test]
    fn yaml_file() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --context=tests/contexts/simple.yml tests/templates/simple.txt",
            None,
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("bar".to_string()));
        assert_eq!(err, Some("".to_string()));

        let (out, err, ret) = run_cmd(
            "target/debug/temple -c tests/contexts/simple.yml -F yaml tests/templates/simple.txt",
            None,
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("bar".to_string()));
        assert_eq!(err, Some("".to_string()));
    }

    #[test]
    fn json_empty() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --format=json tests/templates/simple.txt",
            Some("{}"),
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("".to_string()));
        assert_eq!(err, Some("".to_string()));

        let (out, err, ret) = run_cmd(
            "target/debug/temple -F json tests/templates/simple.txt",
            Some("{}"),
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("".to_string()));
        assert_eq!(err, Some("".to_string()));
    }

    #[test]
    fn yaml_empty() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --format=yaml tests/templates/simple.txt",
            Some("-"),
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("".to_string()));
        assert_eq!(err, Some("".to_string()));

        let (out, err, ret) = run_cmd(
            "target/debug/temple -F yaml tests/templates/simple.txt",
            Some("-"),
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("".to_string()));
        assert_eq!(err, Some("".to_string()));
    }

    #[test]
    fn invalid_empty() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --context=tests/contexts/empty.txt tests/templates/simple.txt",
            Some("-"),
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(64));
        assert_eq!(out, Some("".to_string()));
        assert_ne!(err, Some("".to_string()));
    }

    #[test]
    fn invalid_json_malformed() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --context=tests/contexts/invalid_malformed.json tests/templates/simple.txt",
            Some("-"),
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(65));
        assert_eq!(out, Some("".to_string()));
        assert_ne!(err, Some("".to_string()));
    }

    #[test]
    fn invalid_yaml_malformed() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --context=tests/contexts/invalid_malformed.yml tests/templates/simple.txt",
            Some("-"),
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(65));
        assert_eq!(out, Some("".to_string()));
        assert_ne!(err, Some("".to_string()));
    }

    #[test]
    fn extends() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --context=tests/contexts/simple.json --templates=tests/templates tests/templates/extends.txt",
            None,
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("EXTENDS: bar".to_string()));
        assert_eq!(err, Some("".to_string()));
    }

    #[test]
    fn include() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --context=tests/contexts/simple.json --templates=tests/templates tests/templates/include.txt",
            None,
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("INCLUDE: bar".to_string()));
        assert_eq!(err, Some("".to_string()));
    }

    #[test]
    fn auto_escape() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --context=tests/contexts/auto_escape.json --templates=tests/templates tests/templates/auto_escape.html",
            None,
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(
            out,
            Some("&lt;script&gt;bar&lt;&#x2f;script&gt;".to_string())
        );
        assert_eq!(err, Some("".to_string()));
    }

    #[test]
    fn no_auto_escape() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --no-auto-escape --context=tests/contexts/auto_escape.json --templates=tests/templates tests/templates/auto_escape.html",
            None,
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("<script>bar</script>".to_string()));
        assert_eq!(err, Some("".to_string()));
    }

    #[test]
    fn options_from_env_templates() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple --context=tests/contexts/simple.json tests/templates/include.txt",
            None,
            Some(vec![("TEMPLE_TEMPLATES".into(), "tests/templates".into())]),
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("INCLUDE: bar".to_string()));
        assert_eq!(err, Some("".to_string()));
    }

    #[test]
    fn options_from_env_context_format() {
        let (out, err, ret) = run_cmd(
            "target/debug/temple tests/templates/simple.txt",
            Some("{\"FOO\": \"bar\"}"),
            Some(vec![("TEMPLE_CONTEXT_FORMAT".into(), "json".into())]),
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(out, Some("bar".to_string()));
        assert_eq!(err, Some("".to_string()));
    }

    #[test]
    fn output_file() {
        let _ = fs::remove_file(Path::new("tests/outputs/output.txt"));
        let (out, err, ret) = run_cmd(
            "target/debug/temple --output=tests/outputs/output.txt --context=tests/contexts/simple.json tests/templates/simple.txt",
            None,
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(
            Some(fs::read_to_string("tests/outputs/output.txt").unwrap()),
            Some("bar".to_string())
        );
        assert_eq!(out, Some("".to_string()));
        assert_eq!(err, Some("".to_string()));

        // Run it again and make sure it doesn't overwrite the previous output file.
        let (out, err, ret) = run_cmd(
            "target/debug/temple --output=tests/outputs/output.txt --context=tests/contexts/simple.json tests/templates/simple.txt",
            None,
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(73));
        assert_eq!(
            Some(fs::read_to_string("tests/outputs/output.txt").unwrap()),
            Some("bar".to_string())
        );
        assert_eq!(out, Some("".to_string()));
        assert_ne!(err, Some("".to_string()));

        // Run it again and make sure it overwrote the previous output file.
        let (out, err, ret) = run_cmd(
            "target/debug/temple --force --output=tests/outputs/output.txt --context=tests/contexts/simple.json tests/templates/simple.txt",
            None,
            None,
        )
        .unwrap();
        assert_eq!(ret, ExitStatus::Exited(0));
        assert_eq!(
            Some(fs::read_to_string("tests/outputs/output.txt").unwrap()),
            Some("bar".to_string())
        );
        assert_eq!(out, Some("".to_string()));
        assert_eq!(err, Some("".to_string()));
    }
}
