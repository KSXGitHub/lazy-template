//! This example demonstrates dynamically and lazily loading expensive resources into string interpolations of [`lazy_template::simple_curly_braces`].

use derive_more::Display;
use lazy_template::simple_curly_braces;
use pipe_trait::Pipe;
use std::{borrow::Cow, env, fs, io, process::ExitCode};
use text_block_macros::text_block;

static HELP: &str = text_block! {
    "println — format and print data"
    ""
    "Usage:"
    "  cargo run --example=println -- <TEMPLATE> [...ARGS]"
    ""
    "Template Syntax:"
    "  * String interpolations are placed in curly braces"
    "  * Interpolation types:"
    "    - Empty (literally `{}`): Arguments after the template"
    "    - All uppercase letters and optional underscores: Environment variable (e.g. 'Home directory: {HOME}, Path: {PATH}')"
    "    - Prefixed with 'env:': Environment variable (e.g. 'Home directory: {env:HOME}, Path: {env:PATH}')"
    "    - Prefixed with 'file:': Read a text file (e.g. 'Content of my thesis: {file:./Documents/thesis.txt}')"
    ""
    "Examples:"
    "  cargo run --example=println -- 'Hello {}, my name is {}' Bob Alice"
    "  cargo run --example=println -- 'The current Rust version is {file:rust-toolchain}'"
    "  cargo run --example=println -- '{HOME} is my home'"
    "  cargo run --example=println -- '{env:HOME} is my home'"
};

#[derive(Debug, Display)]
#[display("{}", _0.trim_end_matches('\n'))]
struct DisplayMessage<'a>(Cow<'a, str>);

#[derive(Debug, Display)]
enum ErrorMessage<'a> {
    #[display("Positional argument at {_0} is not provided")]
    Arg(usize),
    #[display("Cannot read environment variable '{_0}': {_1}")]
    Env(&'a str, env::VarError),
    #[display("Failed to read file '{_0}': {_1}")]
    File(&'a str, io::Error),
    #[display("Unsupported query: {_0}")]
    Query(&'a str),
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        eprintln!("{HELP}");
        return ExitCode::SUCCESS;
    }

    let template_str = &args[1];
    let mut pos_args = args[2..].iter();
    let mut pos_index = 0;

    let system = simple_curly_braces();
    let template = system.lazy_parse(template_str);

    let result = template.to_string(|query| -> Result<DisplayMessage, ErrorMessage> {
        if is_positional(query) {
            let result = pos_args
                .next()
                .ok_or(ErrorMessage::Arg(pos_index))
                .map(String::as_str)
                .map(Cow::Borrowed)
                .map(DisplayMessage);
            pos_index += 1;
            return result;
        }

        if let Some(env) = parse_env_var(query) {
            return env
                .pipe(env::var)
                .map_err(|error| ErrorMessage::Env(env, error))
                .map(Cow::Owned)
                .map(DisplayMessage);
        }

        if let Some(file) = parse_file_path(query) {
            return file
                .pipe(fs::read_to_string)
                .map_err(|error| ErrorMessage::File(file, error))
                .map(Cow::Owned)
                .map(DisplayMessage);
        }

        query.pipe(ErrorMessage::Query).pipe(Err)
    });

    match result {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn is_positional(query: &str) -> bool {
    query.is_empty()
}

fn parse_env_var(query: &str) -> Option<&str> {
    if query.chars().all(|char| matches!(char, 'A'..='Z' | '_')) {
        return Some(query);
    }

    query.strip_prefix("env:")
}

fn parse_file_path(query: &str) -> Option<&str> {
    query.strip_prefix("file:")
}
