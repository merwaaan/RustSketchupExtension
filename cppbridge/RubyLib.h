#pragma once

// The purpose of this file is to control the damage done by Ruby overriding common names
// with its own definitions, which creates an endless variety of conflicts with other libraries.
//
// By wrapping the actual Ruby inclusion around our own #defines and only including
// this wrapper, we can at least selectively manage such conflicts.


// We have to define those or Ruby will be a pain and generate name clashes
#define HAVE_ACOSH 1
#define HAVE_CBRT 1
#define HAVE_ERF 1
#define HAVE_ROUND 1
#define HAVE_TGAMMA 1
#define HAVE_ISINF 1
#define HAVE_ISNAN 1
#define HAVE_FINITE 1
#define HAVE_ISFINITE 1
#define RUBY_DONT_SUBST 1

// Another issue: Ruby will try to redefine isnan if it's not already defined,
// which will break all the other libraries that use this name
// (undef'd down below)
//#define isnan 

#undef STRINGIZE // Would generate warnings as it's already defined in MathGeoLib

#include <ruby/ruby.h>

#ifdef HAVE_RUBY_ENCODING_H
    #include <ruby/ruby/encoding.h>
#endif


#undef isnan
#undef int128_t // Conflits with fmt which is an indirect dependency (via OIIO, spdlog)
#undef uint128_t
#undef ssize_t // Yet another issue u_u Ruby will define this but other libraries use this name (eg. Embree)

#undef STRINGIZE

/*
 * Need to be very careful about how these macros are defined, especially
 * when compiling C++ code or C code with an ANSI C compiler.
 *
 * VALUEFUNC(f) is a macro used to typecast a C function that implements
 * a Ruby method so that it can be passed as an argument to API functions
 * like rb_define_method() and rb_define_singleton_method().
 *
 * VOIDFUNC(f) is a macro used to typecast a C function that implements
 * either the "mark" or "free" stuff for a Ruby Data object, so that it
 * can be passed as an argument to API functions like Data_Wrap_Struct()
 * and Data_Make_Struct().
 */

#define VALUEFUNC(f) ((VALUE (*)(ANYARGS)) f)
#define VOIDFUNC(f)  ((RUBY_DATA_FUNC) f)
