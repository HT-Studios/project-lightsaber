use std::io::{
    self,
    BufRead,
    Write
};

use anyhow::Context;

pub enum AdvancedBuildChoice {
    ProceedWithoutAdvancedBuild,
    ProceedWithAdvancedBuild,
    AbortBuild
}

pub fn advanced_build() -> anyhow::Result<AdvancedBuildChoice> {
    println!("1: Proceed with the default build options (default)");
    println!("2: Customize the build options");
    println!("3: Abort build");

    print!("> ");

    let _ = io::stdout().flush();
    let input = read_line()?;

    let return_value = match &*input {
        "1" | "" => AdvancedBuildChoice::ProceedWithoutAdvancedBuild,
        "2" => AdvancedBuildChoice::ProceedWithAdvancedBuild,
        _ => AdvancedBuildChoice::AbortBuild
    };

    println!();

    Ok(return_value)
}

pub fn question_str(question: &str, default: &str) -> anyhow::Result<String> {
    println!("{} [{}]", question, default);

    let _ = io::stdout().flush();
    let input = read_line()?;

    println!();

    if input.is_empty() {
        Ok(default.to_string())
    }
    else {
        Ok(input)
    }
}

pub fn read_line() -> anyhow::Result<String> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut lines = stdin.lines();
    let lines = lines.next().transpose()?;

    match lines {
        Some(value) => Ok(value),
        None => Err(anyhow::anyhow!("No lines from standard input is detected."))
    }
        .context("Unable to read from standard input.")
}
