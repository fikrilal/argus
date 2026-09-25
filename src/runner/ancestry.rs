/// Detects whether the current process was spawned from within an active Argus audit session.
///
/// Prevents recursive multi-agent swarm execution that exhausts CPU/threads and triggers timeouts.
#[must_use]
pub fn is_recursive_audit() -> bool {
    // 1. Primary check: Environment variable propagated down to child subprocesses
    if std::env::var("ARGUS_ACTIVE_AUDIT").is_ok() {
        return true;
    }

    // 2. Linux OS process hierarchy traversal fallback via `/proc`
    #[cfg(target_os = "linux")]
    {
        if check_linux_proc_ancestry() {
            return true;
        }
    }

    false
}

#[cfg(target_os = "linux")]
fn check_linux_proc_ancestry() -> bool {
    let mut current_pid = std::process::id();
    while current_pid > 1 {
        let stat_path = format!("/proc/{current_pid}/stat");
        let Ok(stat) = std::fs::read_to_string(&stat_path) else {
            break;
        };

        // Linux `/proc/[pid]/stat` format: `pid (comm) state ppid ...`
        // `comm` can contain arbitrary spaces and parentheses, so find the last `)`
        let Some(close_paren) = stat.rfind(')') else {
            break;
        };

        let rest = &stat[close_paren + 1..];
        let mut fields = rest.split_whitespace();
        let _state = fields.next();
        let Some(parent_pid_str) = fields.next() else {
            break;
        };

        let Ok(parent_pid) = parent_pid_str.parse::<u32>() else {
            break;
        };

        if parent_pid <= 1 {
            break;
        }

        let parent_comm_path = format!("/proc/{parent_pid}/comm");
        if let Ok(comm) = std::fs::read_to_string(&parent_comm_path)
            && comm.trim() == "argus"
        {
            return true;
        }

        current_pid = parent_pid;
    }

    false
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    #[cfg(target_os = "linux")]
    #[test]
    fn test_proc_stat_parsing_handles_spaces_in_comm() {
        let mock_stat = "1234 (cargo test runner) S 5678 1234 1234 0 -1 4194304";
        let close_paren = mock_stat.rfind(')').unwrap();
        let rest = &mock_stat[close_paren + 1..];
        let mut fields = rest.split_whitespace();
        let state = fields.next().unwrap();
        let parent_pid: u32 = fields.next().unwrap().parse().unwrap();

        assert_eq!(state, "S");
        assert_eq!(parent_pid, 5678);
    }
}
