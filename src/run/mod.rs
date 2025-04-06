use crate::commit::AwqConfig;
use crate::run::c::RUN_C;
use crate::run::cmake::RUN_CMAKE;
use crate::run::nodejs::RUN_NPM;
use crate::run::rust::RUN_RUST;
use anyhow::{anyhow, Error};
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;

pub mod c;
pub mod go;
pub mod cmake;
pub mod nodejs;
pub mod rust;
pub struct Checker {
    config: AwqConfig,
}

impl Checker {
    #[must_use]
    pub const fn new(config: AwqConfig) -> Self {
        Self { config }
    }

    pub fn run(&self, lang: &str, tasks: Vec<(&str, &str, &str)>) -> Result<(), Error> {
        let p = format!("zuu/{lang}");
        if Path::new(p.as_str()).exists() {
            fs::remove_dir_all(p.as_str())?;
        }
        if Path::new(p.as_str()).exists().eq(&false) {
            fs::create_dir(p.as_str())?;
            fs::create_dir(format!("{p}/stdout").as_str())?;
            fs::create_dir(format!("{p}/stderr").as_str())?;
        }
        println!(">> checking {lang} code");
        for task in &tasks {
            println!(">> {}", task.1.to_string());
            if Command::new("sh")
                .args(&["-c", task.2])
                .current_dir(".awq/src")
                .stdout(File::create(
                    format!(
                        "{}/zuu/{lang}/stdout/{}",
                        current_dir()?.to_str().expect("failed to get current dir"),
                        task.0
                    )
                    .as_str(),
                )?)
                .stderr(File::create(
                    format!(
                        "{}/zuu/{lang}/stderr/{}",
                        current_dir()?.to_str().expect("failed to get current dir"),
                        task.0
                    )
                    .as_str(),
                )?)
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
        if Path::new("zuu").exists().eq(&false) {
            fs::create_dir("zuu")?;
        }

        for lang in &self.config.language {
            match lang.as_str() {
                "rust" => self.run(lang.as_str(), RUN_RUST.to_vec())?,
                "js" => self.run(lang.as_str(), RUN_NPM.to_vec())?,
                "c" => self.run(lang.as_str(), RUN_C.to_vec())?,
                "cmake" => self.run(lang.as_str(), RUN_CMAKE.to_vec())?,
                _ => {
                    eprintln!(">> unknown language : {lang}")
                }
            }
        }
        Ok(())
    }
}
