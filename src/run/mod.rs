use crate::run::rust::RUN_RUST;
use anyhow::{anyhow, Error};
use std::fs::File;
use std::process::Command;
pub mod rust;
pub struct Checker {}

impl Checker {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }

    fn run(&self, lang: &str, tasks: Vec<(&str, &str, &str)>) -> Result<(), Error> {
        println!(">> checking {lang} code");
        for task in &tasks {
            println!(">> {}", task.1.to_string());
            if Command::new("sh")
                .args(&["-c", task.2])
                .current_dir(".")
                .stdout(File::create(
                    format!("zuu/{lang}/stdout/{}", task.0).as_str(),
                )?)
                .stderr(File::create(format!("zuu/stderr/{}", task.0).as_str())?)
                .spawn()?
                .wait()?
                .success()
            {
                continue;
            }
            return Err(anyhow!(format!(">> {} failure", task.1)));
        }
        println!(">> {lang} code validated");
        Ok(())
    }
    pub fn check(&self) -> Result<(), Error> {
        self.run("rust", RUN_RUST.to_vec())?;
        Ok(())
    }
}
