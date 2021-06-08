use std::process::ExitStatus;
use std::sync::{Arc, Mutex};

use super::{normal, Error};
use tokio::process::Command;

/// Execute the given command in a secure isolated environment.
///
/// `stdout` and `stderr` is streamed line by line to the `output` closure,
/// which is called for each line that received.
pub async fn execute<O>(cmd: String, output: O) -> Result<ExitStatus, Error>
where
    O: Fn(String) -> Result<(), Error> + Clone + 'static,
{
    // Use Docker as base command
    let mut isolated_cmd = Command::new("docker");

    // Configure Docker and set a timeout for the command to run
    // TODO: configurable timeout
    // TODO: also handle a timeout fallback outside the actual container
    // TODO: map container UIDs to something above 10k
    let isolated_cmd = isolated_cmd
        .arg("run")
        .arg("--rm")
        .args(&["--cpus", "0.2"])
        // TODO: enable these memory limits once the warning is fixed
        // .args(&["--memory", "100m"])
        // .args(&["--kernel-memory", "25m"])
        // .args(&["--memory-swappiness", "0"])
        // .args(&["--device-read-bps", "/:50mb"])
        // .args(&["--device-write-bps", "/:50mb"])
        .args(&["--pids-limit", "64"])
        .args(&["--workdir", "/root"])
        .args(&["--restart", "no"])
        .args(&["--stop-timeout", "1"])
        .arg("risc-exec")
        .args(&["timeout", "--signal=SIGTERM", "--kill-after=305", "300"])
        .args(&["bash", "-c", &cmd]);

    // Execute the isolated command in the normal environment
    normal::execute(isolated_cmd, output).await
}

/// Execute the given command in a secure isolated environment.
///
/// The `stdout` and `stderr` is collected and returned with the future.
pub async fn execute_sync(cmd: String) -> Result<(String, ExitStatus), Error> {
    // Create a sharable buffer
    let buf = Arc::new(Mutex::new(String::new()));
    let buf_exec = buf.clone();

    // Execute the sed command, fill the buffer, stringify the buffer and return
    let status = execute(cmd, move |out| {
        buf_exec.lock().unwrap().push_str(&out);
        Ok(())
    })
    .await?;

    let buf = buf.lock().unwrap().to_owned();
    Ok((buf, status))
}
