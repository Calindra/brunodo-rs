// #!/usr/bin/env node
// const fs = require("node:fs");
// const { spawn, spawnSync } = require("node:child_process");
// const path = require("node:path");
// const { arch, platform } = require("node:os");
// const { version } = require("../package.json");
//
// const PACKAGE_NONODO_VERSION = process.env.PACKAGE_NONODO_VERSION ?? "0.1.0";
// const PACKAGE_NONODO_PATH =
//   process.env.PACKAGE_NONODO_PATH ?? path.join(__dirname, "..", "bin");
//
// const AVAILABLE_NONODE_ARCH = new Set([
//   "darwin-amd64",
//   "darwin-arm64",
//   "linux-amd64",
//   "linux-arm64",
//   "windows-amd64",
// ]);
//
// function runNonodo(location) {
//   // console.log(`Running nonodo binary: ${location}`);
//
//   const args = process.argv.slice(2);
//   const nonodoBin = spawn(location, args, { stdio: "inherit" });
//   nonodoBin.on("exit", (code, signal) => {
//     process.on("exit", () => {
//       if (signal) {
//         process.kill(process.pid, signal);
//       } else {
//         process.exit(code);
//       }
//     });
//   });
//
//   process.on("SIGINT", function () {
//     nonodoBin.kill("SIGINT");
//     nonodoBin.kill("SIGTERM");
//   });
// }
//
// function getNonodoAvailable() {
//   const nonodoPath = PACKAGE_NONODO_PATH;
//
//   let myPlatform = platform();
//   if (myPlatform === "win32") myPlatform = "windows";
//
//   let myArch = arch();
//   if (myArch === "x64") myArch = "amd64";
//
//   const support = `${myPlatform}-${myArch}`;
//
//   // console.log(`Looking for nonodo binary for: ${support}`);
//
//   if (AVAILABLE_NONODE_ARCH.has(support)) {
//     let filename = `nonodo-v${PACKAGE_NONODO_VERSION}-${support}`;
//     if (platform() === "win32") filename += ".exe";
//
//     const fullpath = path.join(nonodoPath, filename);
//
//     // console.log(`Checking: ${fullpath}`);
//
//     // Check if the file exists
//     if (!fs.existsSync(fullpath)) {
//       throw new Error(`No nonodo binary found: ${fullpath}`);
//     }
//
//     // Check if the file is accessible
//     try {
//       fs.accessSync(fullpath, fs.constants.F_OK);
//     } catch (e) {
//       throw new Error(`No access: ${fullpath}`);
//     }
//
//     return fullpath;
//   }
//
//   throw new Error(
//     `Incompatible platform. Nonodo supports: ${[...AVAILABLE_NONODE_ARCH].join(", ")}`,
//   );
// }
//
// function tryPackageNonodo() {
//   console.log(`Running nonodo ${version} for ${arch()} ${platform()}`);
//
//   try {
//     const nonodoPath = getNonodoAvailable();
//     runNonodo(nonodoPath);
//     return true;
//   } catch (e) {
//     console.error(e);
//   }
//
//   return false;
// }
//
// tryPackageNonodo();
//

use dotenvy::var;
use std::env::args;
use std::env::consts::{ARCH, OS};
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

fn run_nonodo(location: PathBuf) -> Result<ExitStatus, Box<dyn Error>> {
    println!("Running nonodo binary: {}", location.display());

    let args = args().collect::<Vec<_>>();
    let mut nonodo_bin = Command::new(location).args(&args[1..]).spawn()?;
    Ok(nonodo_bin.wait()?)
}

fn get_nonodo_available() -> Result<PathBuf, Box<dyn Error>> {
    let pkg_version = var("PACKAGE_NONODO_VERSION").unwrap_or("0.1.0".to_string());

    let pkg_path = var("PACKAGE_NONODO_PATH").unwrap_or_else(|_| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("bin")
            .to_string_lossy()
            .to_string()
    });

    let support = match (ARCH, OS) {
        ("x86_64", "macos") => "darwin-amd64",
        ("aarch64", "macos") => "darwin-arm64",
        ("x86_64", "linux") => "linux-amd64",
        ("aarch64", "linux") => "linux-arm64",
        ("x86_64", "windows") => "windows-amd64.exe",
        _ => Err("Unsupported platform")?,
    };

    let filename = format!("nonodo-v{}-{}", pkg_version, support);
    let fullpath = PathBuf::from(pkg_path).join(filename);

    if !fullpath.exists() {
        return Err("No nonodo binary found".into());
    }

    if !fullpath.is_file() {
        return Err("No access".into());
    }

    Ok(fullpath)
}

fn main() -> Result<(), Box<dyn Error>> {
    let version = env!("CARGO_PKG_VERSION");

    println!("Running nonodo {} for {} {}", version, ARCH, OS);
    let nonodo_path = get_nonodo_available().unwrap();
    let result = run_nonodo(nonodo_path)?;

    dbg!(result);

    Ok(())
}
