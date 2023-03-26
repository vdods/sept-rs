# `sept-gui` Design Notes

## 2023.03.02

Initial implementation of sept-gui.
-   First goal is to create initial viewer components for existing sept data types, so that any sept data (which is inductively defined from those types) can be rendered.
-   Second goal is to add a notion of a cursor.  The cursor should have a canonical path which ideally is the same as the query path for the sept data, which is displayed in the view footer.  Cursor should be rendered over the whole element that it addresses.  There should be fine-grained keyboard control of the cursor, including structure-aware navigation and selection.
-   Second goal is to add editing capabilities to those components, so it's possible to create/edit/delete sept data.  Need to handle invalid intermediate states of input (e.g. when you type `1.04e` on the way to typing `1.04e3` for a float value).

Notes on viewer components for various types
-   Non-parametric terms; require very little to render, just the term name
    -   Void, VoidType
    -   Bool, BoolType
    -   True, TrueType
    -   False, FalseType
    -   EmptyType
    -   Sint#, Sint#Type
    -   Uint#, Uint#Type
    -   Float#, Float#Type
    -   Utf8String, Utf8StringType
    -   Array, ArrayType,
    -   Struct, StructType
    -   Tuple, TupleType,
    -   GlobalSymRef, GlobalSymRefType
    -   LocalSymRef, LocalSymRefType
