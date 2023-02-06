use xshell::{Shell, cmd};

use crate::flags::Build;

mod flags {
    xflags::xflags! {
        /// Extra tasks for the Kettle project
        cmd xtask {
            /// Builds Kettle from source.
            cmd build {
                // Build in release mode
                optional -r, --release
            }
        }
    }
}

use std::{path::{Path, PathBuf}, env};

fn main() -> anyhow::Result<()> {
    use flags::{Xtask, XtaskCmd};

    let flags = Xtask::from_env_or_exit();
    
    let sh = &Shell::new()?;
    sh.change_dir(project_root());

    match flags.subcommand {
        XtaskCmd::Build(Build { release }) => {
            let _d = sh.push_dir("./kettle");

            let mut args = vec![];
            if release {
                args.push("--release");
            }

            cmd!(sh, "cargo build {args...}").run()?;

            drop(_d);
            Ok(())
        }
    }
}

fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}
