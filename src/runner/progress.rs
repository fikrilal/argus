use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::Arc;
use std::time::Duration;

/// Coordinates live terminal progress reporting for concurrent agent execution.
#[derive(Clone)]
pub struct SwarmProgressTracker {
    mp: Arc<MultiProgress>,
    spinner_style: ProgressStyle,
}

impl std::fmt::Debug for SwarmProgressTracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SwarmProgressTracker")
            .finish_non_exhaustive()
    }
}

impl SwarmProgressTracker {
    /// Creates a new progress tracker with styled multi-spinners.
    pub fn new() -> Self {
        let mp = Arc::new(MultiProgress::new());

        let spinner_style = ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
            .template("{spinner:.cyan} {wide_msg}")
            .unwrap_or_else(|_| ProgressStyle::default_spinner());

        Self { mp, spinner_style }
    }

    /// Registers and starts a progress spinner for an individual agent persona.
    pub fn start_agent_spinner(&self, persona_name: &str) -> AgentSpinner {
        let pb = self.mp.add(ProgressBar::new_spinner());
        pb.set_style(self.spinner_style.clone());
        pb.enable_steady_tick(Duration::from_millis(80));
        pb.set_message(format!(
            "{:<30} {}",
            persona_name.bold(),
            "auditing...".dimmed()
        ));

        AgentSpinner {
            pb,
            persona_name: persona_name.to_string(),
        }
    }
}

impl Default for SwarmProgressTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle to an active progress spinner for a single agent.
#[derive(Debug)]
pub struct AgentSpinner {
    pb: ProgressBar,
    persona_name: String,
}

impl AgentSpinner {
    /// Updates the status message displayed next to the agent name.
    pub fn update_status(&self, status: &str) {
        self.pb.set_message(format!(
            "{:<30} {}",
            self.persona_name.bold(),
            status.dimmed()
        ));
    }

    /// Finishes the spinner indicating successful completion.
    pub fn complete(&self, duration: Duration) {
        let secs = duration.as_secs_f32();
        self.pb.finish_with_message(format!(
            "{:<30} {} {}",
            self.persona_name.bold(),
            "✔ completed".bold().green(),
            format!("({secs:.1}s)").dimmed()
        ));
    }

    /// Finishes the spinner indicating an error or timeout.
    pub fn fail(&self, error_message: &str) {
        self.pb.finish_with_message(format!(
            "{:<30} {} {}",
            self.persona_name.bold(),
            "✖ failed".bold().red(),
            format!("({})", error_message.trim()).dimmed()
        ));
    }
}
