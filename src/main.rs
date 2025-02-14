#![feature(rustc_private)]
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;

use rustc_driver::Callbacks;
use rustc_errors::{emitter::HumanReadableErrorType, ColorConfig};
use rustc_interface::interface;
use rustc_session::config::ErrorOutputType;
use rustc_session::EarlyDiagCtxt;

struct OrphansOk;

impl Callbacks for OrphansOk {
    fn config(&mut self, config: &mut interface::Config) {
        config.override_queries =
            Some(|_session, providers: &mut rustc_middle::util::Providers| {
                providers.queries.orphan_check_impl = |_tcx, _local_key| Ok(());
            });
    }
}

fn main() {
    rustc_driver::install_ice_hook(
        "https://github.com/aDotInTheVoid/orphan-fool/issues/new",
        |_| (),
    );
    let handler = EarlyDiagCtxt::new(ErrorOutputType::HumanReadable(
        HumanReadableErrorType::Default,
        ColorConfig::Auto,
    ));
    rustc_driver::init_rustc_env_logger(&handler);
    std::process::exit(rustc_driver::catch_with_exit_code(move || {
        let args: Vec<String> = std::env::args().collect();
        rustc_driver::run_compiler(&args, &mut OrphansOk);
    }))
}
