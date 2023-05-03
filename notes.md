# vocab and concepts

## Scalar types
- scalary types =  a single value.
- rust has 4 primary: integers, floats, booleans, characters.
- ints can be signed (negative possible) or unsigned

## Compound types
- tuples and arrays
- tuples group a number of values with a variety of types into a single compound type
- tuples have a fixed length -> cannot grow/shrink after declared
- can destructure the tuple with smooth brackets
- arrays are useful when you want your data on the stack rather than the heap (???)
- arrays are different that vectors. arrays = fixed size. vectors = flexible, like a js array

## Functions
- typed args are mandatory
- statements perform an action, do not return a value
- expression evaluate to a resultant value
- so in js, we use functions for both of these cases and return a value, carry out an action, or both.
- expression do NOT have semicolons at the end

## Control flow
### if/else
- if statement conditions must be booleans
- if x = 3, we cannot do if x { do y } like in js
- but rather if x === 3 { do y }
- if we get caught in if else hell, we should look to use match instead
- which is sort of like a switch statement
- can use if/else within a statement b/c it's an expression
- if/else arms must be the same type

### loops
- rust has three loops -- loop, while, for
- loop label look like `'counting_up: loop`
- can be used to specific which loops to `break` or `continue`
- `break 'counting_up;`

## ownership

### memory and allocation
- 1 `allocate` must equal 1 `free`
- rust automatically calls a function called `drop` when variables go out of scope

```
  let s1 = String::from("hello");
  let s2 = s1;
```

- in the following example, `s1` is no longer valid, and the info is `moved` into `s2`
- must called `clone()` to deep copy a var

### references and borrowing
- `&` used for referencing vars
- `*` de-references
- this way we can use the var's value without transfering ownership
- basically creates a pointer that points to the variables pointer
- references are obviously read only unless specificed with `mut`
- when using mutable references, they are the only references that can exist to that specific value (within the same scope)