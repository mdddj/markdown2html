cmake_minimum_required(VERSION 3.10)
set(PROJECT_NAME "markdown2html")
project(${PROJECT_NAME} LANGUAGES CXX)
include("../cargokit/cmake/cargokit.cmake")
apply_cargokit(${PROJECT_NAME} ../rust markdown2html "")
set(markdown2html_bundled_libraries
  "${${PROJECT_NAME}_cargokit_lib}"
  PARENT_SCOPE
)