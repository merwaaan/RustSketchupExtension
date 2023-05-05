cmake_minimum_required(VERSION 3.23)

project(RubyRustBridge)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(SOURCES
  cppbridge/main.cpp
  cppbridge/RubyLib.h
)

add_library(RubyRustBridge SHARED ${SOURCES})

set_target_properties(RubyRustBridge PROPERTIES
  PREFIX "" # No "lib" prefix
  DEBUG_POSTFIX "" # No "d" debug suffix
  OUTPUT_NAME RustSketchupTest
)

if (WIN32)
  set_target_properties(RubyRustBridge PROPERTIES SUFFIX ".so")
else()
  set_target_properties(RubyRustBridge PROPERTIES SUFFIX ".bundle")
endif()

include(${CMAKE_SOURCE_DIR}/_deps/ruby/import.cmake)

target_link_libraries(RubyRustBridge
	"ruby2.7.1"
	"${CMAKE_SOURCE_DIR}/target/debug/rust_sketchup_test.lib"
)

install(
	TARGETS RubyRustBridge
	DESTINATION "${CMAKE_SOURCE_DIR}/extension/RustSketchupTest"
)
