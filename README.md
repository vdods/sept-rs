# sept-rs

Rust SDK for Structured Expression Project Toolkit

## To-dos

-   Create a runtime for the term/type system.
    -   Store a poset of types for efficient type computations (inhabitation, subtype, supertype, common subtype,
        common supertype, etc).
    -   Will eventually need to handle schemes of types, meaning that there are parameterized families
        (potentially infinite in size) of terms/types that need to be handled without needing to actually
        instantiate them.
    -   Particular types
        -   All common types that one would encounter in a programming language
            -   Integer types
            -   Floating point types
            -   Boolean
            -   Array types
            -   Map types
            -   Struct metatypes
            -   Tuple types
        -   Semantic subtypes
        -   Pointer/Reference types (references have referential transparency, pointers do not)
            -   In-memory
            -   On disk
            -   URL
            -   etc
-   Implement serialization for the term/type system.
    -   Eventually implement "projected" serialization, in which a given piece of data can be projected
        into a "known context" component and a "data" component.  E.g. if the element type and size of
        an array are known, then it's not necessary to serialize either of those values.
-   Research, design, and implement formal semiotic scheme where context is formally treated, separately
    from the subject data (as opposed to being an implicit, second-class consideration as it is in
    practically every human exposition).
