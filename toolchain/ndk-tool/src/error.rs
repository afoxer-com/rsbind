use error_chain::error_chain;

error_chain! {
    errors {
        NdkBuildError(msg: String) {
            description("Ndk error"),
            display("ndk error: {}", msg),
        }
    }

    foreign_links {
        Ndk(ndk_build::error::NdkError);
        Io(::std::io::Error);
        Fmt(::std::fmt::Error);
    }
}
