melange design
============

# Language constructs

## Structure of a program

A melange program is defined as a possibly empty sequence of items. An item is one
of the following:
- a type definition
- a function definiton
- a constant definition

## Comments

Comments are written using the usual `//` marker.

## Modules 

TBD

## Types

### Primitive types

melange provides support for the following built-in primitive types.

| type  | type description               | values of N M | size (bits) | literal         |
|:------|:-------------------------------|:-------------:|:-----------:|:---------------:|
|`char` | characters                     | x             | 32          | `'c'`           |
|`uN`   | fixed-sized unsigned integers  | 8, 16, 32, 64 | N           | `100`           |
|`iN`   | fixed-sized signed integers    | 8, 16, 32, 64 | N           | `-101`          |
|`fN`   | floating-point numbers         | 32, 64        | N           | `100.01`        |
|`cN`   | complex floating-point numbers | 32, 64        | 2 * N       | TBD             |
|`bool` | boolean                        | x             | 8           | `true`, `false` |

### Pointers

Support for pointers is done through the usual `& : T -> *T` and `* : *T -> T`
operators, where `*T` is the type of a pointer to a value of type `T`. Hence,
`x: T` ⇒ `&x: *T` and `p: *T` ⇒ `*p: T`.

### Arrays

The type of an array of `N` elements of type `T` is `[T; N]`, and the elements
of the array are accessed with the subscript operator `[·] : [T; N] -> T`.
Then if `x: [T; N]`, then for all i, 0 <= i < N, `x[i]: T`.

### Tuples

Tuples are defined by a parenthesized, comma-separated list of types. If `T1`,
`T2`, ..., `TN` are types then `(T1, T2, ..., TN)` is a tuple made of those
types, and then if `x1: T1`, `x2: T2`, ..., `xn: TN` then `(x1, x2, ..., xn) :
(T1, T2, ..., TN)`. Individual values are obtained by destructuring the tuple.

### User-defined types

Every user-defined typed is declared using the `type` keyword with the
following, general schema:

```
type TypeName := ... .
```

#### Records

Records (structs) are declared by giving a list of (identifier, type) pairs :
```
type Person := (name: [char; 12]) (age: u32).
```
Fields with the same type can be gathered together, hence `type Point = (x: f64)
(y: f64).` and `type Point = (x y: f64).` declare the same type.

Fields are accessed using the `~` operator : `p: Person` ⇒ `p~age: u32`. They 
are private by default, but can be made public with the `pub` keyword` :
```
// `name` is accessible outside the module
type Person := (pub name: [char; 12]) (age: u32).
// both `x` and `y` and are accessible outside the module
type Point := (pub x y: f64).
// only `x` is accessible outside the module
type PointBis := (pub x: f64) (y: f64).
```
Record literals are written like function application with the record name as
the function name, i.e if `x: f64` and `y: f64` then `Point x y : Point`. A
record literal is private to the module unless the type is public and every
field is public.

Note that two records with the exact same fields are strictly different.

#### Enums

Enums are declared by writing a sequence of `|` separated variants, variants
being a name followed by a tuple type:
```
type Shape :=
	| Circle (Point, f64)
	| Triangle (Point, Point, Point)
	| Square (Point, Point).

type Orientation :=
	| Direct
	| Indirect.
```

If the type is public, then every variant and their field are made public.
Enum literals are written like function application, with the enum name and the
variant name as function name, separated by `~`, i.e. if `p: Point` and `x: f64`
then `Shape~Circle p x : Shape`.

Note that two enums with the exact same variants are strictly different.

#### Type aliases

If the rhs of a type declaration is a pointer type, an array type, a tuple type
or the name of an other type, then the type declaration creates a type alias
that is strictly equivalent to the RHS.

## Variables and destructuring

Variables are declared using the `let` keyword:
```
let x : i32 := 3.
```
If the type can be infered, then the type ascription can be ommited:
```
let y := 2.
```
Note: the default type for integer literals is `i32`, and for floating point
literals it is `f32`.

If the value on the rhs is a tuple or a record, the `let` binding can
be used to destructure it into simpler parts:
```
let p := Point 3.0 2.0.
let Point x y := p.
let (z, w) := (10, 20).
```
After destructuring, the original variable is destroyed.

## Functions

### Function declaration

Functions are declared with the `fun` keyword, a name, a list of (identifier,
type) pairs, possibly a return type, and a function body delimited by `begin`
and `end`.
```
fun new_point_with_a_tweak (x y: f64) : Point 
begin
	let z := x + y.
	let w := x - y.
	return Point (z + w) (z-w).
