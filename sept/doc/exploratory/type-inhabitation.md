# Notes on Type Inhabitation

These are organized based on date, because these notes are often "thinking out loud", and earlier conceptualizations may be overridden by later developments.

## 2021.12.25
-   Regarding how type inhabitation is implemented, apropos the concept of an abstract type of a term. Abstract type is meant to be separate from concrete type (aka representational type).  Abstract type conveys more semantic meaning than concrete type (an unsigned 32 bit int is the concrete type of many different things with different semantics).  In order to allow for abstract types to be "open" in the sense of allowing future terms to inhabit them, the definition of inhabitation should be done within the term itself.  Though the challenge in Rust is, because of its coherency rules not allowing multiple definitions or re-definitions of traits, how to be able to add certain existing types to more abstract types in the future.

    One approach would be to have a mode of defining certain kinds of type inhabitation only in the runtime. Doing this would be a bit confusing because the analogous compile time data types wouldn't necessarily obey the same data model.  Though if it's possible to bridge the gap between a compile-time-defined term (e.g. `Vec<String>`) and its runtime counterpart, then type inhabitation for a compile-time-defined term would be consistent.

    Type inhabitation in the runtime is necessarily defined in terms of pairs of concrete types.  But in order to support type inhabitation in the runtime on parametric concrete types (e.g. `Vec<T>` for an arbitrary type `T` that's recognized by the runtime), some fancier scheme is needed).  In general, type inhabitation would be determined by pairs of maps `(term: Fn(P1,...,Pn) ->

    x inhabits T
    x = Vec<String>
    T = ArrayE<String>

    inhabits(x, T) would first look up the parameterized concrete type of each x and T.  Perhaps first it looks up the concrete type of each, then checks if each concrete type is parameterized,

    want to have compile-time and runtime analogs

    Compile time would be something like Inhabits<Vec<T>,ArrayETerm>::inhabits(x: &Vec<T>, t: &ArrayETerm) whereas
    runtime would be something like

        inhabits(x, t) := concrete_inhabits(x, ArrayTerm) &&
                          abstract_inhabits(t, ArrayETerm) &&
                          all(concrete_inhabits(e, ArrayETerm::element_type_of(t)) for e in ArrayTerm::elements())

-   Going further afield, but as a natural extension of the data model, abstract data types are effectively interfaces.  For example, ArrayE has inhabitants of the form ArrayE(T) for some type T.  It has a number of associated functions, such as retrieving the element type, retrieving the length, and getting a reference to an element.  Then abstract inhabitation of ArrayE(Z) (where Z is the element type) could be defined as x inhabits T if x inhabits ArrayTerm, T inhabits ArrayE, and ArrayTerm::element_type_of(x) == ArrayE::element_type_of(T).  As phrased here, this assumes (just as Rust does), probably due to the lack of concise expressiveness in text-based source code, that there is only a single ArrayTerm and ArrayE structure on x and T respectively (in Rust terms, this is the "coherence rule" in which a type can only have one implementation of a given trait).
    -   If there was a single implementation of a given abstract type (interface/trait), then it could be automatically used.
    -   If there were multiple, then which one would have to be specified.
