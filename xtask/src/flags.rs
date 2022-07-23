//! xflags is a bit jank still, but pretty nice compared to pico_args, and
//! compiles a billion times faster than clap/structopt.
//!
//! When you modify stuff in the args_parser block, you'll get compile errors.
//! to fix these:
//! 1. run `env UPDATE_XFLAGS=1 cargo build -p xtask` in the shell.
//! 2. this will error, but should update the contents of this file that exists
//!    between `// generated start` and `// generated end`.
//! 3. If it has trouble updating, try deleting everything from this file that
//!    isn't part of the `xflags!` macro (all the generated code), or ping me
//!    (@thomcc).

#![allow(dead_code)]

xflags::xflags! {
    src "./src/flags.rs"
    /// Run custom build command.
    cmd xtask {
        optional -v, --verbose

        default cmd help {
            /// Print help information.
            optional -h, --help
        }
        /// Run lints the way we run it in CI
        cmd lint {}
        /// Run tests the way we run them in CI
        cmd test {}
        /// produce bindings using installed `bindgen`.
        cmd bindgen {
            /// folder containing cimgui output (default: imgui-sys/third-party)
            optional --cimgui-path cimgui_path: String
            /// default: imgui-sys/src
            optional --output-path output_path: String
            /// default: imgui-sys-v0
            optional --wasm-import-name wasm_import_name: String
        }
    }
}

// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct Xtask {
    pub verbose: bool,
    pub subcommand: XtaskCmd,
}

#[derive(Debug)]
pub enum XtaskCmd {
    Help(Help),
    Lint(Lint),
    Test(Test),
    Bindgen(Bindgen),
}

#[derive(Debug)]
pub struct Help {
    pub help: bool,
}

#[derive(Debug)]
pub struct Lint;

#[derive(Debug)]
pub struct Test;

#[derive(Debug)]
pub struct Bindgen {
    pub cimgui_path: Option<String>,
    pub output_path: Option<String>,
    pub wasm_import_name: Option<String>,
}

impl Xtask {
    pub const HELP: &'static str = Self::HELP_;

    #[allow(dead_code)]
    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    #[allow(dead_code)]
    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end
