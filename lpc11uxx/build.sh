#!/bin/bash -e

AUTHORS='"MasterR3C0RD <masterr3c0rd@epochal.quest"'
SVD2RUST_FEATURES="--strict --generic_mod --atomics --feature_group"

NAME=$(basename $PWD)
VERSION=0.5.0
SVD_FILE=$NAME.svd

# clean up old files
rm -rf src/ Cargo.toml build.rs generic.rs device.x features.toml 

# generate from SVD file
svd2rust -i $SVD_FILE $SVD2RUST_FEATURES
form -i lib.rs -o src
mv generic.rs src/
rm lib.rs

# build Cargo.toml
FEATURE_GROUPS=$(sed -e 1,3d < features.toml)

cat >> Cargo.toml << EOF
[package]
name = "$NAME"
version = "$VERSION"
authors = [$AUTHORS]
edition = "2021"

[dependencies]
critical-section = { version = "1.0", optional = true }
cortex-m = "0.7.6"
cortex-m-rt = { version = "0.6.13", optional = true }
vcell = "0.1.2"
derive_more = { version = "0.99.17", default_features = false, features = ["from", "deref", "deref_mut"] }
portable-atomic = { version = "1", default-features = false }

[features]
default = ["all-groups"]
rt = ["cortex-m-rt/device"]
$FEATURE_GROUPS
EOF

rm features.toml

# format
cargo fmt --quiet

# check to mak sure that everything's all good
cargo check --quiet --message-format short --features all-groups