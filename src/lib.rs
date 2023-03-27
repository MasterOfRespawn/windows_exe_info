#![feature(exit_status_error)]
#![allow(dead_code)]

#[cfg(feature = "build_cfg")]
extern crate build_cfg;
#[cfg(feature = "embed_resource")]
extern crate embed_resource;

use std::env::var;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const WINDRES_RESOURCE_SCRIPT: &str = "id ICON \"[PATH]\"\n";
#[cfg(feature = "build_cfg")]
const WINDRES_COMMAND: &str = "-i [INPUT] -O coff -F [ARCH] -o [OUTPUT] -v";
#[cfg(not(feature = "build_cfg"))]
const WINDRES_COMMAND: &str = "-i [INPUT] -O coff -o [OUTPUT] -v";
const MAGICK_COMMAND_SCALE_PNG: &str = "convert [INPUT] -scale [SCALE]x[SCALE] -extent [SCALE]x[SCALE] -background None -alpha on [OUTPUT][SCALE].png";
const MAGICK_COMMAND_XXX_TO_PNG: &str =
    "convert [INPUT] -background None -alpha on -scale 256x256 -layers merge [OUTPUT]";

const MAGICK_ICON_SCALES: &[&str] = &["8", "16", "32", "48", "64", "128", "256"];

#[cfg(feature = "icon_placeholder")]
const PLACEHOLDER: &str = include_str!("../icon.svg");
#[cfg(feature = "icon_placeholder")]
pub fn placeholder() {
    let output_dir = var("OUT_DIR").unwrap();
    let png_path = output_dir.clone() + "/icon.svg";
    let _ = std::fs::File::options()
        .write(true)
        .create(true)
        .open(&png_path)
        .unwrap()
        .write(PLACEHOLDER.as_bytes());
    icon_svg(&std::path::PathBuf::from(&png_path));
}

#[cfg(feature = "icon_autodetect")]
pub fn icon(path: &Path) {
    if !path.exists() {
        panic!("File does not exist");
    }
    if let Some(extension) = path.extension() {
        #[cfg(feature = "icon_ico")]
        if extension == "ico" {
            icon_ico(path);
            return;
        }
        #[cfg(feature = "icon_png")]
        if extension == "png" {
            icon_png(path);
            return;
        }
        #[cfg(feature = "icon_svg")]
        if extension == "svg" {
            icon_svg(path);
            return;
        }
        #[cfg(feature = "icon_xcf")]
        if extension == "xcf" {
            icon_xcf(path);
            return;
        }
    }
    icon_xxx(path);
}

#[cfg(feature = "icon_ico")]
pub fn icon_ico(path: &Path) {
    if !path.exists() {
        panic!("Path does not exist");
    }

    let output_dir = var("OUT_DIR").unwrap();
    let buildres_file = output_dir.clone() + "/icon.rc";

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(buildres_file.as_str())
        .unwrap();
    let resource_script_content = WINDRES_RESOURCE_SCRIPT.replace(
        "[PATH]",
        &path
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string()
            .replace("\\", "/"),
    );

    if resource_script_content.len() != file.write(resource_script_content.as_bytes()).unwrap() {
        panic!("An error occurred while writing the resource file.");
    }

    #[cfg(feature = "embed_resource")]
    embed_resource::compile(buildres_file);

    #[cfg(not(feature = "embed_resource"))]
    {
        let resource_file = output_dir.clone() + "/icon.res";
        let args = WINDRES_COMMAND
            .replace("[INPUT]", buildres_file.as_str())
            .replace("[OUTPUT]", resource_file.as_str());

        #[cfg(feature = "build_cfg")]
        let args = if build_cfg!(target_os = "windows") {
            if build_cfg!(target_pointer_width = "64") {
                args.replace("[ARCH]", "pe-x86-64")
            } else {
                args.replace("[ARCH]", "pe-i386")
            }
        } else {
            panic!("Invalid target operating system");
        };

        let _ = Command::new("windres")
            .args(args.split(" "))
            .spawn()
            .expect("Execution failed")
            .wait()
            .expect("Execution failed")
            .exit_ok()
            .expect("Command Failed");

        #[cfg(target_family = "windows")]
        println!("cargo:rustc-link-arg={resource_file}"); // Tell it to link
    }
}

#[cfg(feature = "icon_png")]
pub fn icon_png(path: &Path) {
    if !path.exists() {
        panic!("Path does not exist");
    }
    let output_dir = var("OUT_DIR").unwrap();
    let icon_path = output_dir.clone() + "/icon.ico";

    for scale in MAGICK_ICON_SCALES.iter(){
        let args = MAGICK_COMMAND_SCALE_PNG
        .replace("[INPUT]", path.as_os_str().to_str().unwrap())
        .replace("[SCALE]", scale)
        .replace("[OUTPUT]", &output_dir.clone())
        .replace("\n", " ");

        let args = args.split(" ");

        let _ = Command::new("magick")
            .args(args)
            .spawn()
            .expect("Execution failed")
            .wait()
            .expect("Execution failed")
            .exit_ok()
            .expect("Command Failed");
    }


    let args = ["convert", path.as_os_str().to_str().unwrap(), icon_path.as_str()];

    let mut cmd = Command::new("magick");
    let cmd = cmd.arg("convert");
    
    for scale in MAGICK_ICON_SCALES.iter(){
        cmd.arg(output_dir.clone() + scale + ".png");
    }
    
    let _ = cmd.spawn()
        .expect("Execution failed")
        .wait()
        .expect("Execution failed")
        .exit_ok()
        .expect("Command Failed");

    icon_ico(Path::new(&icon_path));
}

#[cfg(feature = "icon_xxx")]
pub fn icon_xxx(path: &Path) {
    if !path.exists() {
        panic!("Path does not exist");
    }
    let output_dir = var("OUT_DIR").unwrap();
    let png_path = output_dir.clone() + "/icon.png";

    let args = MAGICK_COMMAND_XXX_TO_PNG
        .replace("[INPUT]", path.as_os_str().to_str().unwrap())
        .replace("[OUTPUT]", png_path.as_str());
    let args = args.split(" ");

    let _ = Command::new("magick")
        .args(args)
        .spawn()
        .expect("Execution failed")
        .wait()
        .expect("Execution failed")
        .exit_ok()
        .expect("Command Failed");

    icon_png(Path::new(&png_path));
}

#[cfg(feature = "icon_svg")]
pub fn icon_svg(path: &Path) {
    if !path.exists() {
        panic!("Path does not exist");
    }

    icon_xxx(path); // ToDo: Add specific optimized implementation
}

#[cfg(feature = "icon_xcf")]
pub fn icon_xcf(path: &Path) {
    if !path.exists() {
        panic!("Path does not exist");
    }

    icon_xxx(path); // ToDo: Add specific optimized implementation
}
