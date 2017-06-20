# mash

[![Build Status](https://travis-ci.com/dylanmckay/mash.svg?token=yjrpKtNnXqa4h6sV1BQh&branch=master)](https://travis-ci.com/dylanmckay/mash)
[![license](https://img.shields.io/github/license/dylanmckay/mash.svg)]()

[Documentation](https://docs.rs/mash)

3D mesh manipulation library.

## Supported formats

All formats are enabled by default when including `mash` as a dependency.

* [Wavefront OBJ](https://en.wikipedia.org/wiki/Wavefront_.obj_file)

In order to pick and choose which formats are supported, explicitly set which features
you want to enable.

```toml
[dependencies]
mash = { version = "*", default-features = false, features = ["wavefront"]}
```

