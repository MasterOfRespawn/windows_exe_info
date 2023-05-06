# windows exe info

A [`Cargo` build script](http://doc.crates.io/build-script.html) library to
handle inclusion of Windows icons and version information without the use of
external `.rc` files.

Inspired by and using [embed_resource](https://docs.rs/embed-resource).
Use embed_resource when the `windres` command is not on PATH.

Generic image format conversion is done calling
[imagemagick](https://imagemagick.org). Imagemagick needs to be in PATH for
the conversion functions to work. If imagemagick is not found, the build
script will fail.

The only icon format capable of being used without imagemagick is `.ico`.

**INFO:** At the moment only one icon and versioninfo struct can be linked.

This crate only works on windows as resource scripts are a windows thing.
Linking for web will fail (I have not yet checked for linux or macos).
Use [build_cfg](https://docs.rs/build_cfg) for checking the target.
Build_cfg is also used for cross architecture compilation support.

## Features
- build_cfg: use [build_cfg](https://docs.rs/build_cfg) to use the correct target architecture
- embed_resource: use [embed_resource](https://docs.rs/embed-resource) crate for selecting `.rc` compiler
- icon_ico: allow inclusion of an icon
- icon_png: png format support using imagemagick
- icon_xxx: generic format support using imagemagick
- icon_svg: svg specific format conversion (currently using generic function)
- icon_xcf: xcf specific format conversion (currently using generic function)
- icon_placeholder: add a placeholder todo icon
- icon_autodetect: autodetect icon format and use specific conversion function (currently using generic function)
