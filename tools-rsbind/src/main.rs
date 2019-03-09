extern crate rsbind_core as gen;

use gen::Action;
use gen::Target;
use std::env;
use std::process;

fn main() {
    let mut args = env::args();
    let _ = args.next(); // path
    let path = match args.next() {
        Some(real) => real,
        _ => {
            eprintln!(
                "Usage: rsbind path-to-project java/ios/all ast/bridge/dest/header/build/all"
            );
            process::exit(1);
        }
    };

    let target = match args.next() {
        Some(option) => option,
        _ => "all".to_string(),
    };

    let target_enum = match target.as_ref() {
        "android" => Target::Android,
        "ios" => Target::Ios,
        "all" => Target::All,
        _ => {
            eprintln!(
                "Usage: rsbind path-to-project java/ios/all ast/bridge/dest/header/build/all"
            );
            process::exit(1);
        }
    };

    let action = match args.next() {
        Some(option) => option,
        _ => {
            eprintln!(
                "Usage: rsbind path-to-project java/ios/all ast/bridge/dest/header/build/all"
            );
            process::exit(1);
        }
    };

    let action_enum = match action.as_ref() {
        "ast" => Action::GEN_AST,
        "bridge" => Action::GEN_BRIDGE,
        "dest" => Action::GEN_BIND_SRC,
        "header" => Action::GEN_C_HEADER,
        "build" => Action::BUILD,
        "all" => Action::ALL,
        _ => {
            eprintln!(
                "Usage: rsbind path-to-project java/ios/all ast/bridge/dest/header/build/all"
            );
            process::exit(1);
        }
    };

    gen::Bind::from(path, target_enum, action_enum)
        .gen_all()
        .expect("generate failed");
}
