# windows exe info

A [`Cargo` build script](http://doc.crates.io/build-script.html) library to
handle inclusion of Windows icons, version information and application
manifests without the use of external `.rc` files.

Inspired by and using [embed_resource](https://docs.rs/embed-resource).
Use embed_resource when the `windres` command is not on PATH.

Generic image format conversion requires
[imagemagick](https://imagemagick.org). Imagemagick needs to be in PATH for
the conversion functions to work. If imagemagick is not found, the build
script will fail.

The only icon format available without imagemagick is `.ico`.

This crate only works on windows as resource scripts are a windows thing.
By default it will check whether it is compiling for windows and will ignore
linking calls otherwise. 
Build_cfg is required for cross architecture compilation.

## Features
- build_cfg: use [build_cfg](https://docs.rs/build_cfg) to target the correct architecture
- embed_resource: use [embed_resource](https://docs.rs/embed-resource) crate for selecting `.rc` compiler
- icon_ico: basic icon linking
- icon_placeholder: add a placeholder todo icon
- icon_png: png format support using imagemagick
- icon_magick: generic format support using imagemagick
- icon_autodetect: autodetect icon format by file extension and use specific conversion function (ico, png or magick)
- manifest: allow embedding [Windows application manifest xml files](https://learn.microsoft.com/en-us/windows/win32/sbscs/manifest-files-reference) in the executable \
the manifest feature is **not** required to embed version information or an icon
- versioninfo: allow adding windows version information to the executable
- windows_only: check if the compilation target is windows and do not link if otherwise

The default features are `embed_resource`, `icon_ico`, `icon_placeholder`, `versioninfo` and `windows_only`

## breaking changes
### 0.4.2
- none
### 0.4.1
- add `windows_only` feature by default to prevent linking against non windows operating systems
### 0.4.0
- `icon_xxx`, `icon_svg` and `icon_xcf` all have been replaced by `icon_magick`
- the `manifest` feature is now optional

## examples
add [this crate](https://crates.io/crates/windows_exe_info) to your build-dependencies

In `Cargo.toml`
```toml
# the rest of the [package] section
build = "build.rs"

[build-dependencies]
windows_exe_info = "0.4"
```

- adding an icon (`.ico`)

In `build.rs`
```rust
extern crate windows_exe_info;
fn main(){
    windows_exe_info::icon::icon_ico(std::path::Path::new("PATH/TO/ICON.ico"));
}
```

- adding version information based on cargo's environment variables

In `build.rs` choose one of these options
```rust
extern crate windows_exe_info;
fn main(){
    // simple option 1
    windows_exe_info::versioninfo::link_cargo_env();
    // simple option 2
    windows_exe_info::versioninfo::VersionInfo::from_cargo_env().link().unwrap();
    // advanced option
    windows_exe_info::versioninfo::VersionInfo::from_cargo_env_ex(
        Some("comment"),
        Some("company name"),
        Some("copyright"),
        Some("trademarks")
    ).link().unwrap();
    // these three function calls do effectively the same but are required only once
}
```

- adding version information manually

In `build.rs`
```rust
extern crate windows_exe_info;
fn main(){
    use windows_exe_info::versioninfo::*;
    // Change these attributes as you need
    VersionInfo {
        file_version: Version(0, 1, 0, 0),
        product_version: Version(0, 1, 0, 0),
        file_flag_mask: FileFlagMask::Win16,
        file_flags: FileFlags {
            debug: false,
            patched: false,
            prerelease: false,
            privatebuild: false,
            infoinferred: false,
            specialbuild: false,
        },
        file_os: FileOS::Windows32,
        file_type: FileType::App,
        file_info: vec![FileInfo {
            lang: Language::USEnglish,
            charset: CharacterSet::Multilingual,
            comment: None,
            company_name: "".into(),
            file_description: "An example build script".into(),
            file_version: "0.1.0.0".into(),
            internal_name: "example".into(),
            legal_copyright: None,
            legal_trademarks: None,
            original_filename: "example.exe".into(),
            product_name: "Example".into(),
            product_version: "0.1.0.0".into(),
            private_build: None,
            special_build: None,
        }],
    }
    .link().unwrap();
}
```

- embedding a [manifest](https://learn.microsoft.com/en-us/windows/win32/sbscs/manifest-files-reference)

add the manifest feature in `Cargo.toml`
```toml
windows_exe_info = {version = "0.4", features = ["manifest"]}
```
In `build.rs`
```rust
extern crate windows_exe_info;
fn main(){
    windows_exe_info::manifest::manifest(std::path::Path::new("PATH/TO/MANIFEST.XML"));
}
```