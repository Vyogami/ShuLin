use std::io;
use std::process::Stdio;
use tokio::process::Command;

pub async fn toggle_service(name: &str, toggle: bool) -> Result<(), std::io::Error> {
    let mut cmd = Command::new("systemctl");

    if toggle {
        cmd.args(["enable", "--now"]);
    } else {
        cmd.args(["disable", "--now"]);
    }

    cmd.arg(name);

    cmd.output().await?;

    Ok(())
}

pub async fn is_active(unit: &str) -> Result<bool, std::io::Error> {
    let mut cmd = Command::new("systemctl");
    cmd.arg("is-active").arg(unit);
    cmd.stdout(Stdio::piped());

    Ok(std::str::from_utf8(&cmd.output().await?.stdout)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF8 in stdout"))?
        .trim_end()
        .eq("active"))
}
