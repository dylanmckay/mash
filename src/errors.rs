error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        UnknownModelFormat(reason: String) {
            description("unknown model format")
            display("unknown model format: {}", reason)
        }
    }
}

