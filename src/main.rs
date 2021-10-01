#[macro_use]
extern crate clap;
use clap::App;
use minijinja::Environment;
use rlua::{Function, Lua, Table};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
struct Template {
    name: String,
    path: Option<PathBuf>,
    source: String,
}

#[derive(Debug, PartialEq)]
enum ContextFormat {
    Json,
    Yaml,
    Kv,
    Env,
    Unknown,
}

fn find_templates(root_dir: &str) -> Vec<Template> {
    let mut templates: Vec<Template> = Vec::new();
    let walker = WalkDir::new(root_dir).into_iter();

    for entry in walker
        .filter_entry(|entry| {
            !entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
    {
        let path = entry.path();
        let name = path
            .strip_prefix(root_dir)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        if let Ok(source) = std::fs::read_to_string(path) {
            templates.push(Template {
                name,
                path: Some(path.to_path_buf()),
                source,
            });
        }
    }

    templates
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

fn error_exit(msg: &str, code: exitcode::ExitCode) {
    eprintln!("temple: {}", msg);
    std::process::exit(code);
}

fn render_template(
    template: minijinja::Template,
    context_format: ContextFormat,
    context_source: Option<String>,
) -> anyhow::Result<String> {
    let rendered;

    match (context_format, context_source) {
        (ContextFormat::Json, Some(context_source)) => {
            let context: serde_json::Value = serde_json::from_str(&context_source)?;
            rendered = template.render(context)?;
        }
        (ContextFormat::Yaml, Some(context_source)) => {
            let context: serde_yaml::Value = serde_yaml::from_str(&context_source)?;
            rendered = template.render(context)?;
        }
        (ContextFormat::Kv, Some(context_source)) => {
            let context = parse_kv(&context_source)?;
            rendered = template.render(context)?;
        }
        (ContextFormat::Env, None) => {
            let mut context = HashMap::new();

            for (key, value) in std::env::vars() {
                context.insert(key, value);
            }

            rendered = template.render(context)?;
        }
        _ => {
            panic!("unknown context input format");
        }
    }

    Ok(rendered)
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

fn setup_lua_package_path(lua: &Lua, load_path: &Path) -> anyhow::Result<()> {
    lua.context::<_, anyhow::Result<()>>(|ctx| {
        let globals = ctx.globals();
        let package: Table = globals.get("package")?;
        let package_path: String = package.get("path")?;
        package.set(
            "path",
            format!("{}/?.lua;{}", load_path.display(), package_path),
        )?;

        Ok(())
    })?;

    Ok(())
}

fn build_lua_script(user_script: &str) -> anyhow::Result<String> {
    let temple_script = include_str!("temple.lua");
    Ok(format!("{}\n{}", &temple_script, &user_script))
}

fn load_lua_scripts(lua: &Lua, load_path: Option<&Path>) -> anyhow::Result<()> {
    let user_script;

    if let Some(load_path) = load_path {
        if load_path.is_dir() {
            setup_lua_package_path(lua, load_path)?;
            user_script = std::fs::read_to_string(load_path.join("init.lua"))?;
        } else {
            user_script = std::fs::read_to_string(load_path)?;
        }
    } else {
        user_script = String::new();
    }

    let final_script = build_lua_script(&user_script)?;

    lua.context::<_, anyhow::Result<()>>(|ctx| {
        ctx.load(&final_script).exec()?;
        Ok(())
    })?;

    Ok(())
}

fn main() {
    let cli = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli).get_matches();
    let mut env = Environment::new();
    let mut templates: Vec<Template>;
    let lua = Lua::new();

    if let Some(load_path) = matches.value_of("load") {
        load_lua_scripts(&lua, Some(Path::new(load_path)))
            .map_err(|e| {
                let msg = format!("{}", e);
                error_exit(&msg, exitcode::SOFTWARE);
            })
            .unwrap();
    } else {
        load_lua_scripts(&lua, None)
            .map_err(|e| {
                let msg = format!("{}", e);
                error_exit(&msg, exitcode::SOFTWARE);
            })
            .unwrap();
    }

    /*
    let mut filters = Vec::new();

    lua.context::<_, anyhow::Result<()>>(|ctx| {
        let globals = ctx.globals();
        let temple_table: Table = globals.get("temple")?;
        let filters_table: Table = temple_table.get("filters")?;

        for pair in filters_table.pairs::<String, Function>() {
            let (name, _func) = pair?;
            filters.push(name);
        }

        for name in &filters {
            env.add_filter(
                name,
                |_env: &Environment,
                 v: String,
                 s: String|
                 -> anyhow::Result<String, minijinja::Error> {
                    Ok(name.to_string())
                },
            );
        }

        Ok(())
    })
    .unwrap();
    */

    if matches.is_present("no_auto_escape") {
        env.set_auto_escape_callback(|_| minijinja::AutoEscape::None);
    }

    if let Some(templates_root_dir) = matches.value_of("templates") {
        templates = find_templates(templates_root_dir);
    } else {
        templates = Vec::new();
    }

    let root_template = matches.value_of("TEMPLATE").unwrap();
    let root_template_path = Path::new(root_template);

    match std::fs::read_to_string(root_template_path) {
        Ok(source) => {
            templates.push(Template {
                name: root_template.to_string(),
                path: Some(root_template_path.to_path_buf()),
                source,
            });
        }
        Err(e) => {
            let msg = format!("{}: {}", root_template_path.display(), e);
            return error_exit(&msg, exitcode::IOERR);
        }
    }

    for template in &templates {
        match env.add_template(&template.name, &template.source) {
            Ok(_) => {
                //println!("Added template: {:?}", template);
            }
            Err(e) => {
                let msg;

                if let Some(path) = &template.path {
                    msg = format!("{}: {}", path.display(), e)
                } else {
                    msg = format!("{}", e);
                }

                return error_exit(&msg, exitcode::DATAERR);
            }
        }
    }

    let mut context_format = ContextFormat::Unknown;
    let context_source;

    if matches.is_present("env") {
        context_format = ContextFormat::Env;
        context_source = None;
    } else {
        if let Some(f) = matches.value_of("format") {
            match f.to_lowercase().as_str() {
                "json" => context_format = ContextFormat::Json,
                "yaml" | "yml" => context_format = ContextFormat::Yaml,
                "kv" => context_format = ContextFormat::Kv,
                _ => {}
            }
        }

        context_source = match matches.value_of("context") {
            Some("-") | None => match read_stdin() {
                Ok(s) => Some(s),
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
                    Ok(s) => Some(s),
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

    let template = env.get_template(root_template).unwrap();

    match render_template(template, context_format, context_source) {
        Ok(rendered) => match matches.value_of("output") {
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
        },
        Err(e) => {
            let msg = format!("{}", e);
            error_exit(&msg, exitcode::DATAERR);
        }
    }
}
