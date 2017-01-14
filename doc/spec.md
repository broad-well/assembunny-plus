# Assembunny-Plus Language Specification

## <a name="toc" /> Table of Contents

* [(1) Description](#1)
  * [(1.1) Assembunny](#1.1)
  * [(1.2) Assembunny-plus](#1.2)
  * [(1.3) Compatibility with Assembunny](#1.3)
  * [(1.4) Terminology](#1.4)
* [(2) Registers](#2)
	* [(2.1) In CPUs](#2.1)
	* [(2.2) In Assembunny-plus](#2.2)
	* [(2.3) Restrictions](#2.3)
* [(3) Keywords](#3)
	* [(3.1) Definition](#3.1)
	* [(3.2) General Usage](#3.2)
	* [(3.3) List of Keywords](#3.3)
* [(4) Using this program](#4)
  * [(4.1) Interactive Interpreter](#4.1)
  * [(4.2) Interpreting a File](#4.2)
  * [(4.3) Compiling a File to C](#4.3)

<hr/>

## <a name="1" /> Description

_Assembunny-plus_ is a programming language extended from the _Assembunny_ concept in [Advent of Code 2016](https://adventofcode.com/2016); _Assembunny_ was first mentioned on [Day 12](https://adventofcode.com/2016/day/12). Just like _Assembunny_, _Assembunny-plus_ is similar to [Assembly](https://en.wikipedia.org/wiki/Assembly_language). Its official implementation is an interpreter and a compiler _(compiles to C source code)_ in [Rust](https://rust-lang.org).

### <a name="1.1" /> Assembunny

Assembunny is a programming language specification defined in the programming puzzle for [Day 12 of Advent of Code](https://adventofcode.com/2016/day/12). It is used several times throughout [Advent of Code 2016](https://adventofcode.com/2016) (including Day 25) for users to implement.

### <a name="1.2" /> Assembunny-plus

Frowned upon Assembunny's lack of features, `broad-well <michael@mcmoo.org>` decided to extend upon it to create a new programming language that includes the features he was anticipating in Assembunny.

Added features include:
- `MUL` keyword for multiplication
- `DIV` keyword for division
- `OUTN` keyword for printing value to STDOUT plus a newline
- `OUTC` keyword for printing character based on char code to STDOUT
- `DEF` keyword for defining new registers
- `INCT` keyword for adding a value to a register
- `DECT` keyword for subtracting a value from a register

### <a name="1.3" /> Compatibility with Assembunny

Assembunny-plus is mostly compatible with Assembunny, with a few exceptions:
- `DEF` is required for any register. In order to keep all Advent of Code puzzles compatible with Assembunny-plus, the following lines should be prepended to the puzzle inputs:
```
def a 0
def b 0
def c 0
def d 0
```
- The `TGL` keyword, introduced in [Advent of Code 2016 Day 23](https://adventofcode.com/2016/day/23), is not implemented yet.
