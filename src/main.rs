use clap::Parser;
use regex::Regex;
use std::{
    process,
    process::{Command, Stdio},
};

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"),version = env!("CARGO_PKG_VERSION"),arg_required_else_help = true,)]
struct Args {
    problemid: String,
}

struct Problem {
    contest_id: String,
    problem_index: String,
}

fn check_problemid(problemid: String) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9]+_[a-zA-Z0-9]+$").unwrap();

    re.is_match(&problemid)
}

fn parse_problemid(problemid: String) -> Problem {
    let contest_id_re = Regex::new(r"^[a-zA-Z0-9]+").unwrap();
    let cap_contest_id = contest_id_re.captures(&problemid).unwrap();
    let contest_id = &cap_contest_id[0].to_string();

    let problem_index_re = Regex::new(r"[a-zA-Z0-9]+$").unwrap();
    let cap_problem_index = problem_index_re.captures(&problemid).unwrap();
    let problem_index = &cap_problem_index[0].to_string();

    return Problem {
        contest_id: contest_id.to_string(),
        problem_index: problem_index.to_string(),
    };
}

fn download(problem: Problem) -> std::io::Result<()> {
    let mut cmd = Command::new("oj")
        .arg("d")
        .arg(format!(
            "https://atcoder.jp/contests/{}/tasks/{}_{}",
            problem.contest_id, problem.contest_id, problem.problem_index
        ))
        .arg("-d")
        .arg("tests")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let status = cmd.wait()?;

    println!("");

    match status.code() {
        Some(code) => {
            if code == 0 {
                println!("download successful")
            } else {
                println!("download faild");
            }
        }
        None => {
            println!("download faild")
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    if !check_problemid(args.problemid.clone()) {
        println!("format is \"[contest id]_[problem_index]\"");
        process::exit(1);
    }

    let problem = parse_problemid(args.problemid.clone());
    let _result = download(problem);
}
