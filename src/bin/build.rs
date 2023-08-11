fn main() {
    let cwd = std::env::current_dir().unwrap();

    let final_dir = "ffi-out";

    // check if the directory exists, if not create it
    let final_dir = cwd.join(final_dir);
    if !final_dir.exists() {
        std::fs::create_dir(&final_dir).unwrap();
    }

    // a map of the file to copy and the new name
    let files_to_copy = [
        (
            "aarch64-unknown-linux-gnu/release/libdecode_struct.so",
            "libdecode_struct_aarch.so",
            "aarch64",
        ),
        (
            "release/libdecode_struct.dylib",
            "libdecode_struct.dylib",
            "mac",
        ),
        (
            "x86_64-unknown-linux-gnu/release/libdecode_struct.so",
            "libdecode_struct_x86_64.so",
            "x86_64",
        ),
    ];

    // copy the files to the final directory, renaming them in the process. If the file does not exist, log and continue
    for (file, new_name, arch) in files_to_copy.iter() {
        let file = cwd.join("target").join(file);
        let new_name = final_dir.join(new_name);

        if !file.exists() {
            println!("file {} does not exist", file.display());
        }

        // delete the file if it exists
        if file.exists() {
            std::fs::remove_file(&file).unwrap();
        }

        run_build_script(arch);

        std::fs::copy(&file, &new_name).unwrap();
    }

    let header_file = std::env::current_dir().unwrap().join("def.h");

    if header_file.exists() {
        std::fs::copy(&header_file, &final_dir.join("def.h")).unwrap();
    }

    println!("{}", final_dir.display());
}

fn run_build_script(arch: &str) {
    let cwd = std::env::current_dir().unwrap();

    if arch == "mac" {
        println!("building for {} 🎁", arch);

        let mut command = std::process::Command::new("cargo");
        command.current_dir(&cwd).arg("build").arg("--release");

        let output = command.output().unwrap();

        if !output.status.success() {
            println!("build failed for {}", arch);
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }

    if arch == "aarch64" {
        println!("building for {} 🎁", arch);

        // cross build --target aarch64-unknown-linux-gnu -r
        let mut command = std::process::Command::new("cross");

        command
            .current_dir(&cwd)
            .arg("build")
            .arg("--target")
            .arg("aarch64-unknown-linux-gnu")
            .arg("-r");

        let output = command.output().unwrap();

        if !output.status.success() {
            println!("build failed for {}", arch);
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }

    if arch == "x86_64" {
        println!("building for {} 🎁", arch);

        // cross build --target x86_64-unknown-linux-gnu -r
        let mut command = std::process::Command::new("cross");

        command
            .current_dir(&cwd)
            .arg("build")
            .arg("--target")
            .arg("x86_64-unknown-linux-gnu")
            .arg("-r");

        let output = command.output().unwrap();

        if !output.status.success() {
            println!("build failed for {}", arch);
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
