use std::fs::File;
use std::io::Write;
use std::process::*;

pub fn run() {
    let mut f =
        File::create("/home/jonathan/Documents/project/src/main.rs").expect("cant create file");
    f.write_all(
        b"fn main(){
        println!(\"HelloWorld\");
    }",
    )
    .expect("cant write to file");

    // Run `cargo run`
    let mut status = Command::new("cargo")
        .arg("run")
        .current_dir("/home/jonathan/Documents/project/") // <-- set working directory
        .stdout(Stdio::inherit()) // <-- print stdout live
        .stderr(Stdio::inherit()) // <-- print stderr live
        .spawn()
        .expect("failed to run cargo");

    status.wait().expect("cargo run failed");

    println!("Done! Cargo status: {:?}", status);
}
