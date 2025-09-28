Rules of ownership:
- a variable owns a value, and when it goes out of scope, value drops(memory freed), and variable is inaccessible afterwards
reference can't own a value, just borrow it
- a variable going out of scope means end of enclosing curly brace, as well as value moved to some other variable
- types drop values recursively ie for a complex type values nested inside it also drop when it drops
- if a type implements copy trait, then assignment is plain copy, not movement and hence we have another copy of the value which
  is totally independent of original value
- variables (including function arguments) are dropped in reverse order, and nested values(tuples, arrays, struct etc) are dropped in source-code order

Rules of borrowing(borrowing happens via a reference, &T):
- either multiple immutable references(&T,also called shared references) OR a single mutable reference(&mut T, exclusive reference)
- once a value is borrowed, rust compiler would not allow any change in the value until borrower has gone out of scope or become invalid
- rust compiler is allowed to assume that the value a shared reference(&T) points to will not change while that reference lives
- rust compiler assumes that there are no other threads accessing the target value of a mutable reference, whether through a shared
  reference or a mutable one
- a slice reference is a non-owning pointer to a range of consecutive values

As a slice is of variable length, it cant be stored directly in a variable or passed as argument to function. Slices can only
be passed by reference.

Any value whose size can't be determined at compile time(like slice) can only be used via reference.
