use std::{
    fmt,
    fs,
    path::Path,
    process
};

use crate::utils;

pub mod common;
pub mod target;

use common::AdvancedBuildChoice;

pub const WELCOME_MESSAGE: &'static str =
    r"Welcome to the Project Lightsaber Build System!

This will build Project Lightsaber, the bootloader, kernel, and other dependencies / crates
that are required to fully build the Project; and also copy the built files to the `build`
directory, as if the Project has been successfully built.
    ";

#[derive(Clone)]
pub struct BuildOptions {
    pub profile: Option<(ProfileOptions, OptimizationLevel)>,
    pub target_architecture: Option<TargetArchitecture>,
    pub target_triple: Option<target::TargetTriple>
}

#[derive(Clone)]
pub enum OptimizationLevel {
    NoOptimizations,
    BasicOptimizations,
    SomeOptimizations,
    AllOptimizations,
    BinarySizeOptimization,
    BinarySizeOptimizationLoopVectorizationOff
}

impl fmt::Display for OptimizationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NoOptimizations => write!(f, "0"),
            Self::BasicOptimizations => write!(f, "1"),
            Self::SomeOptimizations => write!(f, "2"),
            Self::AllOptimizations => write!(f, "3"),
            Self::BinarySizeOptimization => write!(f, "s"),
            Self::BinarySizeOptimizationLoopVectorizationOff => write!(f, "z")
        }
    }
}

impl fmt::Debug for OptimizationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NoOptimizations => write!(f, "0 - No optimizations."),
            Self::BasicOptimizations => write!(f, "1 - Basic optimizations only."),
            Self::SomeOptimizations => write!(f, "2 - Some optimizations only."),
            Self::AllOptimizations => write!(f, "3 - All optimzations."),
            Self::BinarySizeOptimization => write!(f, "s - Optimize for binary size."),
            Self::BinarySizeOptimizationLoopVectorizationOff => write!(f, "z - Optimize for binary size; loop vectorization is also turned off.")
        }
    }
}

#[derive(Clone)]
pub enum ProfileOptions {
    Debug,
    Release
}

impl fmt::Display for ProfileOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Debug => write!(f, "debug"),
            Self::Release => write!(f, "release")
        }
    }
}

#[derive(Clone)]
pub enum TargetArchitecture {
    X86_64,
    Aarch64
}

impl fmt::Display for TargetArchitecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::X86_64 => write!(f, "x86_64"),
            Self::Aarch64 => write!(f, "aarch64")
        }
    }
}

pub fn build(mut options: BuildOptions) -> anyhow::Result<utils::ExitCode> {
    println!("{}", WELCOME_MESSAGE);

    loop {
        println!("{}", current_build_options(&options));

        match common::advanced_build()? {
            AdvancedBuildChoice::ProceedWithoutAdvancedBuild => {
                options.profile = Some((ProfileOptions::Debug, OptimizationLevel::NoOptimizations));
                options.target_architecture = Some(TargetArchitecture::X86_64);
                options.target_triple = Some(target::TargetTriple::new("x86_64-unknown-lightsaber"));

                break;
            },
            AdvancedBuildChoice::ProceedWithAdvancedBuild => {
                options = customize_build(options)?;
            },
            AdvancedBuildChoice::AbortBuild => {
                println!("Aborting build.");

                return Ok(utils::ExitCode(0));
            }
        }
    }

    println!("Stage 1: Rust compiler metadata (for debugging).");
    let mut rustc_metadata_command = process::Command::new("rustc");
    rustc_metadata_command
        .arg("--version")
        .arg("--verbose");

    if !rustc_metadata_command
        .status()
        .expect(&format!("Could not run command: {:?}", rustc_metadata_command))
        .success() {
        eprintln!("Could not get Rust compiler metadata.");

        return Ok(utils::ExitCode(1));
    }

    println!();
    println!("Stage 2: Removing previous build artifacts.");

    let mut remove_previous_build_artifacts_command = process::Command::new("cargo");
    remove_previous_build_artifacts_command.arg("clean");

    if !remove_previous_build_artifacts_command
        .status()
        .expect(&format!("Could not run command: {:?}", remove_previous_build_artifacts_command))
        .success() {
        eprintln!("Could not remove previously built artifacts.");

        return Ok(utils::ExitCode(1));
    }

    let target_architecture = options.target_architecture.clone().unwrap();
    let bootloader_target = format!("{}-unknown-uefi", target_architecture);
    let mut bootloader_build_command = process::Command::new("cargo");
    bootloader_build_command
        .arg("build")
        .arg("--package")
        .arg("lightsaber_bootloader")
        .arg("--target")
        .arg(&bootloader_target)
        .arg("-Z")
        .arg("build-std=core,alloc");

    let profile = options.profile.clone().unwrap();

    match profile.0 {
        ProfileOptions::Release => {
            bootloader_build_command.arg("--release");
        }
        _ => ()
    };

    bootloader_build_command.env("RUSTFLAGS", &format!("-C opt-level={}", profile.1));

    println!();
    println!("Stage 3: Compiling bootloader.");

    if !bootloader_build_command
        .status()
        .expect(&format!("Could not run command: {:?}", bootloader_build_command))
        .success() {
        eprintln!("Could not build bootloader.");

        return Ok(utils::ExitCode(-1));
    }

    let target_triple = options.target_triple.clone().unwrap();
    let mut kernel_build_command = process::Command::new("cargo");
    kernel_build_command
        .arg("build")
        .arg("--package")
        .arg("lightsaber_kernel")
        .arg("--target")
        .arg(&format!("./{}.json", target_triple.0))
        .arg("-Z")
        .arg("build-std=core,alloc");

    match profile.0 {
        ProfileOptions::Release => {
            kernel_build_command.arg("--release");
        }
        _ => ()
    };

    kernel_build_command.env("RUSTFLAGS", &format!("-C opt-level={}", profile.1));

    println!();
    println!("Stage 4: Compiling kernel.");

    if !kernel_build_command
        .status()
        .expect(&format!("Could not run command: {:?}", bootloader_build_command))
        .success() {
        eprintln!("Could not build kernel.");

        return Ok(utils::ExitCode(-1));
    }

    println!();
    println!("Stage 5: Copying built files.");

    println!("Copying `{}` to `./build/efi/boot/lightsaber_bootloader.efi`", &format!(r".\target\{}\{}\lightsaber_bootloader.efi", bootloader_target, profile.0));
    if let Err(error) = fs::copy(&format!(r".\target\{}\{}\lightsaber_bootloader.efi", bootloader_target, profile.0), r"./build/efi/boot/lightsaber_bootloader.efi") {
        println!("Could not copy bootloader file: {}", error);

        return Ok(utils::ExitCode(-1));
    };

    println!("Copying `{}` to `./build/efi/boot/kernel.elf`", &format!(r".\target\{}\{}\lightsaber_kernel", target_triple.0, profile.0));
    if let Err(error) = fs::copy(Path::new(&format!(r".\target\{}\{}\lightsaber_kernel", target_triple.0, profile.0)), r"./build/efi/kernel/kernel.elf") {
        println!("Could not copy kernel file: {}", error);

        return Ok(utils::ExitCode(-1));
    };

    println!();
    println!("The Lightsaber build is successfully complete! You can now run this build in a CPU");
    println!("emulator, QEMU for example; or boot it on a real computer by writing the files to a");
    println!("USB drive. Please note that the computer MUST have support for UEFI.");

    Ok(utils::ExitCode(0))
}

