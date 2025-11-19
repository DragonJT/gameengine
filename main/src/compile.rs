use crate::visual_scripting::*;
use std::fs::File;
use std::io::Write;
use std::process::*;

pub fn run(ui: &mut UI) {
    let node0 = Element::Node(0);
    let children = ui.get_children(node0);
    let value = ui.get_editable_text(children[0]).unwrap();

    let code = format!(
        r#"fn main() {{
        println!("{}");
    }}"#,
        value
    );

    let mut f =
        File::create("/home/jonathan/Documents/project/src/main.rs").expect("cant create file");
    f.write_all(code.as_bytes()).expect("cant write to file");

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
