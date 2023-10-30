//! Functions for linking icons to the executable.
//! File paths are specified relative to the root of the crate.
//! The first linked icon will be used by windows as the executable's icon

#[cfg(feature = "build_cfg")]
extern crate build_cfg;
#[cfg(feature = "embed_resource")]
extern crate embed_resource;

use std::env::var;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
#[cfg(feature = "icon_png")]
use std::process::Command;

const ICON_RESOURCE_SCRIPT: &str = "[ID] ICON \"[PATH]\"\n";
const MAGICK_COMMAND_SCALE_PNG: &str = "convert [INPUT] -scale [SCALE]x[SCALE] -extent [SCALE]x[SCALE] -background None -alpha on [OUTPUT][SCALE].png";
const MAGICK_COMMAND_XXX_TO_PNG: &str =
    "convert [INPUT] -background None -alpha on -scale 256x256 -layers merge [OUTPUT]";

const MAGICK_ICON_SCALES: &[&str] = &["8", "16", "32", "48", "64", "128", "256"];

pub(crate) static mut CURRENT_ICON_ID: u16 = 0;

#[cfg(feature = "icon_placeholder")]
const PLACEHOLDER: &[u8] = include_bytes!("../icon.ico");
#[cfg(feature = "icon_placeholder")]
/// add a todo icon to the executable
pub fn placeholder() {
    let output_dir = var("OUT_DIR").unwrap();
    let png_path = output_dir.clone() + "/icon.ico";
    let _ = std::fs::File::options()
        .write(true)
        .create(true)
        .open(&png_path)
        .unwrap()
        .write(PLACEHOLDER)
        .unwrap();
    icon_ico(&std::path::PathBuf::from(&png_path));
}

#[cfg(feature = "icon_autodetect")]
/// autodetect icon format based on file ending
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
    }
    #[cfg(feature = "icon_xxx")]
    icon_xxx(path);
}

#[cfg(feature = "icon_ico")]
/// link icon in `ico` format to executable
pub fn icon_ico(path: &Path) {
    if !path.exists() {
        panic!("Path does not exist");
    }

    let output_dir = var("OUT_DIR").unwrap();
    let buildres_file = output_dir.clone() + &unsafe { format!("/icon{}.rc", CURRENT_ICON_ID) };

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(buildres_file.as_str())
        .unwrap();
    let resource_script_content = ICON_RESOURCE_SCRIPT
        .replace(
            "[PATH]",
            &path
                .as_os_str()
                .to_str()
                .unwrap()
                .to_string()
                .replace("\\", "/"),
        )
        .replace("[ID]", &unsafe { format!("icon{}", CURRENT_ICON_ID) });
    unsafe {
        CURRENT_ICON_ID += 1;
    }

    if resource_script_content.len() != file.write(resource_script_content.as_bytes()).unwrap() {
        panic!("An error occurred while writing the resource file.");
    }

    super::link::link(buildres_file);
}

#[cfg(feature = "icon_png")]
/// convert and scale `png` format to `ico` using imagemagick
pub fn icon_png(path: &Path) {
    if !path.exists() {
        panic!("Path does not exist");
    }
    let output_dir = var("OUT_DIR").unwrap();
    let icon_path = output_dir.clone() + "/icon.ico";

    for scale in MAGICK_ICON_SCALES.iter() {
        let args = MAGICK_COMMAND_SCALE_PNG
            .replace("[INPUT]", path.as_os_str().to_str().unwrap())
            .replace("[SCALE]", scale)
            .replace("[OUTPUT]", &output_dir.clone())
            .replace("\n", " ");

        let args = args.split(" ");

        assert!(Command::new("magick")
            .args(args)
            .spawn()
            .expect("Execution failed")
            .wait()
            .expect("Execution failed")
            .success());
    }

    //let args = ["convert", path.as_os_str().to_str().unwrap(), icon_path.as_str()];

    let mut cmd = Command::new("magick");
    let cmd = cmd.arg("convert");

    for scale in MAGICK_ICON_SCALES.iter() {
        cmd.arg(output_dir.clone() + scale + ".png");
    }

    cmd.arg(icon_path.as_str());

    assert!(cmd
        .spawn()
        .expect("Execution failed")
        .wait()
        .expect("Execution failed")
        .success());

    icon_ico(Path::new(&icon_path));
}

#[cfg(feature = "icon_xxx")]
/// convert any format to `png` using imagemagick and link it
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

    assert!(Command::new("magick")
        .args(args)
        .spawn()
        .expect("Execution failed")
        .wait()
        .expect("Execution failed")
        .success());

    icon_png(Path::new(&png_path));
}