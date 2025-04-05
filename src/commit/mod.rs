use crate::ask::asked;
use crate::run::Checker;
use anyhow::{Error, Result};
use chrono::Utc;
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::{Confirm, Select};
use orx_tree::{Collection, DynTree};
use serde::{Deserialize, Serialize};
use sha2::{digest::FixedOutput, Digest, Sha512};
use std::fs;
use std::fs::create_dir_all;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use std::{
    collections::HashMap,
    env::var,
    fs::File,
    io::{BufWriter, Write},
};
use tabled::builder::Builder;
use tabled::Table;

#[derive(Debug, Clone)]
pub enum ChangeType {
    Added,
    Removed,
    Modified,
}
#[derive(Default)]
pub struct Security {}

impl Security {
    pub const fn new() -> Self {
        Self {}
    }
    pub fn check_file(path: &Path) -> Result<(), Error> {
        if path.is_symlink() {
            return Err(anyhow::anyhow!("Symlink detected"));
        }

        if !path.exists() {
            return Err(anyhow::anyhow!("Path does not exist"));
        }

        if path.is_dir() {
            return Ok(()); // silently ignore directories
        }

        if path.starts_with("..") || path.is_absolute() {
            return Err(anyhow::anyhow!("Invalid path"));
        }

        if path.extension().is_none() || path.file_name().is_none() {
            return Err(anyhow::anyhow!("Missing name or extension"));
        }

        if fs::metadata(path)
            .map(|m| m.len() > 512 * 1024)
            .unwrap_or(true)
        {
            return Err(anyhow::anyhow!("File too large"));
        }

        if let Ok(content) = fs::read(path) {
            if content.iter().take(2048).any(|&b| b == 0) {
                return Err(anyhow::anyhow!("Binary file detected"));
            }

            if let Some(kind) = infer::get(&content) {
                if kind.mime_type().starts_with("image/") || kind.mime_type().starts_with("video/")
                {
                    return Err(anyhow::anyhow!("Media file detected"));
                }
            }
        } else {
            return Err(anyhow::anyhow!("Unable to read file"));
        }
        Ok(())
    }