end
```
If the body consists of a single expression, the `begin`-`end` pair can be
replace by a simpler `:=`-`.` pair.
```
fun vertical_point (y: f64) : Point := Point 0.0 y.
```

One can also declare functions using function literals:
```
let vertical_point := (y: f64) => Point 0.0 y.
```

Both methods produce en element of type `f64 -> Point`. In general, a function
with `N` arguments of types `T1`, `T2`, ... , `TN` and return type `TR` is of
type `T1 -> T2 -> ... -> TN -> TR`.

Note: the type of a function with no arguments is `! -> TR` and the type of a
function with no return type is `T1 -> ... -> TN -> !`.

### Function calls

As seen with struct and enum literals, function application is done using
whitespace:
```
let p01 := vertical_point 1.0.
let p22 := new_point_with_a_tweak 1.0 1.0.
```
Functions can be curried, i.e. partially applied to create new functions:
```
let vertical_point : f64 -> Point := Point 0.0.
```

## Control flow

### Blocks

As seen with functions, code blocks containing a sequence of statements can be
created using a `begin`-`end` pair.

### Pattern matching

A pattern is a type constructor, used to decompose variables of that type.

NOTE: To be expanded with `or` and `and` operators.

#### `let` destructuring

In assignements, patterns must be irrefutable, so only arrays, tuples and records may
be pattern-matched.

```
let p := Point 2.0 3.0.
let Point x y := p.
```

#### `is` expressions

An another construct is the `is` expression, that tests whether a variable
matches a certain predicate and returns a boolean (`true` if it matches, `false`
if not). Patterns do not need to be irrefutable.

```
let s := Shape~Circle (Point 0.0 0.0) 0.0.
let is_it := s is Shape~Circle (Point 0.0 y) r.
```

#### the `_` pattern

The `_` symbol can be used to match anything and discard its value.

### Branching

#### `if` expressions

The classic `if` expression exists. Given a boolean, it executes the `then`
branch if the boolean is true, and jumps to next branch (if it exists) if it is
not.

```
if x == y then
	do_something.
end
```

`if` expressions can have an `else` branch as a fallback.

```
if x == y then 
	do_something.
else
	do_something_else.
end
```

Before falling back to the `else` branch, there can be other `elif` branches (no
need to nest `if` expressions).

```
if x == y then 
	do_something.
elif x == z then
	do_something_else.
else
	really_do_something.
end
```

If an `is` expression is used as the condition, the variables used in the
pattern will be moved to the `then` branch (except if they are `copy`). If the
predicate does not match, the variables in the expression will not be destroyed
and can be used in other branches. However, they will be destroyed at the end of
the `if` expression (again, except if it is `copy`).

NOTE: To be expanded with `or` and `and` operators.

```
if s is Shape~Circle (Point 0.0 y) r then
	do_something_with y.
else
	do_something_with_else_with s.
end
```

`if` expressions are... expressions.
```
let x := if 2 < 3 then 42 else 420 end.
```

#### `match ... with` expressions

Another branching control flow operation is the `match` expression. It will try
to pattern-match the provided expression to one of the different patterns (all
patterns must be exclusive, the list of patterns must be exhaustive).
```
let new_s := match (s, o) with
| (Shape~Triangle a b c, Orientation~Direct) => Shape~Triangle c a b
| (Shape~Triangle a b c, Orientation~InDirect) => Shape~Triangle b c a
| _ => s
end.
```

#### `guard` expressions

TBD

### Loops

#### `loop` loops 

```
loop 
	print x.

	if 2 < 3 then
		break.
	end
end
```

#### `for` loops

```
for x in y do
	break.
end
```
NOTE: `x` can be an irrefutable pattern.

# Moves & copies

melange follows the move semantics of Rust.

By default, memory is moved. That is, if a variable is used in an expression (a
function call, an arithmetic operation, assigned to another variable), then
after that operation, the data can no longer be accessed through that variable.

However, if a type is marked as `copy`, then the data will no longer be moved
around but copied. In that case, if a variable is used in an expression, it will
still be usable after that.

To mark a type as `copy` : TBD
