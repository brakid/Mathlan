# Mathlan
* Stack-based
* Reverse Polish Notation
* Turing-complete

Inspired by Forth: [Wikipedia](https://en.wikipedia.org/wiki/Forth_(programming_language))

### Implemented:
* heap memory (store load) -> 1 10 store (write value 10 into byte 1 on the heap)
* modulo - allows to split i64 values into u8 chunks to store and load
* labels & function calls - functions need to end with 'ret' keyword
* comments
* includes - for a rudimentary stdlib of functions
* translating code to C for native compilation (using forward declarations for function calls)
* support for nested ifs - else branches can be skipped
* not -> if 0 -> 1, else: -> 0 - logical operators (&& || not)
* const string support
* arbitrary stack rolls [a b c] 3 roll -> [b c a]
* malloc function - obtain n bytes of memory - linearly allocate new memory - never free

## Missing features
* read inputs - getc