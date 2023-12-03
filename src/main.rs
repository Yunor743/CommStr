use structopt::StructOpt;
use std::fs;
use tempfile::TempDir;
use std::process::{Command, exit};
use std::error::Error;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read};
use std::str;


#[derive(StructOpt, Debug)]
#[structopt(name = "commstr", about = "Common strings")]
struct Opt {
    #[structopt(short, long, help = "-d / --directory <folder path>")]
    help: bool,

    #[structopt(short, long, help = "common strings of the samples in the target folder")]
    directory: Option<String>,
}

fn calculate_sha256(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut hasher = Sha256::new();

    let mut buffer = [0; 4096];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash_result = hasher.finalize();

    let hash_hex = hash_result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

    Ok(hash_hex)
}

fn exec(cmd: &str) -> Result<String, Box<dyn Error>> {
    let mut sh = Command::new("sh");
    let mut command = sh.arg("-c").arg(cmd);
    let output = command.output()?;

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout)?;
        Ok(stdout.to_string())
    } else {
        let stderr = str::from_utf8(&output.stderr)?;
        Err(format!("command error ({}): {}", output.status.code().unwrap_or_default(), stderr).into())
    }
}


fn main() {
    let opt = Opt::from_args();

    if opt.help {
        Opt::clap().print_long_help();
    } else if let Some(directory) = opt.directory {
        match fs::read_dir(directory.clone()) {
            Ok(entries) => {
                let temp_dir = TempDir::new().expect("could not create temp dir");
                let tmpdir_path = temp_dir.path();

                for entry in entries {
                    if let Ok(entry) = entry {
                        let filename = entry.file_name().to_string_lossy().to_string();

                        let file_path =  format!("{}/{}", directory, filename);
                        let hash = calculate_sha256(&file_path).expect("SHA-256 sum error");

                        let command1 = format!("bash -c \'strings {} > {}/{}.str\'", file_path, tmpdir_path.display(), hash);
                        let _ = exec(&command1);
                    }
                }

                // execute a command
                // println!("{}", exec(&format!("ls {}", tmpdir_path.display())).expect("error"));

                // compute common
                let mut strings = exec(&format!("ls {}", tmpdir_path.display())).expect("error");

                if strings.lines().count() < 2 {
                    println!("Not enough files !");
                    exit(1)
                }

                let first_line = strings.lines().next().unwrap();
                let second_line = strings.lines().nth(1).unwrap();
                let head = format!("comm -12 {} {}", first_line, second_line);
                let mut payload = head.clone();

                if strings.lines().count() > 2 {
                    let result: Vec<String> = strings.lines().skip(2)
                        .map(|ligne| format!(" | comm -12 - {}", ligne))
                        .collect();
        

                    payload = format!("{} {}", head, result.join(" "))
                }

                let cmd = format!("cd {} && {} | sort -r | uniq", tmpdir_path.display(), payload);
                let output = exec(&cmd);
                println!("{}", output.expect("msg"));
            }
            Err(e) => {
                eprintln!("Error while reading folder : {:?}", e);
            }
        }
    }
}
