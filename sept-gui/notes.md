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
        -   Render as `@"symbol id"` (`@` indicates "global" -- this is just made-up syntax; not sure what's best).
    -   LocalSymRefTerm
        -   Render as `$"symbol id"` (`$` indicates "local" -- made-up syntax).

Consider using "text view" concept from tui-experiment in which the specific mapping from value to text is formal, and the character within the text rendering is addressed for view and edit purposes.
