use std::{
    error::Error,
    io::Write,
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup command
    let mut command = Command::new("python");
    command.args(["-u", "-c", "from main import start; start()"]);
    command.current_dir(std::env::current_dir()?.join("python"));

    // Pipe stdin so we can issue commands to tell Python what we want it to do
    command.stdin(Stdio::piped());

    // Spawn the process & take ownership of stdin
    let mut child = command.spawn()?;
    let mut stdin = child.stdin.take().unwrap();

    // Give a second to make 100% sure Python has started
    sleep(Duration::from_secs(1));

    // Issue start command to Python, wait 5 seconds then tell it to stop

    println!("Writing start");
    stdin.write_all(b"start | \n")?;

    sleep(Duration::from_secs(5));

    println!("Writing stop");
    stdin.write_all(b"stop | \n")?;

    // Make sure the process has exited before we exit
    child.wait()?;

    Ok(())
}
