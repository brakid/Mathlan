# Mathlan
* Stack-based
* Reverse Polish Notation
* Turing-complete

Inspired by Forth: [Wikipedia](https://en.wikipedia.org/wiki/Forth_(programming_language))

### Implememnted:
* heap memory (store load) -> 1 10 store (write value 10 to byte 1 on the heap)
* modulo - allows to split i64 values into u8 chunks to store and load
* labels & function calls
* comments

## Missing features
* not -> if 0 -> 1, else: -> 0
* nested loops & ifs - can be simulated by function calls