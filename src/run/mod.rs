use anyhow::Error;
use rust::RUN_RUST;
use std::process::Command;

use crate::commit::AwqConfig;

pub mod rust;

pub struct Checker {
    config: AwqConfig,
}

impl Checker {
    pub const fn new(config: AwqConfig) -> Self {
        Self { config }
    }
    pub fn check(&self) -> Result<(), Error> {
        match self.config.language.as_str() {
            "rust" => {
                for task in RUN_RUST {
                    if Command::new("cargo")
                        .args(task.split_whitespace())
                        .current_dir(".awq/src")
                        .spawn()?
                        .wait()?
                        .success()
                        .eq(&false)
                    {
                        return Err(anyhow::anyhow!("test failed"));
                    }
                }
            }
            _ => {
                println!("Language: {}", self.config.language);
            }
        }
        Ok(())
    }
}
