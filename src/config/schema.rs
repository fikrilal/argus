use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Root configuration for Argus loaded from `.argus/config.yaml`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgusConfig {
    /// Schema configuration version (default: 1)
    #[serde(default = "default_version")]
    pub version: u32,

    /// Project identifier or name
    #[serde(default = "default_project")]
    pub project: String,

    /// Specification (SFD) document configuration
    #[serde(default)]
    pub sfd: SfdConfig,

    /// Model tier selections for cost/intelligence optimization
    #[serde(default)]
    pub models: ModelTiersConfig,

    /// Pre-configured squads mapping squad name to a list of persona names
    #[serde(default = "default_squads")]
    pub squads: HashMap<String, Vec<String>>,
}

fn default_version() -> u32 {
    1
}

fn default_project() -> String {
    "default-project".to_string()
}

/// Configuration for System Functional Design (SFD) documents
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SfdConfig {
    /// Directory containing Markdown SFD specifications
    #[serde(default = "default_sfd_dir")]
    pub dir: PathBuf,

    /// Name or path of the currently active SFD file
    #[serde(default)]
    pub active: Option<PathBuf>,
}

fn default_sfd_dir() -> PathBuf {
    PathBuf::from(".argus/context/sfd")
}

impl Default for SfdConfig {
    fn default() -> Self {
        Self {
            dir: default_sfd_dir(),
            active: None,
        }
    }
}

/// Model tier mappings for agent execution
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelTiersConfig {
    /// Fast model for simple scans and input boundary checks
    #[serde(default = "default_fast_model")]
    pub fast: String,

    /// Standard model for general adversarial evaluation
    #[serde(default = "default_standard_model")]
    pub standard: String,

    /// High-reasoning model for complex SFD cross-referencing and state invariant analysis
    #[serde(default = "default_deep_model")]
    pub deep: String,
}

fn default_fast_model() -> String {
    "google/gemini-2.5-flash".to_string()
}

fn default_standard_model() -> String {
    "anthropic/claude-3-7-sonnet".to_string()
}

fn default_deep_model() -> String {
    "anthropic/claude-3-7-sonnet:high".to_string()
}

impl Default for ModelTiersConfig {
    fn default() -> Self {
        Self {
            fast: default_fast_model(),
            standard: default_standard_model(),
            deep: default_deep_model(),
        }
    }
}

fn default_squads() -> HashMap<String, Vec<String>> {
    let mut squads = HashMap::new();

    squads.insert(
        "forms".to_string(),
        vec![
            "id-regulatory-sentinel".to_string(),
            "form-boundary-saboteur".to_string(),
            "cascade-dropdown-glitcher".to_string(),
        ],
    );

    squads.insert(
        "state".to_string(),
        vec![
            "stock-ledger-auditor".to_string(),
            "orphan-cascade-hunter".to_string(),
        ],
    );

    squads.insert(
        "sync".to_string(),
        vec![
            "payload-pessimist".to_string(),
            "sync-deadlock-guard".to_string(),
            "hardware-sensor-adversary".to_string(),
        ],
    );

    squads.insert(
        "spec".to_string(),
        vec![
            "sfd-clause-detective".to_string(),
            "rbac-identity-gatekeeper".to_string(),
        ],
    );

    squads.insert(
        "all".to_string(),
        vec![
            "sfd-clause-detective".to_string(),
            "id-regulatory-sentinel".to_string(),
            "rbac-identity-gatekeeper".to_string(),
            "form-boundary-saboteur".to_string(),
            "cascade-dropdown-glitcher".to_string(),
            "concurrency-double-tapper".to_string(),
            "stock-ledger-auditor".to_string(),
            "orphan-cascade-hunter".to_string(),
            "payload-pessimist".to_string(),
            "sync-deadlock-guard".to_string(),
            "hardware-sensor-adversary".to_string(),
        ],
    );

    squads
}

impl Default for ArgusConfig {
    fn default() -> Self {
        Self {
            version: default_version(),
            project: default_project(),
            sfd: SfdConfig::default(),
            models: ModelTiersConfig::default(),
            squads: default_squads(),
        }
    }
}
