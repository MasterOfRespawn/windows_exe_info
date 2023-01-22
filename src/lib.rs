#![feature(exit_status_error)]

#[cfg(feature="build_cfg")]
#[macro_use]
extern crate build_cfg;

use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::fs::OpenOptions;
use std::env::var;

const WINDRES_RESOURCE_SCRIPT: &str = "id ICON \"[PATH]\"";
#[cfg(feature="build_cfg")]
const WINDRES_COMMAND: &str = "-i [INPUT] -O coff -F [ARCH] -o [OUTPUT] -v";
#[cfg(not(feature="build_cfg"))]
const WINDRES_COMMAND: &str = "-i [INPUT] -O coff -o [OUTPUT] -v";
const MAGICK_COMMAND_PNG_TO_ICO: &str = 
"convert [INPUT] -background None
( -clone 0 -scale 256x256 -extent 256x256 -background None -alpha on )
( -clone 1 -scale 128x128 -extent 128x128 -background None -alpha on )
( -clone 2 -scale 64x64 -extent 64x64 -background None -alpha on )
( -clone 3 -scale 48x48 -extent 48x48 -background None -alpha on )
( -clone 4 -scale 32x32 -extent 32x32 -background None -alpha on )
( -clone 5 -scale 16x16 -extent 16x16 -background None -alpha on )
( -clone 6 -scale 8x8 -extent 8x8 -background None -alpha on )
-alpha on -colors 256 [OUTPUT]";
const MAGICK_COMMAND_XXX_TO_PNG: &str = 
"convert [INPUT] -background None -alpha on -scale 256x256 -layers merge [OUTPUT]";

#[cfg(feature="placeholder")]
const PLACEHOLDER: &str = include_str!("../icon.svg");
#[cfg(feature="placeholder")]
pub fn placeholder(){
    let output_dir = var("OUT_DIR").unwrap();
    let png_path = output_dir.clone() + "icon.svg";
    let _ = std::fs::File::options().write(true).create(true).open(&png_path).unwrap().write(PLACEHOLDER.as_bytes());
    icon_svg(&std::path::PathBuf::from(&png_path));
}

#[cfg(feature="auto")]
pub fn icon(path: &Path){
    if !path.exists() { panic!("File does not exist"); }
    if let Some(extension) = path.extension(){
        #[cfg(feature="ico")]
        if extension == "ico" { icon_ico(path); return }
        #[cfg(feature="png")]
        if extension == "png" { icon_png(path); return }
        #[cfg(feature="svg")]
        if extension == "svg" { icon_svg(path); return }
        #[cfg(feature="xcf")]
        if extension == "xcf" { icon_xcf(path); return }
    } //else { panic!("Please specify the icon type!") }
    icon_xxx(path);
}

#[cfg(feature="ico")]
pub fn icon_ico(path: &Path){
    if !path.exists() { panic!("Path does not exist"); }

    let output_dir = var("OUT_DIR").unwrap();
    let buildres_file = output_dir.clone() + "icon.rc";
    let resource_file = output_dir.clone() + "icon.res";

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(buildres_file.as_str())
        .unwrap();
    let resource_script_content = WINDRES_RESOURCE_SCRIPT.replace("[PATH]", &path.as_os_str().to_str().unwrap().to_string().replace("\\", "/"));
    if resource_script_content.len() != file.write(resource_script_content.as_bytes()).unwrap(){
        panic!("An error occurred while writing the resource file.");
    }

    let args = WINDRES_COMMAND
        .replace("[INPUT]", buildres_file.as_str())
        .replace("[OUTPUT]", resource_file.as_str());
    
    #[cfg(feature="build_cfg")]
    let args = if build_cfg!(target_os = "windows") {
        if build_cfg!(target_pointer_width = "64"){
            args.replace("[ARCH]", "pe-x86-64")
        } else {
            args.replace("[ARCH]", "pe-i386")
        }
    } else {
        panic!("Invalid target operating system");
    };
    
    let _ = Command::new("windres")
        .args(args
            .split(" "))
        .spawn()
        .expect("Execution failed")
        .wait()
        .expect("Execution failed")
        .exit_ok()
        .expect("Command Failed");

    #[cfg(target_family="windows")]
    println!("cargo:rustc-link-arg={resource_file}"); // Tell it to link
}

#[cfg(feature="png")]
pub fn icon_png(path: &Path){
    if !path.exists() { panic!("Path does not exist"); }
    let output_dir = var("OUT_DIR").unwrap();
    let icon_path = output_dir.clone() + "icon.ico";

    let args = MAGICK_COMMAND_PNG_TO_ICO
        .replace("[INPUT]", path.as_os_str().to_str().unwrap())
        .replace("[OUTPUT]", icon_path.as_str())
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

    icon_ico(Path::new(&icon_path));
}

#[cfg(feature="xxx")]
pub fn icon_xxx(path: &Path){
    if !path.exists() { panic!("Path does not exist"); }
    let output_dir = var("OUT_DIR").unwrap();
    let png_path = output_dir.clone() + "icon.png";

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

#[cfg(feature="svg")]
pub fn icon_svg(path: &Path){
    if !path.exists() { panic!("Path does not exist"); }

    icon_xxx(path); // ToDo: Add specific optimized implementation
}

#[cfg(feature="xcf")]
pub fn icon_xcf(path: &Path){
    if !path.exists() { panic!("Path does not exist"); }

    icon_xxx(path); // ToDo: Add specific optimized implementation
}