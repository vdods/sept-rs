# To-dos

-   Create a runtime for the term/type system.
    -   Store a poset of types for efficient type computations (inhabitation, subtype, supertype, common subtype, common supertype, etc).
    -   Will eventually need to handle schemes of types, meaning that there are parameterized families (potentially infinite in size) of terms/types that need to be handled without needing to actually instantiate them.
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
    -   Eventually implement "projected" serialization, in which a given piece of data can be projected into a "known context" component and a "data" component.  E.g. if the element type and size of an array are known, then it's not necessary to serialize either of those values.
-   Research, design, and implement formal semiotic scheme where context is formally treated, separately from the subject data (as opposed to being an implicit, second-class consideration as it is in practically every human exposition).
-   Develop and implement the groupoid theory of data types and their diffs.  In particular, the objects are terms and the morphisms are transformations between terms.  Diffs can be expressed as invertible transformations.  Diffs are associative, as required by the groupoid structure.  An important factor is when two diffs are commutative, meaning they have a single, well-defined "merge".  Use all of this to develop diff and merge tools for sept data.
-   Implement the category theory of type identifications, projections, and inclusions.  The goal would be to have an automatic and formally verifiable method for deriving the "glue" between related types, reducing boilerplate.
-   It should be possible to use symbolic and URL-like references to types in the following ways:
    -   In a dynamic mode, declaring a term has a symbolic/URL-specified type simply does dynamic validation.
    -   In a static mode, a natively defined struct/class could have a derive attribute that verifies (maybe at load-time?) that the given struct/class faithfully represents the type referred to.  This would be the ideal way to implement strongly typed, statically compiled usages of schemas that are defined in some public, online repository.
-   Rename types involving acronyms to use all caps, e.g. `Utf8String` -> `UTF8String`, `PodType` -> `PODType`, because it looks better.

## Design Notes For `sept-tui`

