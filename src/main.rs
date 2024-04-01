use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use gcode::{Commands, Subcommands};
use file_mng::{unzip, FilesEnv};

mod file_mng;
mod gcode;
mod screen;

fn main() {
    let f_mng = FilesEnv::new("/home/bruno/Projects/dlp_printer/", "temp", "uploads");
    let project_name = "Test_KeySwitch.stl_0.05_6_2024_03_28_22_10_00.zip";

    let work_dir = f_mng.get_workdir_path();
    let storage_dir = f_mng.get_storagedir_path();
    let gcode_path = f_mng.get_gcode_path();

    let project_file = Path::new(&storage_dir).join(project_name);
    unzip(project_file.to_str().unwrap(), work_dir.to_str().unwrap());

    let scr = screen::handler::new();
    let scr_weak_clone = Arc::clone(&scr.weak);

    thread::spawn(move || {

        let commands = gcode::parse_from_file(gcode_path.to_str().unwrap());

        if let Ok(weak_ref) = scr_weak_clone.lock() {
            for cmd in commands {
                match cmd {
                    Commands::M6054 { path: img_name } => {
                        let path_img = Path::new(&work_dir).join(img_name);

                        screen::handler::set(weak_ref.to_owned(), path_img.to_string_lossy().to_string());
                    },
                    Commands::G4 {time} => {
                        match time {
                            Subcommands::P(s) => {
                                let time = s.round() as u64;
                                println!("Wait: {}s", time);
                                thread::sleep(Duration::from_secs(time));
                            },
                            Subcommands::S(ms) => {
                                let time = ms.round() as u64;
                                println!("Wait: {}ms", time);
                                thread::sleep(Duration::from_millis(time));
                            },
                            _=> {}
                        }
                    },
                    c => comand_exec(c)
                }
            }
        }
        
    });

    println!("App running!!");
    screen::handler::run(scr.app);
}

fn comand_exec(command: Commands) {
    match command {
        Commands::G0 { axes, subcommands } => gcode::validate::print(axes, subcommands),
        Commands::G1 { axes, subcommands } => gcode::validate::print(axes, subcommands),
        Commands::M106 { pwm,} => {
            let mut p = Vec::new();
            p.push(pwm);
            gcode::validate::print(None, Some(p));
        },
        _ => {}
    } 
}