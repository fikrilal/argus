pub mod finding;
pub mod markdown;
pub mod synthesizer;
pub mod terminal;

#[allow(unused_imports)]
pub use finding::{Finding, Severity, parse_agent_findings};
#[allow(unused_imports)]
pub use markdown::{generate_markdown_report, write_markdown_report};
#[allow(unused_imports)]
pub use synthesizer::{AgentReport, AuditSynthesis, synthesize_reports};
#[allow(unused_imports)]
pub use terminal::render_terminal_dashboard;
