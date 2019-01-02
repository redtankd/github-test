README
===========

[![Build Status](https://travis-ci.org/redtankd/rust-test.svg?branch=master)](https://travis-ci.org/redtankd/rust-test) [![Coverage Status](https://coveralls.io/repos/github/redtankd/rust-test/badge.svg?branch=master)](https://coveralls.io/github/redtankd/rust-test?branch=master)

I'm learning Rust!

## Code Coverage

### Installing Kcov

1. Dependencies

    ```
    brew install zlib bash cmake pkgconfig
    ```

2. pulling Kcov from Github

3. make and install

    ```
    cd /path/to/kcov/source/dir
    mkdir build
    cd build
    cmake \
      -DCMAKE_BUILD_TYPE=Release \
      -DCMAKE_INSTALL_PREFIX=/usr/local \
      ..
    make
    make install
    ```

4. Uninstall Kcov

    ```
    cd /path/to/kcov/source/dir
    cd build
    xargs rm < install_manifest.txt
    ```

### Installing cargo-kcov

```
cargo install cargo-kcov
```