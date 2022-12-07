use anyhow::{anyhow, Result};
use clap::Parser;

use crate::ExtraArgs;

#[derive(Parser, Debug, Clone)]
#[clap(version, author, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Parser, Debug, Clone)]
#[non_exhaustive]
pub enum Action {
    Run(RunArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct RunArgs {
    #[clap(short, long, value_parser)]
    pub profile: String,

    /// Overrides args.
    /// For query params. use '-e key=value'.
    /// For header, use '-e %key=value'.
    /// For body, use '-e @key=value'.
    #[clap(short, long, value_parser = parse_key_val, number_of_values = 1)]
    pub extra_params: Vec<KeyVal>,

    #[clap(short, long, value_parser)]
    pub config: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum KeyValType {
    Query,
    Header,
    Body,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct KeyVal {
    key_val_type: KeyValType,
    key: String,
    val: String,
}

fn parse_key_val(s: &str) -> Result<KeyVal> {
    let mut parts = s.splitn(2, '=');
    let key = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid key value pair:{s}"))?.trim();
    let val = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid key value pair:{s}"))?.trim();
    let (key_val_type, key) = match key.chars().next() {
        Some('%') => (KeyValType::Header, &key[1..]),
        Some('@') => (KeyValType::Body, &key[1..]),
        Some(x) if x.is_ascii_alphabetic() => (KeyValType::Query, key),
        _ => return Err(anyhow!("Invalid key value pair."))
    };
    Ok(KeyVal {
        key_val_type,
        key: key.to_string(),
        val: val.to_string(),
    })
}


impl From<Vec<KeyVal>> for ExtraArgs {
    fn from(args: Vec<KeyVal>) -> Self {
        let mut query = vec![];
        let mut headers = vec![];
        let mut body = vec![];

        for arg in args {
            match arg.key_val_type {
                KeyValType::Query => query.push((arg.key, arg.val)),
                KeyValType::Header => headers.push((arg.key, arg.val)),
                KeyValType::Body => body.push((arg.key, arg.val)),
            }
        }

        Self {
            headers,
            query,
            body,
        }
    }
}

