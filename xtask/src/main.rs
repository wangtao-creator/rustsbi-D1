


use clap::{clap_app, crate_authors, crate_description, crate_version};
use std::{env, fs, io::{Seek, SeekFrom, Write}, path::{Path, PathBuf}, process::{self, Command}};

#[derive(Debug)]
struct XtaskEnv {
    compile_mode: CompileMode,
}

#[derive(Debug)]
enum CompileMode {
    Debug,
    Release
}

const DEFAULT_TARGET: &'static str = "riscv64imac-unknown-none-elf";

fn main() {
    let matches = clap_app!(xtask =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@subcommand make =>
            (about: "Build project")
            (@arg release: --release "Build artifacts in release mode, with optimizations")
        )
        (@subcommand D1 =>
            (about: "Run project on actual board")
            (@arg release: --release "Build artifacts in release mode, with optimizations")
        )
        (@subcommand detect =>
            (about: "Detect target serial port")
        )
        (@subcommand asm =>
            (about: "View asm code for project")
        )
        (@subcommand size =>
            (about: "View size for project")
        )
    ).get_matches();
    let mut xtask_env = XtaskEnv {
        compile_mode: CompileMode::Debug,
    };
    println!("xtask: mode: {:?}", xtask_env.compile_mode);
    
    if let Some(matches) = matches.subcommand_matches("D1") {

        if matches.is_present("release") {
            xtask_env.compile_mode = CompileMode::Release;
        }

        xtask_build_sbi(&xtask_env);
        xtask_binary_sbi(&xtask_env);
        xtask_build_test_kernel(&xtask_env);
        xtask_binary_test_kernel(&xtask_env);
        xtask_fuse_binary(&xtask_env);
        xtask_run_D1(&xtask_env);
    } else if let Some(_matches) = matches.subcommand_matches("make") {
        xtask_build_sbi(&xtask_env);
        xtask_binary_sbi(&xtask_env);
    } else {
        println!("Use `cargo D1` to run, `cargo xtask --help` for help")
    }
}



/*
xfel ddr ddr3
......
*/
#[warn(non_snake_case)]
fn xtask_run_D1(xtask_env: &XtaskEnv) {
    let status = Command::new("xfel")
    .current_dir(project_root())
    .arg("ddr")
    .arg("ddr3")
    .status().unwrap();
    if !status.success() {
        panic!("run D1 failed")
    }
    let status = Command::new("xfel")
    .current_dir(project_root().join("xtask"))
    .arg("write")
    .arg("0x40000000")
    .arg(dist_dir(xtask_env).join("D1-fused.bin"))
    .status().unwrap();
    if !status.success() {
        panic!("run D1 failed")
    }
    let status = Command::new("xfel")
    .arg("exec")
    .arg("0x40000000")
    .status().unwrap();
    if !status.success() {
        panic!("run D1 failed")
    }
}

fn xtask_build_sbi(xtask_env: &XtaskEnv) {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(cargo);
    command.current_dir(project_root().join("rustsbi-D1"));
    command.arg("build");
    match xtask_env.compile_mode {
        CompileMode::Debug => {},
        CompileMode::Release => { command.arg("--release"); },
    }
    command.args(&["--package", "rustsbi-D1"]);
    command.args(&["--target", DEFAULT_TARGET]);
    let status = command
        .status().unwrap();
    if !status.success() {
        println!("cargo build failed");
        process::exit(1);
    }
}

fn xtask_binary_sbi(xtask_env: &XtaskEnv) {
    let objcopy = "rust-objcopy";
    let status = Command::new(objcopy)
        .current_dir(dist_dir(xtask_env))
        .arg("rustsbi-D1")
        .arg("--binary-architecture=riscv64")
        .arg("--strip-all")
        .args(&["-O", "binary", "rustsbi-D1.bin"])
        .status().unwrap();

    if !status.success() {
        println!("objcopy binary failed");
        process::exit(1);
    }
}

fn xtask_build_test_kernel(xtask_env: &XtaskEnv) {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(cargo);
    command.current_dir(project_root().join("test-kernel"));
    command.arg("build");
    match xtask_env.compile_mode {
        CompileMode::Debug => {},
        CompileMode::Release => { command.arg("--release"); },
    }
    command.args(&["--package", "test-kernel"]);
    command.args(&["--target", DEFAULT_TARGET]);
    let status = command
        .status().unwrap();
    if !status.success() {
        println!("cargo build failed");
        process::exit(1);
    }
}

fn xtask_binary_test_kernel(xtask_env: &XtaskEnv) {
    let objcopy = "rust-objcopy";
    let status = Command::new(objcopy)
        .current_dir(dist_dir(xtask_env))
        .arg("test-kernel")
        .arg("--binary-architecture=riscv64")
        .arg("--strip-all")
        .args(&["-O", "binary", "test-kernel.bin"])
        .status().unwrap();

    if !status.success() {
        println!("objcopy binary failed");
        process::exit(1);
    }
}

fn xtask_fuse_binary(xtask_env: &XtaskEnv) {
    let sbi_binary_path = dist_dir(xtask_env).join("rustsbi-D1.bin");
    let test_kernel_binary_path = dist_dir(xtask_env).join("test-kernel.bin");
    let output_path = dist_dir(xtask_env).join("D1-fused.bin");
    let offset = 0x200000;//0X40200000
    fs::copy(sbi_binary_path, &output_path).expect("copy sbi base");
    let mut output = fs::OpenOptions::new().read(true).write(true).open(output_path)
        .expect("open output file");
    let buf = fs::read(test_kernel_binary_path).expect("read kernel binary");
    output.seek(SeekFrom::Start(offset)).expect("seek to offset");
    output.write(&buf).expect("write output");
}

fn dist_dir(xtask_env: &XtaskEnv) -> PathBuf {
    let mut path_buf = project_root().join("target").join(DEFAULT_TARGET);
    path_buf = match xtask_env.compile_mode {
        CompileMode::Debug => path_buf.join("debug"),
        CompileMode::Release => path_buf.join("release"),
    };
    path_buf
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
