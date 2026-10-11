# Mathlan
* Stack-based
* Reverse Polish Notation
* Turing-complete

Inspired by Forth: [Wikipedia](https://en.wikipedia.org/wiki/Forth_(programming_language))

### Implememnted:
* heap memory (store load) -> 1 10 store (write value 10 into byte 1 on the heap)
* modulo - allows to split i64 values into u8 chunks to store and load
* labels & function calls
* comments
* includes - for a rudimentary stdlib of functions
* translating code to C for native compilation (using forward declarations for function calls)
* support for nested ifs - VM requires else branches (even if empty)
* not -> if 0 -> 1, else: -> 0 - logical operators (&& || not)

## Missing features
* read inputs - getc