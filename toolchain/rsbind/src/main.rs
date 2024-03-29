extern crate rsbind_core as gen;

use std::env;
use std::process;

use gen::Action;
use gen::Target;

fn main() {
    let mut args = env::args();
    let _ = args.next(); // path
    let path = match args.next() {
        Some(real) => real,
        _ => {
            eprintln!(
                "Usage: rsbind path-to-project android/ios/mac/jar ast/bridge/artifact/header/build/all"
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
        "mac" => Target::Mac,
        "jar" => Target::Jar,
        _ => {
            eprintln!(
                "Usage: rsbind path-to-project android/ios/mac/jar ast/bridge/artifact/header/build/all"
            );
            process::exit(1);
        }
    };

    let action = match args.next() {
        Some(option) => option,
        _ => {
            eprintln!(
                "Usage: rsbind path-to-project android/ios/mac/jar ast/bridge/artifact/header/build/all"
            );
            process::exit(1);
        }
    };

    let action_enum = match action.as_ref() {
        "ast" => Action::GenAst,
        "bridge" => Action::GenBridge,
        "artifact" => Action::GenArtifactCode,
        "header" => Action::GenCHeader,
        "build" => Action::BuildArtifact,
        "all" => Action::All,
        _ => {
            eprintln!(
                "Usage: rsbind path-to-project android/ios/mac/jar ast/bridge/artifact/header/build/all"
            );
            process::exit(1);
        }
    };

    gen::Bind::from(path, target_enum, action_enum)
        .gen_all()
        .expect("generate failed");
}