-   Goals
    -   Create a TUI (Textual User Interface) editor for sept data, analogous to a text editor for text.
    -   Achieve a sept-data-equivalent of the [LyX](https://www.lyx.org/) editor, which will be used as a reference and usability benchmark.  In particular, with Lyx you edit a document that looks similar to the final product, and more importantly, keyboard shortcuts allow one to type mathematics (to give a particular example) as fast as one could write it by hand.  The same should be true of `sept-tui`.
    -   Facilitate creation, editing, viewing, search, filtering, and structured manipulation of sept data in a way that is reasonably comprehensible to someone who understands a text editor or programming IDE, in particular, the LyX typesetting program.
    -   Even though full usability through keyboard only is a primary goal, the mouse should still be available.
    -   Use a model-view-controller architecture so that the view of the data is as flexible as the user's needs.

### Specific Examples/Notes for Model-View-Controller:

-   Toggle between compact and verbose viewing modes.  Compact would tend toward in-line, abbreviated, truncated, simplified.  Verbose would tend toward expanded, fully specified, complex.
-   Provide choosable, data-specific view/edit modules, e.g.
    -   Integers
        -   Binary, decimal, hex
    -   Numeric values
        -   Linear scale
        -   Log scale
        -   "Human-readable" log scale, e.g. `3.4k` or `8M` or `1.22B`
    -   Collections of numeric values
        -   As a list of numbers
        -   As a graph
        -   As a histogram
        -   As a basic statistical summary
        -   As a best-fit Gaussian distribution
        -   As a classification (e.g. of clusters)
    -   Timestamps
        -   Formal, e.g. ISO 8601 (e.g. `2006-08-14T02:34:56-06:00`) or the like
            -   In UTC or a particular timezone
            -   In local time
        -   Informal timestamp
-   Side bar for presenting associated information, or additional context
    -   Show types of data within the main panel, on the same line, so it's visually clear.
    -   Show tutorial and help information
    -   Show keyboard shortcut cheatsheet
    -   Show "palette(s)"
    -   Error messages regarding invalid data input
-   Status bar for presenting very reduced summaries, e.g.
    -   Stats of selected Array, e.g. length, type, numerical stats
-   Tool-tip/dialog popups (e.g. via some keyboard shortcut, or via mouseover) to give quick context or analyses of the selected data.  This could include "heavy" analyses such as computing a histogram or clustering of the selected numeric Array.
-   Line-interleaved overlay -- could be isomorphic to the sidebar, except that it is 1-to-1 with the primary data display.
    -   If information needs to be provided about terms very precisely, the rendered lines of data could be changed to double-spacing, and each now-empty space could be populated with information about the term directly above it, using some muted color, so that it's clear that it's not the primary data.  Examples:
        -   Type of a term
        -   Some other function applied to a term
        -   The scope that an identifier resolves to (e.g. in programming, the namespace that a function is defined in)
        -   Comments
        -   The author of the most recent edit to this particular term
-   Hierarchical selection.  Structured data is basically an inductive construction and maps to a hierarchy (or at least a directed graph).  Along with the standard notion of the location of the cursor, there should be a notion of the "level" of the cursor, which indicates what level of the hierarchy the cursor is navigating and selecting.  The level of the cursor should be indicated visually, where the background of the whole selected element is highlighted.  The cursor would move around in the standard ways using up/down/left/right arrows, and the level of the cursor would be modified using some keyboard combo, such as ctrl+up, ctrl+down.
-   "Palette" of frequently used elements
    -   In programming, a user draws upon identifiers from libraries and variables, and the distribution is probably Zipfian in some sense.  A "symbol palette" could track the most frequently used symbols, and allow them to be entered very quickly.
    -   It's important to have quick access to a multitide of views while browsing and editing complex data (e.g. web browsing or programming in an IDE), so having access to a "view palette" would be good.  This could formally track your browsing history within the documents and allow you to go back and forth as in a web browser, but maybe better yet, it would present a navigation hierarchy which you could use to go back to a previous view.  This navigation hierarchy would itself be sept data, and therefore subject to analysis (e.g. by timestamp).

### Other Design Criteria

-   It should be possible to extract sept data from a view, so that literally anything you interact with in `sept-tui` is first class data (at least on read-only basis), and could be copied out and used in a "new document".  For example, the user has selected a collection of numerical data and analyses it, producing a histogram.  That histogram should be selectable and copyable, so that it could be pasted into its own document and used.
-   Search functions
    -   Basic search on `NonParametricTerm`s (i.e. terminal values).  This would be the closest thing to text based search.
    -   Patterns.  Using the sept type system, a term like

            BinOp(BinOp(FreeVar(A), Mul, FreeVar(B)), Add, FreeVar(C))

        could be used to search the whole sept document and find all matching terms.
-   Term factoring
    -   Select at least two terms, and hit "factor".  This produces:
        -   A "base template" which is common to all the terms, and will have `FreeVar` terms indicating the placeholders.
        -   A sequence of "template parameters" which, for each of the formerly selected terms, shows the symbolic replacement to recover the original term.
        -   The user could edit the base template and/or the template parameters
        -   The user can then re-combine the factorizations which applies the changes to the original terms, making for surgical, efficient, and structurally-aware editing.
-   Interesting possibility: Factorization clustering
    -   Select a bunch of terms (or the whole document or set of documents) and perform an analysis which finds clusters of similar terms based on if their factorization is simple or complex.  This could be used to assist the code factoring process.
    -   This depends on having a notion of factorization complexity.
-   In creation of heavily-validated data, the user interface should be really forgiving and helpful in order to make it as easy as possible to arrive at a valid state
    -   Allow the document to deviate from the desired type, but give an indicator (perhaps in the side bar) of in what way the document has deviated from valid content)
    -   For invalid edits, maybe keep them as "provisional" and "separate" (maybe in the side bar, or in some parallel overlay), kind of like individual works in progress, and they don't modify the existing document.  Only once they're valid can they be applied.

### Regarding `sept` Data Model

-   `sept` has a data model which includes a runtime portion which tracks various terms, types, and related functions.  It also has a global symbol table (and optional local symbol tables) in which named data can be defined, and then referred to elsewhere.  There should be a section of the UI dedicated to the runtime's contents and status.

### MVP Implementation Plan

-   Phase 1: A read-only viewer
    -   Display data in a tree format
        -   Expand/collapse tree nodes
        -   Toggle compact (in-line) vs verbose
    -   Colorized content
