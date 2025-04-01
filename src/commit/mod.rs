use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwqId {
    pub previous: Option<String>,
    pub author: String,
    pub timestamp: String,
    pub environment: String,
    pub message: String,
    pub version: u8,
}

impl AwqId {
    /// Calcule le hash de l'ensemble des métadonnées
    pub fn compute_hash(&self, source_files: &[(&str, &str)]) -> String {
        let mut hasher = Sha256::new();

        if let Some(prev) = &self.previous {
            hasher.update(prev.as_bytes());
        }
        hasher.update(self.author.as_bytes());
        hasher.update(self.timestamp.to_string());
        hasher.update(self.environment.as_bytes());
        hasher.update(self.message.as_bytes());
        hasher.update(&[self.version]);

        // Hash du contenu des fichiers triés
        let mut sorted = source_files.to_vec();
        sorted.sort_by_key(|(path, _)| *path);
        for (path, content) in sorted {
            hasher.update(path.as_bytes());
            hasher.update(content.as_bytes());
        }
        let hash = hasher.finalize();
        hex::encode(hash)
    }
}

#[cfg(test)]
mod test {
    use super::AwqId;
    use soul::anima::{soul::Testing, unit::Unit};
    use std::process::ExitCode;
    #[test]
    pub fn idea() -> ExitCode {
        let mut u: Unit = Unit::new();
        u.group("hash must be match expected", |u| {
            let o: AwqId = AwqId {
                previous: Some("abc123".to_string()),
                author: "Eytukan".to_string(),
                timestamp: "2025-04-01T12:00:00Z".to_string(),
                environment: "prod".to_string(),
                message: "initial commit".into(),
                version: 1,
            };

            let ao: AwqId = AwqId {
                previous: Some("adez".to_string()),
                author: "Eytukan".to_string(),
                timestamp: "2025-04-01T12:21:00Z".to_string(),
                environment: "prod".to_string(),
                message: "initials commit".into(),
                version: 1,
            };
            let files1: Vec<(&str, &str)> = vec![("file.rs", "let x = 1;")];
            let files2: Vec<(&str, &str)> = vec![("file.rs", "let x = 2;")];

            let first_hash: String = o.compute_hash(&files1);
            let first_hash_copy: String = o.compute_hash(&files1);

            let second_hash: String = ao.compute_hash(&files2);

            u.eq(
                "hash must be equals on same files",
                vec![first_hash_copy.to_string(), first_hash.to_string()],
                first_hash_copy.to_string(),
            )
            .eq(
                "hash must be equals on same files",
                vec![first_hash_copy.to_string(), first_hash.to_string()],
                first_hash.to_string(),
            )
            .ne(
                "hash must be differents on changes",
                vec![first_hash.to_string(), first_hash_copy.to_string()],
                second_hash.to_string(),
            )
        })
        .run()
    }
}
