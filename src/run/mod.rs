use crate::commit::AwqConfig;
use anyhow::Error;
use is_executable::IsExecutable;
use rust::RUN_RUST;
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;

pub mod rust;
pub struct Checker {
    config: AwqConfig,
}

impl Checker {
    #[must_use]
    pub const fn new(config: AwqConfig) -> Self {
        Self { config }
    }
    pub fn check(&self) -> Result<(), Error> {
        if Path::new("zuu").exists().eq(&false) {
            fs::create_dir("zuu")?;
        }

        for lang in &self.config.language {
            match lang.as_str() {
                "rust" => {
                    if Path::new("zuu/rust").exists() {
                        fs::remove_dir_all("zuu/rust")?;
                    }
                    if Path::new("zuu/rust").exists().eq(&false) {
                        fs::create_dir("zuu/rust")?;
                        fs::create_dir("zuu/rust/stdout")?;
                        fs::create_dir("zuu/rust/stderr")?;
                    }
                    println!(">> checking rust code");
                    for task in RUN_RUST {
                        println!(">> {}", task.1.to_string());
                        if Command::new("cargo")
                            .args(task.2.split_whitespace())
                            .current_dir(".awq/src")
                            .stdout(File::create(
                                format!(
                                    "{}/zuu/rust/stdout/{}",
                                    current_dir()?.to_str().expect("failed to get current dir"),
                                    task.0
                                )
                                .as_str(),
                            )?)
                            .stderr(File::create(
                                format!(
                                    "{}/zuu/rust/stderr/{}",
                                    current_dir()?.to_str().expect("failed to get current dir"),
                                    task.0
                                )
                                .as_str(),
                            )?)
                            .spawn()?
                            .wait()?
                            .success()
                            .eq(&false)
                        {
                            if Path::new("/usr/bin/ranger").is_executable() {
                                let _ =
                                    Command::new("ranger").current_dir("zuu").spawn()?.wait()?;
                            }
                            return Err(anyhow::anyhow!(">> test failed"));
                        }
                    }
                    println!(">> rust code validated");
                }
                _ => {}
            }
        }
        Ok(())
    }
}
