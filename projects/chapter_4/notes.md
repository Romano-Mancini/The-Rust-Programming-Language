# Chapter 4 Notes

## Safeness

A program is safe when it can't have undefined behavior. Rust checks at compile time for those behaviors, such as referencing an undefined variable.
So, Rust wants to make programs safe at runtime, so that:

1. Released software is more reliable;
2. You don't need to do checks at runtime that might slow down the application.

## Ownership

The owner of a box on the heap is the last defined pointer to it. When that pointer is deallocated (its stack frame is dropped) then the box is deallocated on the heap. That's the reason why we need this concept of ownership: it basically skips the need for a garbage collector.

## Important rules

1. Only one variable can own something in the heap at a time.
2. If a variable sees its ownership moved, then that variable can't be accessed anymore.
3. Movement of ownership happen on assignments and function calls.
4. Data should never be aliased and mutated at the same time. Referencing is a good way to pass a reference without the change of ownership. But those aliases need to be destroyed before any modification is possible.

## References

References remove the permission to write from the original variables during their lifetime. A reference's lifetime is the range of code spanning from its birth (where the reference is created) to its death (the last time(s) the reference is used).