-   Phase 2: Cursor navigation and selection
    -   Multi-"level" cursor navigation
    -   Selection
        -   Select none
        -   Grow selection "left"
        -   Grow selection "right"
        -   Select all within this level (e.g. select all terms inside of a tuple)
        -   Select term of higher level (e.g. select the tuple that the cursor is inside)
        -   Save a history of recent selection modifications so that you can revert to the previous state of selection
    -   Basic mouse selection
        -   While selecting stuff with the mouse, could Ctrl+mousewheel be used to increase/decrease the level of the cursor?  The mouse itself would have its own cursor separate from the keyboard cursor.  Clicking would reset the keyboard cursor to the mouse cursor's location and level.
        -   Ctrl+click to toggle selection of a term
        -   Shift+click to select a range
-   Phase 3: Basic editing capabilities
    -   Creation of typed terms via keyboard shortcuts
    -   Copy, paste
    -   Editing existing values -- start with direct text editing which modifies the ASCII rendering of that data, and requires parsing the ASCII successfully (and meeting other value constraints) to cause the edit to go through.
-   Phase 4: Oriented toward creating programming-language-like data
    -   Define a super simple dynamic language and interpreter (see C++ sept implementation for a reference).  This will involve defining a bunch of sept types which represent each of the programming language constructs.
    -   Attempt to use `sept-tui` to create and edit these programs
