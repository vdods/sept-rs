# Diffing and Merging SEPT Data

For two terms, `a` and `b`, having types `A` and `B` (which may or may not be the same), there is a notion of a diff between them, which expresses the transformation taking `a` to `b`.  What the diff is as a piece of typed data depends on `A` and `B`, and potentially `a` and `b`.  A diff could be made to be invertible, meaning that it must contain enough information to be able to construct its own inverse transformation.  A diff should be "minimal" in some sense, meaning no unnecessary information is present, so it's an efficient encoding of the transformation.  Let the diff in discussion be the transformation

    a => b

meaning that the values of `a` and `b` are known, as well as their types `A` and `B`, and what's being determined is the diff itself; the transformation that produces `b` from `a`.

Let "posidiff" denote a one-way, "forward" transformation of one term to another.  Let "negadiff" denote the one-way transformation in the other direction, i.e. the inverse of the posidiff.  Maybe call an invertible diff an "iso-diff" (analogous to isomorphism).

Kinds of diffs
-   `A` and `B` are any types (i.e. no other information).  Transformations:
    -   Assign
        -   Posidiff: assign `b`
        -   Negadiff: assign `a`
    -   Type isomorphism `phi` (i.e. `A` and `B` are types having a "natural" isomorphism)
        -   Posidiff: apply `phi`
        -   Negadiff: apply `phi`-inverse
        -   Examples:
            -   `i32 <-> u32` as a bitwise conversion
            -   A given permutation of a tuple
    -   Type monomorphism `iota` (i.e. `A -> B` is an injective transformation)
        -   Posidiff: apply `iota`
        -   Negadiff: assign `a` (this assumes no knowledge about how to invert `iota`; there are stronger
            assumptions that would allow a more efficient negadiff using less information than `a` itself,
            such as a fibration of `B` over `A`)
    -   Type epimorphism `pi` (i.e. `A -> B` is a surjective transformation)
        -   Posidiff: apply `pi`
        -   Negadiff: assign `a` (this assumes no knowledge about how to invert `pi`; there are stronger
            assumptions that would allow a more efficient negadiff using less information than `a` itself,
            such as a privileged function `B -> A` that inverts `pi`, i.e. a section of `A` over `B`)
-   `A` and `B` are both equal to `T`, which is a sequence of untyped terms (i.e. a tuple).  Transformations on this type:
    -   Assign to `n`th element
        -   Posidiff: set `n`th element to `b[n]`
        -   Negadiff: set `n`th element to `a[n]`
    -   Remove all elements (aka Assign from empty tuple)
        -   Posidiff: assign `()`
        -   Negadiff: assign `a`
    -   Remove `n`th element
        -   Posidiff: remove `n`th element
        -   Negadiff: insert `a[n]` before `n`th element
    -   Insert before `n`th element
        -   Posidiff: insert `b[n]` before `n`th element
        -   Negadiff: remove `n`th element

## Diffs of Inductive Types

Inductive types are types composed from other types, for example structs, arrays, or tuples.  The diffs on an inductive type should have a corresponding inductive definition.

TODO

## Some Applications of Structured Diffs

Having transformations that respect the structure of the data (unlike text-based diffs) allows much more automation in related processing.
-   One particular application of interest is a tool that would allow one to more literally factor source code.  Programs are highly structured data, and therefore are conducive to being represented as sept data.

    One could diff two similar functions and see what code is in common, what the non-generic parts are (because those are the differences), and what would be subject to parameterization in the factorization.  It could be possible to further automate this factorization process in special cases.

-   Another interesting application is automatically producing formal migrations between data schema (e.g. database table schemas, data types within a program, or API data types).  Having automatically derived conversions between different versions of data would allow automatic generation of "glue" code that adapts different versions of the same schema, so that some classes of backward-incompatible software/data changes are eliminated.
