use clap::Parser;
use std::{process::Command};
use question::{Answer, Question};
use reqwest::blocking::Client;
use serde_json::json;

#[derive(Parser, Debug)]
#[command(version)]
#[command(name = "Auto install")]
#[command(author = "Lane.W <geekerlane@gmail.com>")]
#[command(about = "Automagically generate install command.", long_about = None)]
struct Args {
    #[arg(help="Anything you want to install.")]
    anything: String,

    #[arg(short, long, help = "Force execute command without any confirm.", default_value_t={false})]
    force: bool,
}

fn main() {
    let args = Args::parse();

    let anything = args.anything;
    let api_token = std::env::var("OPENAI_KEY").unwrap();

    let system_info = if cfg!(target_os = "macos") {
        "macOS"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else {
        ""
    };

    let client = Client::new();

    let response = client
    .post("https://api.openai.com/v1/completions")
    .json(&json!({
        "top_p": 1,
        "stop": "```",
        "temperature": 0,
        "suffix": "\n```",
        "max_tokens": 1000,
        "presence_penalty": 0,
        "frequency_penalty": 0,
        "model": "text-davinci-003",
        "prompt": format!("Single command to install {} on {}:\n```bash\n#!/bin/bash\n", anything, system_info),
    }))
    .header("Authorization", format!("Bearer {api_token}"))
    .send()
    .unwrap()
    .error_for_status()
    .unwrap();


    let result = response.json::<serde_json::Value>().unwrap()["choices"][0]["text"]
        .as_str()
        .unwrap()
        .trim()
        .to_string();

    println!("================================= Generated Command =======================================");
    println!("{}", result);
    println!("===========================================================================================");


    let mut run_cmd = true;
    

    if !args.force {
        run_cmd = Question::new(
            ">> Run the generated program? [Y/n]"
        )
        .yes_no()
        .until_acceptable()
        .default(Answer::YES)
        .ask()
        .expect("Couldn't ask question.")
            == Answer::YES
    }

    if !run_cmd{
        std::process::exit(0);
    }

    let status=Command::new("bash")
        .arg("-c")
        .arg(result)
        .status()
        .expect("ls command failed to start");
    if status.success() {
        std::process::exit(0);
    }
    else{
        std::process::exit(1);
    }
}
