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
            -   String type(s)
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
    trait, e.g. `TermTrait`, `TypeTrait`, etc.
-   Implement macros for deriving traits, which will clean a ton of boilerplate up.
-   Implement `st::TransparentRefTrait` (contrast with `dy::TransparentRefTrait`) where the dereferenced type
    is specified.  This would be suitable for typed references.  Implementing full dereference (of nested
    references) will be tricky because it requires knowing or specifying all the types in the deref sequence.
-   A `StructTermTerm` is really a kind of typed `TupleTerm`.  Maybe this should be implemented using semantic
    subtypes instead, where generic data is given additional semantic meaning via an associated semantic type
    or marker.
-   Apparently if a rust type is declared as `pub struct X;` then it can be instantiated as `let x = X;` instead
    of `let x = X{};`.  Though apparently this doesn't work for type aliases (error is "can't use a type alias
    as a constructor").  In particular, `Float32`, `Float64`, `Sint8`, `Uint8`, etc are type aliases, so this
    wouldn't work for them.
-   Decompose `st::TermTrait` into traits `st::AbstractTypeOf`, `st::IsParametric`, and `st::IsType`, where
    they don't depend on any `&self` parameter.  Also define `dy` versions of each of these.  Jamming:

        trait dy::AbstractTypeOf {
            fn abstract_type(&self) -> dy::Value;
        }
        trait st::AbstractTypeOf {
            type AbstractTypeFnReturnType;
            fn abstract_type(&self) -> Self::AbstractTypeFnReturnType;
            // This would have a blanket implementation of dy::AbstractTypeOf
        }
        trait st::NonParametricAbstractTypeOf {
            type NonParametricAbstractTypeFnReturnType;
            fn non_parametric_abstract_type() -> Self::NonParametricAbstractTypeFnReturnType;
            // This would have a blanket implementation of st::AbstractTypeOf
        }
        trait dy::IsParametric {
            fn is_parametric(&self) -> bool;
        }
        trait st::IsParametric<const IS_PARAMETRIC: bool> {
            fn is_parametric(&self) -> bool {
                IS_PARAMETRIC
            }
            // This would have a blanked implementation of dy::IsParametric
        }
        trait dy::IsType {
            fn is_type(&self) -> bool;
        }
        trait st::IsType<const IS_TYPE: bool> {
            fn is_type(&self) -> bool {
                IS_TYPE
            }
            // This would have a blanket implementation of dy::IsType
        }