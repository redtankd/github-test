var ffi = require('ffi');

var lib = ffi.Library('rust/target/debug/libdouble_input', {
    double_input: ['int', ['int']]
});

console.log(lib.double_input(10));
