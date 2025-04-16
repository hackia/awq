use crate::ask::asked;
use crate::run::Checker;
use anyhow::{anyhow, Error, Result};
use ignore::WalkBuilder;
use inquire::{Confirm, Select};
use serde::{Deserialize, Serialize};
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::Command;

pub const TEMPLATE: &str = include_str!("/usr/share/awq/templates/commit.txt");

pub const SCOPES: [&str; 67] = [
    "terraform",
    "first contact",
    "interstellar",
    "intergalactic",
    "exoplanet",
    "stellar nursery",
    "moon landing",
    "dark hole",
    "rogue planet",
    "asteroid",
    "nebula",
    "astrophysics",
    "cosmology",
    "eclipse",
    "planetary nebula",
    "white dwarf",
    "red giant",
    "neutron star",
    "gravity",
    "light speed",
    "pulsar",
    "telescope",
    "satellite",
    "probe",
    "spacecraft",
    "rocket",
    "space station",
    "orbit",
    "galaxy cluster",
    "comet",
    "meteor",
    "solar storm",
    "lunar transit",
    "perihelion",
    "void",
    "gravitation",
    "cosmic ray",
    "quantum",
    "hawking",
    "event horizon",
    "redshift",
    "quasar",
    "black hole",
    "dark matter",
    "dark energy",
    "dark star",
    "Kuiper belt",
    "Oort cloud",
    "Milky Way",
    "Andromeda",
    "supercluster",
    "multiverse",
    "antimatter",
    "dark flow",
    "cosmic microwave background",
    "gravitational wave",
    "magnetar",
    "brown dwarf",
    "blue giant",
    "cepheid variable",
    "singularity",
    "solar",
    "solar flare",
    "dwarf star",
    "dwarf planet",
    "aphelion",
    "regression",
];
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwqCommit {
    commit_type: String,
    message: String,
    why: String,
    summary: String,
}

pub fn scan(dir: &str) -> Result<Vec<PathBuf>, Error> {
    let mut x: Vec<PathBuf> = Vec::new();
    let walker = WalkBuilder::new(dir)
        .add_custom_ignore_filename(".gitignore")
        .add_custom_ignore_filename(".hgignore")
        .add_custom_ignore_filename(".svnignore")
        .add_custom_ignore_filename(".awqignore")
        .threads(num_cpus::get())
        .standard_filters(true)
        .build();

    for entry in walker.flatten() {
        x.push(entry.path().to_path_buf());
    }
    Ok(x.clone())
}

impl AwqCommit {
    pub fn new() -> Result<Self, Error> {
        create_dir_all("zuu")?;
        create_dir_all("zuu/stdout")?;
        create_dir_all("zuu/stderr")?;

        Ok(Self {
            message: String::new(),
            why: String::new(),
            summary: String::new(),
            commit_type: String::new(),
        })
    }

    fn set_commit_type(&mut self) {
        loop {
            let ct = Select::new("Select a commit scope:", SCOPES.to_vec())
                .prompt()
                .unwrap()
                .to_string();
            if ct.is_empty() {
                println!("Please select a commit scope.");
                continue;
            }
            self.commit_type = ct;
            break;
        }
    }
    pub fn save(&mut self) -> Result<(), Error> {
        Checker::new().check()?;
        if Confirm::new("show diff ?")
            .with_default(false)
            .prompt()?
            .eq(&true)
        {
            Command::new("git")
                .arg("diff")
                .arg("-p")
                .current_dir(".")
                .spawn()?
                .wait()?;
        } else {
            println!(">> No details shown.");
        }
        if Confirm::new("Do you confirm want to commit these changes?")
            .with_default(false)
            .prompt()?
            .eq(&false)
        {
            println!(">> Commit aborted.");
            return Ok(());
        }
        Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(".")
            .spawn()?
            .wait()?;
        self.set_commit_type();
        self.set_summary(asked("What is the summary of this commit?")?);
        self.set_why(asked("Why are you making this commit?")?);

        let commit = TEMPLATE
            .replace("%scope%", self.commit_type.as_str())
            .replace("%summary%", self.summary.as_str())
            .replace("%why%", self.why.as_str());
        if Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(commit.as_str())
            .current_dir(".")
            .spawn()?
            .wait()?
            .success()
        {
            Ok(())
        } else {
            Err(anyhow!("Failed to commit changes."))
        }
    }
    fn set_why(&mut self, why: String) {
        self.why = why;
    }
    fn set_summary(&mut self, summary: String) {
        self.summary = summary;
    }
}

pub fn commit() -> Result<(AwqCommit), Error> {
    match AwqCommit::new() {
        Ok(mut app) => {
            if let Err(e) = app.save() {
                return Err(e);
            }
            Ok(app)
        }
        Err(e) => Err(e),
    }
}
