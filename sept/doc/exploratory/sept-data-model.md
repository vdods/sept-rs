# SEPT Data Model

-   The `sept` data model defines an understandable framework for defining and using data types that shouldn't be very surprising to most programmers.
-   Because most programming languages don't allow types as first class data, there has to be a sept runtime which provides much of the data model.  In fact, the main body of the data model must be fully supported by the runtime.  Language-specific aspects of the data model can be implemented (e.g. compile-time determination of types), but by construction this can't be supported by all languages.
-   Idea (bit of a can of worms): Have optional language-specific code generation to facilitate things like:
    -   Ingestion of a schema from some external source (e.g. an API, data type, function signature, etc) and production of code that implements that schema so that it can be in language-native form, instead of only through the sept runtime.
    -   Given a bunch of runtime-registered data types and relationships, generate a bunch of code that provides all of that in native language constructs at compile time.

## Abstract vs Concrete Types

Terminology:
-   A "term" is any representable thing within the data model.  This includes both values and types.
-   A "type" is a special kind of term that has a notion of being inhabited by some collection of terms.
-   A "concrete type" is a type which specifies the representation of a collection of terms.  A concrete type doesn't carry any semantic information.  In general, it's either an atomic concrete term (one of the basic data types available, e.g. uint32, float32, etc) or a composition of concrete terms (using one of the basic data structures available, e.g. array, ordered map, tagged enum, etc).
-   An "abstract type" is a type which provides semantic information for a collection of terms.  An abstract type might have nothing to do with the representation of its terms, or it might imply a specific concrete type.

Abstract vs Concrete types is analogous to interface vs representation.  The concrete type of a term is

## Semantic Terms and Semantic Subtyping

A semantic term is a pair `(<term>, <semantic-type>)`, where the value (and therefore concrete type) is carried by the concrete term and the semantic information is carried by the abstract type.  The abstract type is useful for the human programmer, as well as the type-checking compiler.  Once the compiler has generated asm/machine code, the abstract type information is stripped away, as it's totally unnecessary for execution of the code.

A semantic subtype is a pair `(<type>, <semantic-type>)`.

Let `x` be the semantic term above.

    concrete_type_of(x) := concrete_type_of(<concrete-term>)
    semantic_type_of(x) := <semantic-type>
    abstract_type_of(x) := `(<concrete-type>, <semantic-type>)`.

Misc jamming notes:
-
-   A tuple is a semantic subtype of Array (where the elements have no type restriction).


ConcreteTerm
ConcreteType
ConcreteAtomicTerm -- instances of e.g. int32, uint64, float32, bool, etc
ConcreteAtomicType -- the types themselves, e.g. int32, uint64, float32, bool, etc.
ConcreteInductiveTerm -- instances of e.g. Vec<int32>, (float32, bool), struct { ... }, etc.
ConcreteInductiveType -- the types themselves.

An abstract type doesn't necessarily imply a concrete type, but might, e.g. Sha256Sum could imply ArrayES(Uint8, 32).
A pure abstract type doesn't imply a concrete type, e.g. SignedInteger (could be any bit size)
A semantic term is a pair (<concrete-term>, <abstract-type>).
A semantic class might not even imply an abstract type, e.g. Uninitialized or Unvalidated or Untrusted.
A pure semantic class does not imply an abstract type.
Is a non-pure semantic class the same as an abstract type?  E.g. EmailAddress would imply some sort of ascii string?

TODO: Idea: create a TermTrait method that turns a term into its runtime equivalent

Given that Array is an abstract type, meaning that it's open to inhabitation by possibly many concrete types, this effectively defines an interface.  This is analogous to a mathematical structure in the sense that it's a collection of certain typed things.  For example, type `T` has an `Array` structure (i.e. implements the `Array` interface) given

    len : T -> Uint
    element_type : T -> Type
    element : (x:T, i:[0, len(x))) -> element_type(x) // This requires dependent function types

so mathematically, the structure would be a 4-tuple

    (T, len, element_type, element)

with the correct types and relationships.  There could be more methods added to the interface, such as retrieving a view (which would itself be a term of `Array`), or mutating an element.

## Type Inhabitation

## Layers of Data Specification

From lowest to highest level of abstraction:
-   Layout: This dictates the specific layout of bits/bytes in memory/on disk of the given data, including alignment information.
-   Concrete type: The particular representation of the given data (e.g. float32, uint64, null-terminated string, structs composed of concrete types, etc.)
-   Abstract type: Semantic information about the data.