    pub fn check_files(&self, files: &[PathBuf]) -> Vec<PathBuf> {
        files
            .iter()
            .filter_map(|path| {
                if Self::check_file(path).is_ok() {
                    Some(path.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwqId {
    previous: Option<String>,
    author: String,
    timestamp: String,
    version: u8,
    hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwqCommit {
    id: AwqId,
    commit_type: String,
    message: String,
    why: String,
    summary: String,
    environment: String,
    config: AwqConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffResult {
    pub sure: Vec<PathBuf>,
    pub not_sure: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct DiffForest {
    pub added: DynTree<PathBuf>,
    pub removed: DynTree<PathBuf>,
    pub modified: DynTree<PathBuf>,
    pub to_test: DynTree<PathBuf>,
}

pub fn get_sure_files() -> Vec<PathBuf> {
    WalkBuilder::new(".awq/tree")
        .standard_filters(true)
        .build()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.path().to_path_buf())
        .collect()
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
    Ok(x.to_vec())
}

impl DiffResult {
    pub fn stats(&self) -> std::io::Result<()> {
        let line_diffs: HashMap<PathBuf, Vec<String>> = Self::diff_by_lines(".awq/tree", ".")?;

        let mut builder: Builder = Builder::default();

        for (file, changes) in &line_diffs {
            let mut added = 0;
            let mut removed = 0;

            for line in changes {
                if line.starts_with("+ ") {
                    added += 1;
                } else if line.starts_with("- ") {
                    removed += 1;
                }
            }

            builder.push_record([
                String::new(),
                file.display().to_string(),
                added.to_string(),
                removed.to_string(),
            ]);
        }

        // Dernière ligne : total
        let table: Table = builder.build();
        println!("{}", table);

        Ok(())
    }

    pub fn print_summary_diff(&self) -> std::io::Result<()> {
        let line_diffs = Self::diff_by_lines(".awq/tree", ".")?;
        let mut total_added = 0;
        let mut total_removed = 0;
        let mut total_files = 0;

        for (file, changes) in &line_diffs {
            let mut added = 0;
            let mut removed = 0;

            for line in changes {
                if line.starts_with("+ ") {
                    added += 1;
                } else if line.starts_with("- ") {
                    removed += 1;
                }
            }

            total_files += 1;
            total_added += added;
            total_removed += removed;

            println!("* {:<40} [+{} -{}]", file.display(), added, removed);
        }

        println!(
            "\nTotal files: {} | \x1b[32m+{} lines\x1b[0m | \x1b[31m-{} lines\x1b[0m",
            total_files, total_added, total_removed
        );
        Ok(())
    }

    /// Compare deux dossiers ligne par ligne et retourne les différences
    pub fn diff_by_lines(
        dir_a: &str,
        dir_b: &str,
    ) -> std::io::Result<HashMap<PathBuf, Vec<String>>> {
        let mut diffs: HashMap<PathBuf, Vec<String>> = HashMap::new();

        for entry in WalkBuilder::new(dir_b)
            .standard_filters(true)
            .build()
            .filter(|e| e.clone().expect("msg").path().is_file())
        {
            let path_b = entry.expect("msg");
            let rel_path = path_b
                .path()
                .strip_prefix(dir_b)
                .unwrap_or_else(|_| path_b.path());
            let path_a = Path::new(dir_a).join(rel_path);

            let b_lines = fs::read_to_string(path_b.path())
                .unwrap_or_default()
                .lines()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            let a_lines = if path_a.exists() {
                fs::read_to_string(path_a)
                    .unwrap_or_default()
                    .lines()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };

            let mut file_diff: Vec<String> = Vec::new();

            // Détection des ajouts ou modifications ligne par ligne
            for (i, line_b) in b_lines.iter().enumerate() {
                if i >= a_lines.len() {
                    file_diff.push(format!("+ {}", line_b));
                } else if line_b != &a_lines[i] {
                    file_diff.push(format!("~ {}", line_b)); // modifié
                }
            }

            // Lignes supprimées (présentes avant, absentes maintenant)
            if b_lines.len() < a_lines.len() {
                for line in &a_lines[b_lines.len()..] {
                    file_diff.push(format!("- {}", line));
                }
            }

            if !file_diff.is_empty() {
                diffs.insert(rel_path.to_path_buf(), file_diff);
            }
        }
        Ok(diffs)
    }
}

pub fn copy_with_bar(source: &str, dest: &str) -> Result<()> {
    let source_path = Path::new(source);
    let dest_path = Path::new(dest);

    if source_path.exists().eq(&false) {
        return Err(anyhow::Error::new(std::io::Error::new(
            ErrorKind::NotFound,
            "source dir not found",
        )));
    }

    // Liste filtrée avec ignore support
    if let Ok(valid_entries) = scan(".") {
        let total_bytes: u64 = valid_entries
            .iter()
            .filter(|p| p.is_file())
            .map(|p| fs::metadata(p).map(|m| m.len()).unwrap_or(1))
            .sum();
        let pb = ProgressBar::new(total_bytes);
        pb.enable_steady_tick(Duration::from_millis(200));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        for path in &valid_entries {
            let rel = path.strip_prefix(source_path)?;
            let target = dest_path.join(rel);

            if path.is_dir() {
                fs::create_dir_all(&target)?;
            } else if path.is_file() {
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(path, &target)?;
                let size = fs::metadata(path)?.len();
                pb.inc(size);
                sleep(Duration::from_millis(250));
            }
        }
        pb.finish_with_message("Copie terminée.");
        return Ok(());
    }
    Err(anyhow::anyhow!(ErrorKind::InvalidInput))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwqConfig {
    pub language: String,
}

pub fn config() -> Result<AwqConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("awq.yml")?;
    let config: AwqConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}
impl AwqCommit {
    pub fn new() -> Result<Self, anyhow::Error> {
        let author: String = var("USER").unwrap_or_else(|_| "unknown".to_string());
        let timestamp: String = Utc::now().to_rfc3339();
        let version: u8 = 1;
        let hash: Option<String> = None;

        create_dir_all(".awq")?;
        create_dir_all(".awq/logs")?;
        create_dir_all(".awq/commits")?;
        create_dir_all(".awq/tree")?;
        create_dir_all(".awq/src")?;
        if let Ok(tree) = scan(".") {
            for t in &tree {
                let relative_path = t.strip_prefix(".")?; // ex: src/main.rs
                let destination = Path::new(".awq").join("src").join(relative_path);

                if t.is_dir() {
                    fs::create_dir_all(&destination)?;
                } else if t.is_file() {
                    if let Some(parent) = destination.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    if let Err(e) = Security::check_file(t) {
                        return Err(anyhow::anyhow!(e));
                    }
                    fs::copy(t, &destination)?;
                }
            }
        }
        if let Ok(tux) = config() {
            return Ok(Self {
                id: AwqId {
                    previous: None,
                    author,
                    timestamp,
                    version,
                    hash,
                },
                config: tux,
                message: String::new(),
                why: String::new(),
                summary: String::new(),
                environment: String::new(),
                commit_type: String::new(),
            });
        }
        Err(anyhow::anyhow!("missing config file"))
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
        descriptions.insert("cosmic ray", "docs: Add enlightening information.");
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
            "solar flare",
            "perf: Energy optimization (burst of energy).",
        );
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
        descriptions.insert("aphelion", "chore: Other change at the farthest point.");
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

    pub fn save(&mut self) -> Result<(), anyhow::Error> {
        Checker::new(self.config.clone()).check()?;

        let diff: HashMap<PathBuf, Vec<String>> =
            DiffResult::diff_by_lines(".awq/tree", ".awq/src")?;
        if Confirm::new("Show more details?")
            .with_default(false)
            .prompt()
            .unwrap()
            .eq(&true)
        {
            for (file, changes) in &diff {
                println!("\n# {}\n", file.display());
                for line in changes {
                    if line.contains("+") {
                        println!("\x1b[32m{line}\x1b[0m");
                    } else if line.contains("-") {
                        println!("\x1b[31m{line}\x1b[0m");
                    } else if line.contains("~") {
                        println!("\x1b[33m{line}\x1b[0m");
                    } else {
                        println!("\x1b[0m{line}\x1b[0m");
                    }
                }
            }
        } else {
            println!("No details shown.");
        }
        if Confirm::new("Do you confirm want to commit these changes?")
            .with_default(false)
            .prompt()
            .unwrap()
            .eq(&false)
        {
            println!("Commit aborted.");
            return Ok(());
        }
        self.set_commit_type();
        self.set_summary(asked("What is the summary of this commit?")?);
        self.set_why(asked("Why are you making this commit?")?);
        self.set_message(asked("What is the message of this commit?")?);
        self.set_environment(asked("What is the environment")?);
        self.set_hash(self.hash());
        self.set_previous(self.get_previous().unwrap_or_default());
        let mut out = BufWriter::new(File::create(format!(".awq/{}.awq", self.get_hash()))?);
        writeln!(out, "# awq commit")?;
        writeln!(out, "commit author: {}", self.get_author())?;
        writeln!(out, "commit type: {}", self.commit_type)?;
        writeln!(out, "summary: {}", self.summary)?;
        writeln!(out, "why: {}", self.why)?;
        writeln!(out, "message: {}", self.message)?;
        writeln!(out, "environment: {}", self.environment)?;
        writeln!(out, "author: {}", self.id.author)?;
        writeln!(out, "timestamp: {}", self.id.timestamp)?;
        writeln!(out, "version: {}", self.id.version)?;
        writeln!(out, "previous: {}", self.get_previous().unwrap_or_default())?;
        writeln!(out, "hash: {}", self.get_hash())?;
        out.flush()?;
        out.get_ref().sync_all()?;
        Ok(())
    }
    fn set_previous(&mut self, previous: String) {
        self.id.previous = Some(previous);
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
    fn set_environment(&mut self, environment: String) {
        self.environment = environment;
    }
    fn set_hash(&mut self, hash: String) {
        self.id.hash = Some(hash);
    }
    fn hash(&self) -> String {
        let data: String = vec![
            self.id.previous.clone().unwrap_or_default(),
            self.id.author.clone(),
            self.id.timestamp.clone(),
            self.id.version.to_string(),
            self.commit_type.clone(),
            self.message.clone(),
            self.why.clone(),
            self.summary.clone(),
            self.environment.clone(),
        ]
        .join("");
        let hasher = Sha512::new_with_prefix(data.as_bytes()); // Convert to &[u8]
        hex::encode(hasher.finalize_fixed())
    }

    fn get_hash(&self) -> String {
        self.id.hash.clone().unwrap_or_default()
    }
    fn get_previous(&self) -> Option<String> {
        self.id.previous.clone()
    }
    fn get_author(&self) -> String {
        self.id.author.clone()
    }
}