-   Parametric terms; each has content that must be rendered.
    -   BoolTerm = bool (Rust type)
        -   Render as `true` or `false`.
        -   Render options -- `true`/`false` vs `1`/`0`
    -   Sint#Term = i# (Rust type)
        -   Render as decimal value.
    -   Uint#Term = u# (Rust type)
        -   Render as decimal value.
    -   Float#Term = f# (Rust type)
        -   Render as scientific notation with 17 digits of precision.
    -   Utf8StringTerm = String (Rust type)
        -   Render as string literal.
        -   Render options
            -   As string literal with escape sequences -- could be considered "in-line".
            -   As a rendered string where some escape sequences are actually rendered (mainly tabs, newlines).
    -   ArrayTerm
        -   Render as in-line sequence of values (number of elements shown?), bracketed by `[]`.
        -   Render options
            -   In-line (no wrapping)
            -   Compact (newline only as needed to keep content in the view; this may be difficult to implement
                because it makes the ArrayTerm render height depend on the view size)
            -   Expanded (newline per element)
    -   StructTerm
        -   Render as `Struct { x1: T1, x2: T2 }`.
        -   Render options
            -   In-line
            -   Compact
            -   Expanded
    -   TupleTerm
        -   Render as in-line sequence of values, bracketed by `()`.
        -   Render options same as ArrayTerm
    -   GlobalSymRefTerm
        -   Render as `@"::symbol_id"` (`@` indicates "global" -- this is just made-up syntax; not sure what's best).
    -   LocalSymRefTerm
        -   Render as `$"tablename::symbol_id"` (`$` indicates "local" -- made-up syntax).

Consider using "text view" concept from tui-experiment in which the specific mapping from value to text is formal, and the character within the text rendering is addressed for view and edit purposes.

## 2023.03.08

-   View of relevant data types is done.
-   Still haven't implemented a map type in sept yet, and that should probably be implemented before doing any more on the gui, since it has ramifications for proper design of editing.
-   Idea for making editing more human-friendly
    -   Humans are used to editing documents in a relatively free-form way, where the program doesn't require the document to be well-formed at all times.
    -   Data types often have constraints, and therefore corresponding ways for data to be ill-formed.
    -   Editing the data in a conventional way will often involve temporarily violating data type constraints as an intermediate step.
    -   Should allow this temporary violation of constraints so as not to beat the human up with too much strictness.
        -   One way to do that is to have "overlay" values which are essentially deferred, proposed changes to a particular piece of data.  They would be less-structured and they are stored separately, but are rendered over the value they propose to change.  This would be similar to some editing behavior for spreadsheets; you can edit a cell and press enter to commit the change, or escape to revert.  Should visually indicate that there's an overlay value, and there should be a way to see what the existing value is that would be replaced.  This might be helpful also for the tracking of actions for undo/redo and producing diffs of the data edits.
-   Priorities
    -   Implement OrderedMap in sept crate
    -   Finish and merge work-in-progress branch regarding serialization.
    -   Implement element addressing in sept crate.
    -   Implement cursor

### Wonky Ideas on Editing

Regarding address/coordinates of a piece of sept data, there could be query methods attached to data types which are usable via addressing, and in limited ways would facilitate editing, if it gives char-level indexing into a string repr of the data.  These addresses would be used in the "overlay" editing, since a proposed edit needs to know what repr of the data is being used.

-   Ints.  Assume `x` addresses an integer.

        x.type -> produces the int type
        x.num_bits -> produces the number of bits used by this datatype
        x.sign -> produces the sign (-1, 1, or 0)
        x.base.10.num_digits -> produces number of nonzero digits in the base 10 rep (doesn't include the sign); the value 0 requires 1 digit by definition.
        x.base.10.0 -> produces the 1s digit (i.e. the 0th digit)
        x.base.10.1 -> produces the 10s digit (i.e. the 1th digit); digits converge to 0.
        x.le_bytes -> produces little-endian byte array repr of int
        x.be_bytes -> produces big-endian byte array repr of int

    The above all produce int-valued results.  As for editing textual representations of an int,

        x.base.10.str -> produces the string repr in the given base
        x.base.10.str.len -> produces the len of that string repr
        x.base.10.str.0 -> produces the first char in the string repr, e.g. `4` in `452` or `-` in `-300`
        x.base.10.str.1 -> produces the second char in the string repr, e.g. `5` in `452` or `3` in `-300`

-   Strings.  Assume `s` addresses a string.

        s -> produces the string type
        s.len -> length of string
        s.0 -> 0th char of string
        s.1 -> 1th char of string
        s.is_empty -> boolean value of `s.len == 0`.
        s.bytes -> byte array repr of string

-   Floating point numbers.  Assume `f` addresses a float.  This one requires some thought, because representing floats as text is complex, involving scientific notation, number of digits to show, etc.

        f.type -> produces the float type
        f.num_bits -> produces the number of bits used by this datatype
        f.sign -> 1, 0, or -1
        f.base.10.sci.5 -> (this is not sufficient; needs more params) produces the base-10, scientific notation repr of this float using 5 digits of precision.  Probably need more params (e.g. show sign or not) using a different notation convention (e.g. `f.base.10.sci(5, show_sign)`)
        f.le_bytes -> little endian byte repr
        f.be_bytes -> big endian byte repr

-   ArrayTerm.  Assume `a` addresses an ArrayTerm

        a.type -> produces the array type
        a.len -> length of array
        a.is_empty -> boolean value of `len == 0`
        a.element_type -> declared element type (not currently implemented in sept-rs)
        a.0 -> 0th element of array
        a.1 -> 1th element of array

-   TupleTerm.  Assume `t` addresses a TupleTerm

        t.type -> produces the tuple of types of elements of t
        t.len -> length of tuple
        t.is_empty -> boolean value of `len == 0`
        t.0 -> 0th element of array

-   StructTerm.  Assume `s` addresses a StructTerm

        s.type -> produces Struct
        s.len -> number of attributes
        s.is_empty -> boolean value of `len == 0`
        s.0 -> produces the 0th Attribute
        s.as_tuple -> produces the tuple of ordered attribute types

-   StructTermTerm.  Assume `z` addresses a StructTermTerm (an instance of StructTerm)

        z.type -> produces the direct type of the StructTermTerm (e.g. a StructTerm or a GlobalSymRefTerm)
        z.len -> number of attributes
        z.is_empty -> boolean value of `len == 0`
        z.0 -> produces the 0th attribute value
        z.tuple_type -> produces the tuple of ordered attribute types

## 2023.03.19

### Implementation notes for data addressing

The address should simply be a tuple of values which are basically a postfix method-style syntax for a query on a piece of data.  Different data types will have different kinds of possible queries.  The query on a particular term will simply accept a value and determine what to do based on the value.  For now, define a few

Simple examples:
-   ArrayTerm could accept any kind of integer and use that to produce the element with that index.  If the index is negative, that could mean to index from the end, like in Python.
-   TupleTerm would be similar.
-   OrderedMapTerm could accept any term and if it matches a key

Should queries actually be done through a first class data model, where various types have various methods?  Rough idea:
-   All terms should have:
    -   Inhabits(T) -- returns true if this term inhabits T, false otherwise.  Maybe this is only semi-decidable,
        so perhaps it shouldn't be a bool, but rather a tri-conditional with true, false, and unknown.
    -   Require(P) -- causes the query to fail if the given predicate is false; otherwise simply produces the value itself.  An example of P would be "has concrete type == SomeType".  This generality (using predicates) is probably way too far ahead, it should start much simpler.
    -   And other term data methods
-   Bool has:
    -   Not -- returns the negation of this boolean value.  This would look hilarious too, since it might be common for a query to end with `Not` a la Wayne's World.
-   Utf8String has:
    -   Len -- returns the length of this string
    -   Char(N) -- returns the Nth char
    -   Lines -- returns a view into the newline-delimited lines of this string
-   *int# has:
    -   Sign -- returns 1, 0, or -1 as an int.
    -   SignStr -- returns "+", "", or "-".
    -   Base(N) -- view (?) into the base-N representation of this integer
-   Base(N) view into integer (need to formalize this concept -- it could apply to float as well)
    -   Value -- returns the original numeric value
    -   Sign -- returns 1, 0, or -1 as an int.
    -   SignStr -- returns "+", "", or "-".
    -   Str -- returns a base-N string view into this value, including the sign.
    -   Digits -- view into array of base-N digits (this would need to be its own term with its own methods so modification is possible; e.g. insertion/modification/erasure of a digit); this does not include the sign.
-   TupleTerm has:
    -   Len
    -   Element(N)
-   ArrayTerm has:
    -   Len -- returns length of array
    -   Element(N) -- returns element N, where N is any int.
    -   ElementType -- returns the element type of this array; for now can just return `Term` which is the most generic type.
-   StructTerm has:
    -   Len -- returns number of fields in StructTerm
    -   NthField(N) -- returns (field_name, field_type) for the Nth field.
    -   NthFieldName(N) -- returns name of field N, where N is any int -- maybe this should be subordinate to a StructField "view"
    -   NthFieldType(N) -- returns field type N, where N is any int -- maybe this should be subordinate to a StructField "view"
    -   Field(S) -- returns (field_name, field_type) for the field with name S, where S is a string.
    -   FieldType(S) -- returns field type with name S, where S is a string -- maybe this should be subordinate to a StructField "view"
    -   FieldIndex(S) -- returns the index of the field with name S.
-   StructTermTerm has:
    -   Len -- returns number of fields (which is the same as its direct type StructTerm)
    -   NthFieldVal(N) -- returns value of field N, where N is any int.
    -   FieldVal(S) -- returns field with name S, where S is a string.
-   OrderedMapTerm has:
    -   Len
    -   NthKey(N) -- returns the Nth key.
    -   NthVal(N) -- returns the Nth value.
    -   NthKeyVal(N) -- returns the Nth (key, value) pair.
    -   Val(K) -- returns the value for the given key K.
    -   KeyVal(K) -- returns the (key, value) pair for the given key K.

This suggests a set of new terms that would be built into the sept data model, including:
-   Inhabits
-   Not
-   Len
-   Char
-   Lines
-   Sign
-   SignStr
-   Base
-   Element
-   ElementType
-   NthField
-   NthFieldName
-   NthFieldType
-   Field
-   FieldType
-   FieldIndex
-   NthFieldVal
-   FieldVal
-   NthKey
-   NthVal
-   NthKeyVal
-   Val
-   KeyVal

These would be used in what would effectively be ASTs that form the various queries, which would be addresses of a sort.  Notably, this allows addressing derivative data such as different representations of the data (e.g. base-N repr of an int).  Each of these standard sept data model terms should eventually be formally defined relative to formally defined interfaces/traits.

What would an address look like?  Thinking of it as a sequence of method calls, it would correspond to syntax like

    .Val("hippo count").Base(5).Str.Char(3)

which would index the 3rd char of the base-5 string representation of the value with key "hippo count".  The `.Base(5).Str.Char(3)` part could be considered to be details of the view and perhaps hidden by default in "layperson mode".  Or perhaps should `.` denote invoking of constructors as well?  This would make it

    .Val."hippo count".Base.5.Str.Char.3

and it would be possible to address the constructor of a particular `BaseTerm` as `.Value."hippo count".Base`, where the constructor constructs a BaseTerm with a particular value.  This conflation of query sequence with constructor may be too wonky and not actually called for.

The query term itself would be more like

    Query(Val("hippo count"), Base(5u32), Str, Char(3u32))

and what's shown in sept-gui might be some even shorter syntax, like

    ."hippo count".base(5).str.3

where certain data method names are elided in order to more closely resemble standard syntax (in this case, `Val` and `Char`).  It would be very cool to have Rust-inspired syntax to represent multi-value selections, like

    .[."hippo count", ."ostrich count".base(5).str]

Would produce an ArrayTerm with two elements; the value corresponding to key "hippo count", and then the base-5 string repr of the value for "ostrich count".  Note the use of `.` within the `[]` brackets; it implicitly refers to the query's principal argument (i.e. the thing being queried at that level).  Use of `()` brackets would produce a TupleTerm.  To produce OrderedMapTerm,

    .{"blah" => ."hippo count", 456 => "ostrich count".base(5).str}

noting that `"blah"` and `456` are literal values, not query parameters.  And then assuming that OrderedSet exists, it could use `{}` brackets also (though it would be nice if there were another kind of bracket for OrderedMap, since `{}` is used for sets in standard mathematical notation).  Nested multi-query:

    .{"blah" => ."hippo count", 456 => "ostrich count".base(5).str.(.len, .0)}

where what 456 maps to is a TupleTerm with 2 elements:
-   the length of the base-5 string rep of the value for "ostrich count", and
-   the 0th character of that string.

This query would actually be represented by

    OrderedMap("blah" => Query())


AST

    self.val("x").base(5).str.char(3)

would be

    MethodCall
        MethodCall
            MethodCall
                MethodCall
                    self
                    Construction
                        val
                        ("x")
                Construction
                    base
                    (5)
            str
        Construction
            char
            (3)

But as a sequence of instructions:

    r1 := Evaluate(MethodCall(self, val("x")))
    r2 := Evaluate(MethodCall(r1, base(5)))
    r3 := Evaluate(MethodCall(r2, str))
    r4 := Evaluate(MethodCall(r3, char(3)))
    render r4 as the result of the query

Another way to write:

    render EvaluateQuery(EvaluateQuerySingle(self, val("x")), Query(base(5), str, char(3)))

How about an aggregate query like the following?

    self.(.val("x"), .val("y"))

    render TupleEvaluateQuery(self, Tuple(Query(val("x")), Query(val("y")))) as the result of the query

and TupleEvaluateQuery produces a Tuple where it performs each query on the argument, in this case `self`, rendering a tuple of the respective results.

OrderedMap query:

    self.{.key_index("x") => .val("x"), .key_index("y") => .val("y"), 123 => 456}

which would do

    render OrderedMapEvaluateQuery(self, OrderedMap(Query(key_index("x")) => Query(val("x")), Query(key_index("y")) => Query(val("y")), 123 => 456))

OrderedMapEvaluateQuery would recursively evaluate the contents of the ordered map (each key and each value).

## 2023.03.25

Notes regarding query as it applies to cursor/selection and as it applies to modifying data, keeping in mind the need for an undo/redo buffer:
-   For cursor/selection, as the View traverses through the pieces of the overall data, it should track the "address" of the cursor (as a query string).  This update should be done by the impl of View for each type.  Then in terms of showing a visual highlight for the cursor and/or selected values, it would simply compare the current item address with the cursor and/or selection set.  If the current item address is "under" the cursor or is "under" any element of the selection set, then it should highlight that item.  This should be done by background highlighting.  Ideally though an aggregate data type that's highlighted should have a nicely shaped background.  For example, if it's inlined, then the highlighted background should simply be the chars that aggregate occupies, thereby matching its shape as text.  However, if the aggregate is expanded, then all the lines of that aggregate, including those of the brackets, should be highlighted in a rectangle whose left side is the depth of the brackets, and whose right side ideally is either the right side of the view, or includes the rightmost content.
-   However, there's a bit more to it than just the address of the item, since the specific representation of the item in the view is a parametric thing in itself (e.g. number bases, scientific notation vs decimal expansion, etc).  There are a few options here:
    -   Simplest would be to make parameters for representation part of the ViewCtx (or part of the View if the View had some state), since then each item would just have its address and there would be no need to append representational query tokens.  However, the representational query tokens would still need to be specified when making edits to the data.
    -   More complex would be to have each item gain default representation query tokens based on some default that persists in the ViewCtx, but if the repr were changed for an item, the ViewCtx would remember the change, so it would be possible to modify, on an item-per-item basis, how each item is represented.  This could even extend to changing the medium of an item; e.g. an array with floats might simply be represented as a graph, a histogram, or a statistical summary (mean, stddev, etc).
-   For modification, keeping in mind the need for undo/redo, each modification has to be recorded formally, and the whole address with the representational query tokens should be included in that, because that's what translates user input to modifications in the data.  Each formal modification must consist of a forward and backward modification, so that they're reversable.  These modifications will eventually be part of a formal algebra (algebroid?) of data which formally define the "diff" logic for sept data.

Implementation notes
-   Get addressing working.  An address is simply a tuple of addressing values which uniquely identify a piece of data in an aggregate.
-   Get cursor highlighting working.  A basic cursor is simply an address which causes that item to be highlighted.
-   Implement default representation query tokens (come up with a better name than that) in the ViewCtx, so that the representation is known.
-   Allow changing of the ViewCtx's default representation query tokens, so that the user can view data in different representations.
-   Implement modification of data via recording of formal data modification commands.  Each modification consists of:
    -   Address of data being modified
    -   Representation query tokens (these could be thought of as being appended to the address)
    -   The specific modification to that representation.  This is representation dependent.
        -   Maybe actually the specific UI events are recorded instead, and simply played through the representation?  Though this would couple the UI events with the representational logic.
