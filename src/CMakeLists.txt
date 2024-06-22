# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(markdown2html_library VERSION 0.0.1 LANGUAGES C)

add_library(markdown2html SHARED
  "markdown2html.c"
)

set_target_properties(markdown2html PROPERTIES
  PUBLIC_HEADER markdown2html.h
  OUTPUT_NAME "markdown2html"
)

target_compile_definitions(markdown2html PUBLIC DART_SHARED_LIB)
