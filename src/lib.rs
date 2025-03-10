//! lib target

use env_logger::Builder;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref LOG: u64 = init_log();
}

// install a logger facility
fn init_log() -> u64 {
    Builder::from_default_env().init();
    println!("\n ************** initializing logger *****************\n");
    1
}

pub mod io;

pub mod embed;

pub mod embedding;

pub mod validation;

pub mod prelude;
//

pub mod structure;
