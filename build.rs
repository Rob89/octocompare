use std::process::Output;

fn main() {
    println!("cargo:rerun-if-changed=src/ui/");

    let dir: String = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let input = format!("{dir}/src/ui/style.css");
    let output = format!("{dir}/assets/style.css");

    let result: Output;

    if cfg!(windows) {
        let cmd = format!("tailwindcss --yes -i {} -o {}", input, output);
        result = std::process::Command::new("cmd")
            .args(["/C", &cmd])
            .output()
            .expect("Unable to generate css");
    } else {
        result = std::process::Command::new("npx")
            .args(["--yes", "tailwindcss", "-i", &input, "-o", &output])
            .output()
            .expect("Unable to generate css");
    }

    if !result.status.success() {
        let error = String::from_utf8_lossy(&result.stderr);
        println!("cargo:warning=Unable to build CSS !");
        println!("cargo:warning=Output: {error}");
    }
}
