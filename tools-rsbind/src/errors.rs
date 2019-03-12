error_chain! {
    errors {
        FileError(msg: String) {
            description("file error"),
            display("file error: {}", msg),
        }
        ParseError(msg: String) {
            description("parse error"),
            display("parse error: {}", msg),
        }
        GenerateError(msg: String) {
            description("generate error"),
            display("parse error: {}", msg),
        }

        ZipError(msg: String) {
            description("zip error"),
            display("zip error: {}", msg),
        }
        CommandError(msg: String) {
            description("command error"),
            display("command error: {}", msg),
        }
    }

    foreign_links {
        IO(::std::io::Error);
        TOML(::toml::de::Error);
    }
}
