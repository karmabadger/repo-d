use daemonize::Daemonize;
use fork::{daemon, Fork};
use std::{
    fs::File,
    process::{exit, Command},
};

pub fn start() {
    println!("Starting repod...");

    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    // let daemonize = Daemonize::new()
    //     // .pid_file("/tmp/test.pid") // Every method except `new` and `start`
    //     .chown_pid_file(true) // is optional, see `Daemonize` documentation
    //     // .working_directory("/tmp") // for default behaviour.
    //     .user("wenxuan27")
    //     // .group("daemon") // Group name
    //     // .group(2) // or group id.
    //     // .umask(0o777) // Set umask, `0o027` by default.
    //     .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
    //     .stderr(stderr); // Redirect stderr to `/tmp/daemon.err`.
    //                      // .privileged_action(|| "Executed before drop privileges");

    // match daemonize.start() {
    //     Ok(_) => println!("Success, daemonized"),
    //     Err(e) => eprintln!("Error, {}", e),
    // }

    if let Ok(Fork::Child) = daemon(false, false) {
        Command::new("sleep")
            .arg("3")
            .output()
            .expect("failed to execute process");
    }
    println!("repod started");
}
