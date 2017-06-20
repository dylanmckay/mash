error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(::std::io::Error);
        WavefrontLoadError(::tobj::LoadError) #[cfg(feature = "wavefront")];
    }

    errors {
        UnknownModelFormat(reason: String) {
            description("unknown model format")
            display("unknown model format: {}", reason)
        }

        IndexTooSmall(index: u64, bits_available: u8) {
            description("index too small for mesh")
            display("index too small for mesh: index '{}' cannot fit in {}-bits",
                    index, bits_available)
        }
    }
}

