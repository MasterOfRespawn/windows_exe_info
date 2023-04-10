# windows exe info

A very simple rust library to add an icon to an executable via build-scripts.
UPDATE: also adds other infos

## features

- `build_cfg` used for a hacky way of checking the correct output architecture.
  - only works without `embed_resource`
  - only supports the gnu toolchain
- `embed_resource` to embed the `.rc` file more professionally
  - using a seperate crate
  - supports all toolchains

## TODO

- [ ] Improve readability of `icon.rs`
