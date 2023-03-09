# `sept-gui` Design Notes

## 2023.03.02

Initial implementation of sept-gui.
-   First goal is to create initial viewer components for existing sept data types, so that any sept data (which is inductively defined from those types) can be rendered.
-   Second goal is to add a notion of a cursor.  The cursor should have a canonical path which ideally is the same as the query path for the sept data, which is displayed in the view footer.  Cursor should be rendered over the whole element that it addresses.  There should be keyboard control of the cursor.
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