fn current_build_options(options: &BuildOptions) -> String {
    let target_architecture = options.target_architecture.clone().unwrap_or(TargetArchitecture::X86_64);
    let profile = options.profile.clone().unwrap_or((ProfileOptions::Debug, OptimizationLevel::NoOptimizations));

    format!(r"Current Project build options:

- Target Architecture: {}
-       Target Triple: {}
-             Profile: {}
-       Optimizations: {:?}
",
        options.target_architecture.clone().unwrap_or(target_architecture.clone()),
        options.target_triple.clone().unwrap_or(target::TargetTriple::new(&format!("{}-{}-{}", target_architecture, "unknown", "lightsaber"))).0,
        profile.0,
        profile.1
    )
}

fn customize_build(mut options: BuildOptions) -> anyhow::Result<BuildOptions> {
    println!("You will be asked for the value of each of the build options.");
    println!("You can also press the ENTER key to leave the value of that option unchanged.");

    println!();

    let profile = common::question_str("What is the build profile of this Build? (debug / release)", "debug")?;
    let optimization_level = common::question_str("What is the optimization level of this Build? (0 / 1 / 2 / 3 / s / z)", "0")?;
    let target_architecture = common::question_str("What is the target architecture of this Build? (x86_64 / aarch64)", "x86_64")?;
    let target_triple = common::question_str("What is the target triple of this Build (the kernel)? (`target architecture`-`vendor`-`operating system`)", "x86_64-unknown-lightsaber")?;

    options.profile = Some((match &*profile {
        "debug" => ProfileOptions::Debug,
        "release" => ProfileOptions::Release,
        _ => ProfileOptions::Debug
    },
    match &*optimization_level {
        "0" => OptimizationLevel::NoOptimizations,
        "1" => OptimizationLevel::BasicOptimizations,
        "2" => OptimizationLevel::SomeOptimizations,
        "3" => OptimizationLevel::AllOptimizations,
        "s" => OptimizationLevel::BinarySizeOptimization,
        "z" => OptimizationLevel::BinarySizeOptimizationLoopVectorizationOff,
        _ => OptimizationLevel::NoOptimizations
    }));

    options.target_architecture = Some(match &*target_architecture {
        "x86_64" => TargetArchitecture::X86_64,
        "aarch64" => TargetArchitecture::Aarch64,
        _ => TargetArchitecture::X86_64
    });

    options.target_triple = Some(match &*target_triple {
        triple => target::TargetTriple::new(triple)
    });

    Ok(options)
}
