# To-dos

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
-   Develop and implement the groupoid theory of data types and their diffs.  In particular, the objects are
    terms and the morphisms are transformations between terms.  Diffs can be expressed as invertible
    transformations.  Diffs are associative, as required by the groupoid structure.  An important factor
    is when two diffs are commutative, meaning they have a single, well-defined "merge".  Use all of this
    to develop diff and merge tools for sept data.
-   Implement the category theory of type identifications, projections, and inclusions.  The goal would be
    to have an automatic and formally verifiable method for deriving the "glue" between related types,
    reducing boilerplate.

## Low-Level To-dos

-   Maybe split things up into a static types module `st` and dynamic types module `dy`, since there are essentially
    two analogous sides to the sept data model.  Then there would be compile-time and runtime versions of each
    trait, e.g. TermTrait, TypeTrait, etc.
-   Implement macros for deriving traits, which will clean a ton of boilerplate up.
-   Change StructTermTerm::type_ into a dy::Value to generalize.
-   Implement st::TransparentRefTrait (contrast with dy::TransparentRefTrait) where the dereferenced type
    is specified.  This would be suitable for typed references.  Implementing full dereference (of nested
    references) will be tricky because it requires knowing or specifying all the types in the deref sequence.
-   Implement `Clone` for `dy::Value`.