-   Phase 5: Oriented toward creating paragraphical content, like web articles
    -   Rendering of paragraphs as in-line content
    -   Hyperlinks
    -   Images (display in side bar or as tooltip/pop-up using the image display like in https://github.com/lemunozm/termchat)
    -   In-line images?  This might be too hard for now, since it requires nontrivial layout.
    -   Links to in-document references (like fragments), so a table of contents or index is possible.
    -   Associated data (e.g. hashtags) to automatically build an index.  Associated data can show up in the sidebar.
    -   Tabular data (potentially with more than two columns; the view would be configurable)
-   Phase X: Search and filter
    -   Use sept patterns to perform search/filter.
    -   What's the difference between search and filter?  Maybe search produces a separate document of results, whereas filter modifies the view to only show (or highlight) the filtered items.
-   Phase X: Factorization
    -   Implement factorization as described above.
    -   Implement multi-selection
    -   Success criteria are:
        -   Being able to quickly select a collection of terms
        -   Keyboard shortcut to factorize them.  This should bring up a view of the base template and the template parameters.
        -   Edit the base template and template parameters
        -

### Implementation Notes

-   Recursive drawing of elements.
    -   For now just do plain, indented tree mode, no in-lining.
    -   Cursor's position is an element in the hierarchy, and thus could be a reference to that element.
    -   There could also be a hierarchical coordinate representing its position.  This would be something like a path or a sept query string.  In fact, the notion of a coordinate is needed to refer Two passes -- one to figure out how big everything is.
-   There is a general need for a kind of coordinate for an element in a sept data structure.  Examples:

        [3].0.name  // take 3rd element of array, take 0th tuple element, take `name` attribute.
        .x          // take attribute `x`
        .x.real     // take attribute `x`, take attribute `real`

    Getting into selections of multiple coordinates:

        [3].0.name[10..20] // take 3rd element of array, take 0th tuple element, take `name` attribute, take slice `10..20`.
        [{3, 8, 10..12}] // take elements 3, 8, and slice `10..20`


## Low-Level To-dos

-   Done: Maybe split things up into a static types module `st` and dynamic types module `dy`, since there are essentially
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

-   In terms of construction/destructuring, printing/parsing and de/serialization, many or most terms should be decomposable into concrete terms such as `ArrayTerm` or `TupleTerm`, and vice versa with construction.  Thus defining those operations (construction, destructuring) would then allow printing/parsing and de/serialization to be implemented easily using the printing/parsing and de/serialization of the concrete terms.
    -   Define traits for:
        -   Restructure (aka construct; i.e. construct a term of a given type from a concrete type such as a TupleTerm).
        -   Destructure (i.e. produce a concrete term from a term of a given type).
        -   Render (to text) -- this could/would be derivable from an implementation of Destructure
        -   Parse (from text) -- this could/would be derivable from an implementation of Restructure
        -   Serializable (from binary data) -- this could/would be derivable from an implementation of Destructure
        -   Deserialize (to binary data) -- this could/would be derivable from an implementation of Restructure
    -   Notes on Deconstruct
        -   There are two interesting kinds of deconstruction
            -   A semantic kind, which performs a single level of deconstruction, in which

                    TupleTerm(1u32, 4.5f64, true, TupleTerm("abc"))

                turns into

                    ParametricDeconstruction { constructor: Tuple, parameters: (1u32, 4.5f64, true, TupleTerm("abc")) }

            -   A concrete kind, which performs deconstruction recursively, which renders

                    ParametricDeconstruction {
                        constructor: NonParametricDeconstruction(Tuple),
                        parameters: (
                            ParametricDeconstruction {
                                constructor: NonParametricDeconstruction(Uint32),
                                parameters: (NonParametricDeconstruction(1u32)),
                            },
                            ParametricDeconstruction {
                                constructor: NonParametricDeconstruction(Float64),
                                parameters: (NonParametricDeconstruction(4.5f64)),
                            },
                            ParametricDeconstruction {
                                constructor: NonParametricDeconstruction(Bool),
                                parameters: (NonParametricDeconstruction(true)),
                            },
                            ParametricDeconstruction {
                                constructor: NonParametricDeconstruction(Tuple),
                                parameters: (
                                    ParametricDeconstruction {
                                        constructor: NonParametricDeconstruction(Utf8String),
                                        parameters: (NonParametricDeconstruction("abc")),
                                    },
                                ),
                            },
                        ),
                    }

        -   Also interesting is adaptive deconstruction which would be used in destructuring contexts where a particular destructuring pattern is desired, one that doesn't necessarily correspond to a uniform number of layers (i.e. some of the branches of the AST are deconstructed more deeply).
    -   Notes on `Detextify`, i.e. the canonical parsing of a term's textifaction.
        -   Need to implement a basic pushdown automaton, meaning there's a state machine which operates using a stack of states.
        -   Open-delimiters push the stack, close-delimiters pop the stack.
        -   Commas delimit separate elements in a tuple of parameters.
        -   String/char quotes don't have separate open/close versions (at least not in ASCII), so those require slightly different.
        -   Primitives:
            -   String literals
            -   Char literals
            -   Identifiers (maybe relax this and make it less restrictive than just C-style identifiers)
            -   Integer literals
            -   Decimal literals (numeric literals having a decimal point)
            -   Numeric literals with a radix suffix

-   Figure out how to implement proc_macros for deriving traits on generic types.  In particular, will have to parse out not just a `syn::Ident` but whatever the right type is for the relevant generic syntax.
-   Could maybe use https://docs.rs/tuple_list/latest/tuple_list/ to implement a `st::TupleTerm`.
-   The notion of type is related to the `Constructor` trait.  In particular, a type is a term that has a notion of inhabitation by other terms (even if it may be inhabited by no terms, such as `EmptyType`).  This is really a declaration that something exists.  But `Constructor` is more specific, because it actually defines how a term can be used to construct another term, and defines what the parameterization is.
    -   Question: Should a `Constructor` always construct a term that inhabits it?  That's a good starting assumption.  Algebraic data types, e.g. `pub enum Thing { A(String), Nothing}` can be handled by making `A` and `Nothing` non-parametric terms that are passed as parameters to the constructor `Thing`.  Though in this case, `Thing(A, ...)` could also be considered to be a constructor.
    -   A `NonParametricTerm` is a kind of constructor, in that it is its own, zero-parameter, constructor.  Though probably shouldn't be implemented that way, since it also forms the base case of the inductive construction of terms.  Maybe simply allow it, since there's probably no harm in doing so, and it might simplify other logic.
-   Implement destructuring of sept `dy::Value` into Rust types, especially tuples and structs.
-   Rename `type_` to `r#type`.
-   Maybe use the crate `funty` and its "fundamental" traits to clean up some of the POD types.
-   Come up with a scheme for identifying local symbol tables so that they can be unambiguously referred to in a `Deconstruction`, and therefore `Deconstruct` and `Construct` can be implemented for `LocalSymRefTerm`.
-   Rename Ascii* to ASCII* and Utf* to UTF*, and generally make acronyms uppercase in names.
