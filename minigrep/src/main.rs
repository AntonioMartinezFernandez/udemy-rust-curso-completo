/*******************************************************************
 *
 *  PROJECT 01 - minigrep
 *
 *  Description:
 *      Just a little version of the 'grep' unix terminal tool.
 *
 *  Example of use:
 *      minigrep log.example easter_egg
 *
 *  Example of use without compile the code:
 *      cargo run log.example easter_egg
 *
*******************************************************************/

use minigrep::{run, Config};
use std::{env, process::exit};

fn main() {
    // read arguments
    let args = env::args();

    // convert 'Args' to 'Vec<String>' collection
    let args_collection: Vec<String> = args.collect();

    // check if we have enough parameters (or exit)
    if args_collection.len() < 3 {
        println!("not enough parameters");
        exit(0x0000);
    }

    // create the 'Config' struct instance
    let config = Config::new(&args_collection);

    // execute 'run' method
    run(config);
}
