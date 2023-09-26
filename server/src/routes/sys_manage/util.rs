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
