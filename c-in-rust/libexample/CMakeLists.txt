cmake_minimum_required (VERSION 2.8)

project (libexample)

add_library (example add.c)

install(TARGETS example DESTINATION lib)