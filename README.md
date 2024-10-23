# Tiny Rust libc

## Introduction

This is a _tiny_ libc implementation, mostly (but not entirely) written in the Rust programming language. It is useful for bare-metal embedded Rust applications that need a C library (maybe because of some third-party library written in C they want to use) but don't want to link against a full [newlib](https://sourceware.org/newlib), or who tried but had trouble with both newlib and [compiler_builtins](https://github.com/rust-lang-nursery/compiler-builtins) defining symbols like `memset`.

This crate basically came about so that the [nrfxlib](https://github.com/NordicPlayground/nrfxlib) binary interface library for the nRF9160 would work with Rust.

## Implemented so far

* abs
* strol
* atoi
* isspace
* isdigit
* isalpha
* isupper
* memchr
* strcmp
* strncmp
* strncasecmp
* strcat
* strcpy
* strncpy
* strlen
* strtol
* strtoll
* strtoul
* strtoull
* strtoimax
* strtoumax
* strstr
* strchr
* strrchr
* snprintf
* vsnprintf
* qsort
* alloc (optional)
    * malloc
    * calloc
    * realloc
    * free
* signal (optional)
    * signal
    * raise
    * abort

## Non-standard helper functions

* itoa
* utoa
* rand_r

## To Do

* Anything else nrfxlib needs
* Anything anyone is prepared to submit

## Licence

As this is going to be a bunch of bits taken from all over the place (some newlib, some relibc, etc), each function has its own file and each file has its own licence. Any new licences should be appended to the [LICENCE.md](./LICENCE.md) file.

