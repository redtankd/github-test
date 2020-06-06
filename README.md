README
===========

[![Build Status](https://travis-ci.org/redtankd/rust-test.svg?branch=master)](https://travis-ci.org/redtankd/rust-test) [![Coverage Status](https://coveralls.io/repos/github/redtankd/rust-test/badge.svg?branch=master)](https://coveralls.io/github/redtankd/rust-test?branch=master)

I'm learning Rust!

## Code Coverage

### Solution 1: grcov

Only work in nightly now.

#### Installing grcov

```
cargo install grcov
```

#### Testing

```
export CARGO_INCREMENTAL=0

# Nightly
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"

cargo +nightly test --all

grcov . -o ./target/debug/coverage/ -s . -t html --llvm --branch --ignore-not-existing --excl-start "#\[test\]"
```

### Solution 2: kcov

Doesn't work in Mac OS

#### Installing kcov

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

#### Installing cargo-kcov

```
cargo install cargo-kcov
```