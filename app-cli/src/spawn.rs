use std::{io, path::PathBuf, process::ExitStatus};

pub fn spawn_gui(args: Vec<String>) -> io::Result<ExitStatus> {
    let exe_dir: PathBuf = std::env::current_exe()?
        .parent()
        .expect("executable should have a parent directory")
        .to_path_buf();

    let program = if cfg!(target_os = "windows") {
        exe_dir.join("app-gui.exe")
    } else {
        exe_dir.join("app-gui")
    };

    let spawn = std::process::Command::new(&program).args(args).spawn();
    let mut instance = match spawn {
        Ok(instance) => instance,
        Err(e) => {
            eprintln!("Failed to spawn instance: {}", e);
            std::process::exit(1);
        }
    };
    instance.wait()
}
