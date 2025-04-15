use crate::ask::asked;
use crate::run::Checker;
use anyhow::{anyhow, Error, Result};
use ignore::WalkBuilder;
use inquire::{Confirm, Select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::Command;

pub const API_CHANGE: &str = include_str!("/usr/share/awq/templates/api/changes.txt");

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
    fn get_awq_commit_descriptions(&self) -> HashMap<&'static str, &'static str> {
        let mut descriptions: HashMap<&str, &str> = HashMap::new();
        descriptions.insert("terraform", "feat: Add a new feature (planet management).");
        descriptions.insert(
            "first contact",
            "feat: Add a new feature (external interaction).",
        );
        descriptions.insert("interstellar", "feat: Add a large-scale feature.");
        descriptions.insert("intergalactic", "feat: Add a very ambitious feature.");
        descriptions.insert(
            "exoplanet",
            "feat: Add a feature external to the main system.",
        );
        descriptions.insert("stellar nursery", "feat: Add a feature under development.");
        descriptions.insert("moon landing", "feat: Add a major feature (exploit).");
        descriptions.insert("big bang", "feat: Major change introducing a new feature.");

        descriptions.insert("dark hole", "fix: Fix a bug (data loss).");
        descriptions.insert("rogue planet", "fix: Fix an unexpected behavior.");
        descriptions.insert("asteroid", "fix: Fix a small issue.");

        descriptions.insert(
            "nebula",
            "docs: Improve documentation (cloud of information).",
        );
        descriptions.insert(
            "astrophysics",
            "docs: High-level documentation about physics.",
        );
        descriptions.insert(
            "cosmology",
            "docs: High-level documentation about architecture.",
        );

        descriptions.insert("eclipse", "style: Change in appearance (visual theme).");
        descriptions.insert(
            "planetary nebula",
            "style: Visual organization of the code.",
        );

        descriptions.insert(
            "white dwarf",
            "refactor: Restructuring of a module (evolution).",
        );
        descriptions.insert(
            "red giant",
            "refactor: Significant refactoring of a part of the code.",
        );
        descriptions.insert(
            "neutron star",
            "refactor: Deep transformation of a component.",
        );
        descriptions.insert(
            "gravity",
            "refactor: Fundamental reorganization of the code.",
        );

        descriptions.insert("light speed", "perf: Significant speed improvement.");
        descriptions.insert(
            "pulsar",
            "perf: Performance improvement with regular emissions.",
        );
        descriptions.insert(
            "telescope",
            "test: Add tests (observation and verification).",
        );
        descriptions.insert("satellite", "test: Add monitoring tests.");
        descriptions.insert("probe", "test: Add exploratory tests.");

        descriptions.insert("spacecraft", "build: Update build infrastructure.");
        descriptions.insert("rocket", "build: Change related to the launch system.");
        descriptions.insert(
            "space station",
            "build: Configuration of the build environment.",
        );
        descriptions.insert(
            "orbit",
            "ci: Configuration of the continuous integration cycle.",
        );
        descriptions.insert(
            "galaxy cluster",
            "ci: Configuration of a set of CI systems.",
        );
        descriptions.insert("comet", "chore: Other change (cleanup).");
        descriptions.insert("meteor", "chore: Other minor change.");
        descriptions.insert("solar storm", "chore: Other potentially disruptive change.");
        descriptions.insert("lunar transit", "chore: Other transient change.");
        descriptions.insert("perihelion", "chore: Other change at the closest point.");
        descriptions.insert("void", "chore: Other change (removal).");
        descriptions.insert("gravitation", "chore: Other fundamental change.");
        descriptions.insert("cosmic ray", "chore: Other change (minor impact).");
        descriptions.insert("quantum", "chore: Other change (small scale).");
        descriptions.insert("hawking", "chore: Other change (theoretical idea).");
        descriptions.insert("event horizon", "chore: Other change (limit).");
        descriptions.insert("redshift", "chore: Other change (shift).");
        descriptions.insert("quasar", "chore: Other change (very luminous).");
        descriptions.insert("black hole", "chore: Other change (permanent removal?).");
        descriptions.insert(
            "dark matter",
            "chore: Other change invisible but with effect.",
        );
        descriptions.insert(
            "dark energy",
            "chore: Other change that accelerates something.",
        );
        descriptions.insert("dark star", "chore: Other invisible change.");
        descriptions.insert("Kuiper belt", "chore: Other change in a peripheral area.");
        descriptions.insert("Oort cloud", "chore: Other very distant change.");
        descriptions.insert("Milky Way", "chore: Other project-wide change.");
        descriptions.insert(
            "Andromeda",
            "chore: Other change related to an external project.",
        );
        descriptions.insert("supercluster", "chore: Other very large-scale change.");
        descriptions.insert("universe", "chore: Other fundamental change.");
        descriptions.insert(
            "multiverse",
            "chore: Other change affecting multiple aspects.",
        );
        descriptions.insert("antimatter", "chore: Other potentially destructive change.");
        descriptions.insert("dark flow", "chore: Other change with a hidden direction.");
        descriptions.insert(
            "cosmic microwave background",
            "chore: Other fundamental change (remnant).",
        );
        descriptions.insert(
            "gravitational wave",
            "chore: Other change with a perturbation.",
        );
        descriptions.insert("magnetar", "chore: Other change with a strong influence.");
        descriptions.insert("brown dwarf", "chore: Other change (almost a feature).");
        descriptions.insert("blue giant", "chore: Other significant change.");
        descriptions.insert(
            "cepheid variable",
            "chore: Other change with periodic variation.",
        );
        descriptions.insert("singularity", "chore: Other unique and specific change.");
        descriptions.insert("solar", "chore: Other change related to the main system.");
        descriptions.insert("solar flare", "chore: Other sudden and intense change.");
        descriptions.insert("dwarf star", "chore: Other small change.");
        descriptions.insert(
            "dwarf planet",
            "chore: Other change that looks like a feature but is smaller.",
        );
        descriptions.insert(
            "aphelion",
            "revert: Revert a previous commit (moving away).",
        );
        descriptions.insert("regression", "revert: Restore to an earlier version.");
        descriptions
    }

    fn set_commit_type(&mut self) {
        let descriptions: HashMap<&str, &str> = self.get_awq_commit_descriptions();

        let mut ty: Vec<String> = Vec::new();
        descriptions
            .iter()
            .for_each(|(k, v)| ty.push(format!("{k}: ({v})")));
        loop {
            let ct = Select::new("Select a commit type:", ty.to_vec())
                .prompt()
                .unwrap()
                .to_string();
            if ct.is_empty() {
                println!("Please select a commit type.");
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
        self.set_message(asked("What is the message of this commit?")?);

        let commit = API_CHANGE
            .replace("%type%", self.commit_type.as_str())
            .replace("%summary%", self.summary.as_str())
            .replace("%why%", self.why.as_str())
            .replace("%explain%", self.message.as_str());
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
    fn set_message(&mut self, message: String) {
        self.message = message;
    }
    fn set_why(&mut self, why: String) {
        self.why = why;
    }
    fn set_summary(&mut self, summary: String) {
        self.summary = summary;
    }
}
