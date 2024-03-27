use std::process::Command;

fn main() {
    Command::new("deno")
        .args(["task", "build"])
        .spawn()
        .unwrap();

    println!("cargo:rerun-if-changed=./build.rs");
    println!("cargo:rerun-if-changed=./build.ts");
}
