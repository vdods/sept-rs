# `sept-gui` Design Notes

## 2021.10.18

From conversation with Ellie: data creation could be done on a mobile device using a “symbol palette” and search bars and autocomplete, etc, and we were comparing it to writing it out in text, and she came up with the perfect analogy — writing it out in text is like writing it out in long hand, where it’s a useful way to learn at first, but eventually you want something more structured and fast, so you switch to short hand, which would be the optimized input method with view/model architecture for sept data.

## 2022.01.17

Ideas for mobile device UI
-   Have a kind of “palette” of commonly used input for efficient selection
-   Use a press and draw through a hierarchy of menus to quickly select an item from the menu. Some thinking needs to be done for how to make this work when the menu changes and ideally people don’t have to relearn the muscle memory movements to adapt.
-   Voice activated input
-   Ideas for mobile input: somehow keep the positions of the swipe buttons fixed, by reserving space for new ones, so that existing button positions don’t have to change when a new one is added
-   Could also maybe use a direct mapping onto lyx-like input, like ctrl/alt is pressing on a button, and then swipe through a sequence of letter mappings, which could even be the same mappings as in the tui app.
-   Might want to use a better swipe keyboard layout, like in a circle, so that the swipes are much more easily distinguishable.
-   This circular keyboard could be more generally useful since existing swipe keyboard on a qwerty layout suck.

TUI/GUI
-   LyX-like keyboard shortcuts to make creating sept data be as fast as one can think it; ctrl/alt+key and then a sequence of keys to navigate a menu hierarchy. This would be more robust to change, since one change wouldn’t screw up the other letter mappings

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
-   David DeConde suggests using the VIM keyboard shortcuts for navigation in order to have "free" avenue for adoption by an existing segment of users.
    -   https://www.maketecheasier.com/cheatsheet/vim-keyboard-shortcuts/
-   This may be useful:
    -   https://docs.rs/supercow/latest/supercow/ -- like Cow (copy on write), but allegedly better.

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
-   Addressing.  An address is simply a tuple of addressing values which uniquely identify a piece of data in an aggregate.
-   Get cursor highlighting working.  A basic cursor is simply an address which causes that item to be highlighted.
-   Implement default representation query tokens (come up with a better name than that) in the ViewCtx, so that the representation is known.
-   Allow changing of the ViewCtx's default representation query tokens, so that the user can view data in different representations.
-   Implement modification of data via recording of formal data modification commands.  Each modification consists of:
    -   Address of data being modified
    -   Representation query tokens (these could be thought of as being appended to the address)
    -   The specific modification to that representation.  This is representation dependent.
        -   Maybe actually the specific UI events are recorded instead, and simply played through the representation?  Though this would couple the UI events with the representational logic.

## 2023.03.26

Notes on cursor, keyboard events, and prep work for handling data modifications:
-   Keyboard events really need to be handled in implementations of `View`, since keyboard events are interpreted differently based on the configuration of the view (e.g. inline vs expanded).
-   Because data modifications will be handled via formal modification commands, and not in the `mut`-based immediate paradigm of `egui`, the `self` field of the methods of `View` should be made non-`mut`.  This way, immutable data can be traversed naturally (in particular, `OrderedMap` terms' keys).

## 2023.03.27

Notes on cursor, keyboard events, and prep work for handling data modifications for strings:
-   Strings are shown in one of two `LayoutMode`s:
    -   `Expanded`: as a sequence of newline-delimited lines, or
    -   `Inline`: as a single character sequence.
-   When the string is entered (via Enter key, or `"`), it should push a cursor address token which depends on the `LayoutMode`:
    -   `Expanded`: it should push a "lines view" token (for now just use the string "lines"), and then `0`, meaning that the cursor is addressing the 0th line of the string.
    -   `Inline`: it should simply push `0`, understanding that integers naturally index the chars of a string.  Though maybe for consistency there could be a "chars" token before the `0`.
-   If the cursor is in a "lines" view of a string and Enter is pressed (or `c`?), then it should push a "chars" token and `0`, so that it's addressing the 0th char of the current line.
-   It should be possible to explicitly enter the "lines" or "chars" view of a string regardless of what its `LayoutMode` is.  This can be done by pressing `l` or `c` instead of Enter or `"`.  Enter or `"` gets you whatever the default for the `LayoutMode` is.
-   When in "lines" "chars" mode (i.e. chars of lines), probably moving the cursor before the first or after the last char on that line should cause it to go to the previous/next line, so as to not violate the principle of least surprise.
-   There should also be a "bytes" view of a string.  A view into bytes should be its own thing, having its own view parameters (e.g. number base, some sort of aligned grid display, could run hex digits all together for a compact form, etc).

With container/aggregate data types, it's necessary to be able to append elements "at the end", so some placeholder UI is needed for:
-   When the container is empty and the user enters it.  There should be some visual indicator of the cursor indicating that it's ready to append items but that there are no items present.
-   When the cursor is already in the container but goes one past the last item, where the intention is to append items.  Again, there should be some visual indicator of the cursor.
-   Probably use a sentinel "end" cursor token to indicate this (in the sense of programming language ranges having the form begin..end where end is exclusive).  Might this idea naturally extend to insertion of items before the first item?

## 2023.03.31

Idea for "conventional" cursor navigation:
-   Simply do an ordered traversal of the leaf AST nodes via left/right arrow keys.  This order depends on the specific view of each type.  Up/down arrow keys should navigate to what's above or below, which is dependent on what else is layed out in the view, and can't necessarily be determined from the data alone.

Notes for improvement of cursor-related work:
-   Cursor is hard to see, especially when it's moving.  Do one of the following:
    -   Make a high contrast outline around the background highlight
    -   Make the cursor blink
    -   Make the cursor be high contrast somehow
-   The various string views (line, char, and line-char) are a bit disjointed.  UX improvements:
    -   line mode: This seems fine for now.
    -   char mode: when the line wraps, the up/down arrow keys really should go up/down visually so as not to violate the principle of least surprise.  Similarly, page up/down should go vertically when the line wraps.  Conventionally, up/down and page up/down always go vertically.  Though maybe char mode is really just a special mode that is meant to be more programmatic and less about UX.  line-char mode is generally better UX.
    -   line-char mode: This is working the best, but the cursor should wrap to prev/next line when it goes off the end of the current line to be like conventional text editors.  However, it still has the problem that when it's displaying in LayoutMode::Inline, up/down and page/up down should move vertically in the view.  Still need to figure out how to do this sort of hit detection in egui.

Still to do:
-   Views into numerical types (ints and floats).  Just use base 10 for now.

Notes for editing of data
-   It probably makes sense to formally track the cursor in the log of data modification commands.  This way, each command doesn't have to have a full copy of the address of the value being edited.
-   Data modification commands
    -   For Utf8StringTerm
        -   InsertChar -- inverse is DeleteChar
            -   insert_address
            -   char_to_insert
        -   DeleteChar -- inverse is InsertChar
            -   delete_address
            -   char_to_delete -- this makes this command invertible
        -   ReplaceChar -- inverse is itself, but with new_char and old_char reversed
            -   replace_address
            -   new_char
            -   old_char
        -   InsertSubString -- inverse is DeleteSubString
            -   insert_address
            -   string_to_insert
        -   DeleteSubString -- inverse is InsertSubString
            -   delete_address
            -   string_to_delete -- this makes this command invertible
        -   ReplaceSubString -- inverse is itself, but with new_string and old_string reversed
            -   replace_address
            -   old_string
            -   new_string

Notes on editing of data through a view, e.g. a string representation of an int
-   In the ideal case, each individual data modification to the view is a valid modification to the underlying data.
-   However, some data has constraints that aren't structurally enforced by the view representation.  For example, rendering an int as a base 10 string of digits.  In that case, modifications to the string may cause that string to not parse as an int.  There might be legitimate intermediate edits to the view that are simply part of a sequence of edits that get back to a valid data value.
    -   When a valid modification is made to a view, the modification should take effect in the underlying data immediately.
    -   When the view has invalid data, it should be kept in ViewCtx as an edit overlay, and will simply wait until it's changed to be a valid modification before applying it to the underlying data.  This edit overlay will create a copy of the last valid value of the view (e.g. the base 10 string rendering of an int), along with the address that the edit overlay applies to, and allow edits to that string until it becomes a valid modification, at which point it will apply the change and delete the edit overlay.  This will be needed in particular for modifying OrderedMapTerm or StructTerm where there are higher-order uniqueness constraints.
    -   There will eventually be higher order constraints that involve multiple data elements, and some sort of edit overlay situation needs to be figured out for those.

## 2023.04.22

Notes for simplest possible implementation of query/edit.
-   Goal is to use the view/model pattern as purely as possible.
-   Use Queryable trait with
    -   query (immutable) which takes an iterator of address tokens and produces a query view object (this has to be generic, so it should be something like `Arc<RwLock<dyn QueryView>>`, where `QueryView` is a trait).
    -   query_mut (mutable), analogous, produces a mutable query view.
    -   Maybe actually there's only one query method, and the (im)mutability of the thing is controlled by the runtime checks.
        This would reduce code duplication, though may have other drawbacks.
-   The query view should be able to
    -   Indicate the type of the value that would be returned
    -   Return (a reference to) the value
    -   In the case of mutable query view, it should be able to apply an edit value.
    -   A query view type necessarily has an associated lifetime, which is the lifetime of the original queried value.
    -   Perhaps in the future, it should be able to produce a list of the valid next query address tokens, as well as the valid edit types.
-   Example:
    -   Let the data be

            [
                "hippo\nOSTRICH",
                true,
                { 123 => 456 },
                { { "a" => "b", "c" => "d" } => 9000, { "x" => "y", "z" => "w" } => 9009 },
                Struct { "name": Utf8String, "age": Uint8 },
                Struct { "name": Utf8String, "age": Uint8 } { "Ftanley", 100 },
            ]

    -   Query `()` should return a reference (which is a query view) to the whole array.
    -   Query `(0)` should return a reference (which is a query view) to the string.
    -   Query `(0, "line", 0)` should return a `Utf8StringLine` query view which itself has a reference to the string `"hippo\nOSTRICH"` and which line is being viewed.  That query view object should indicate the type of the value is Utf8String and should be able to produce the value `"hippo\n"`.
        -   An edit to this query view should edit the substring `"hippo\n"` within the larger string.
    -   Query `(0, "line", 1, "char", 0)` should return a `Utf8StringLineChar` query view which itself has a reference to the string `"hippo\nOSTRICH"` and which line and char is being viewed.  That query view object should indicate the type of the value is UnicodeChar and should be able to produce the value `'O'`.
    -   Query `(0, "line", "count")` (this is made-up for now) should return a query view object which returns the number of lines in the string.  It should not be editable.
        -   An edit to this query view should edit the character `'O'` within the larger string.
    -   Query `(1)` should return a reference to the boolean.
    -   Query `(2)` should return a reference to the OrderedMapTerm `{ 123 => 456 }`.
    -   Query `(2, 'v', 123)` should return a reference to `456` (the value associated with the key `123`).
        -   An edit to this query view should edit the value `456` within the OrderedMapTerm, which can be done "directly" (in Rust and C++).
    -   Query `(2, 'k', 123)` should return an `OrderedMapKeyView` to the key `123` within the OrderedMapTerm
        -   An edit to this query view would need to handle checking if the new key value is already present, and if not, removing the pre-edit key-value pair, modifying the key, and re-adding the key-value pair with the new key.
    -   Query `(2, 'kv', 123)` should return an `OrderedMapKeyValueView` to the key-value pair `123 => 456` within the OrderedMapTerm.
    -   Query `(2, 'kv', 123, 'k')` should return an `OrderedMapKeyView` query view to the key within the key-value pair `123 => 456` within the OrderedMapTerm.
    -   Query `(2, 'kv', 123, 'v')` should return an `OrderedMapValueView` query view to to the value within the key-value pair `123 => 456` within the OrderedMapTerm.
    -   Query `(3, 'k', { "a" => "b", "c" => "d"}, 'k', "a")` should essentially create a stack of query view objects whose outermost one refers to the `"a"` key.
        -   An edit to this query view should first apply the edit to the outermost one (which is a view into the `"a"` key), which causes the edit to propagate to the next inner one (which is a view into the `{ "a" => "b", "c" => "d"}` key), which carries through with the edit.
    -   Future possibilities:
        -   Query `(0, "line")` could return a query view which is the sequence of lines in that Utf8StringTerm.
        -   Query `(2, 'k')` could return a query view which is the ordered set of keys of that OrderedMapTerm.
        -   Etc.
        -   Advanced queries could produce things like
            -   Stats on arrays of numbers
            -   Histograms
            -   A Kernel Density Estimation of a given array of values
            -   Various other ML models of arrays of values
            -   A categorization of elements from an aggregate type into a map which maps categories to sets of the elements falling into the respective categories.
        -   It will be necessary that these queries are extensible (likely using the Runtime registration pattern) so that the overall data model is extensible.

## 2023.05.07

Design notes for better event-handling
-   I've recently gotten formal query and edit working, so that data within the aggregate "root value" can be individually addressed and edited, and this makes for a nice basis for implementing an edit UI.
-   Each of the views should implement
    -   Handling of events
    -   Rendering
        -   Inline
        -   Expanded
-   The current handling of events is rather monolithic, where events meant for a "child" view are handled by a parent view, and this is hard to manage.  It would be better if each view handled its own events directly.
-   There should be cursor movement methods in view_ctx to make it easier for views to modify the cursor, since the cursor needs to be modified while handling editing events (e.g. interleaving updating the cursor with the handling of keypress events (which each cause a char insertion)).
-   The view objects in sept should be used, but traits in sept-gui should be used to implement event handling and rendering, since they use egui-specific types.  It's possible that later sept will provide an abstraction of this which can easily plug into other UI frameworks, including TUI frameworks.

## 2023.05.09

Design notes for event handling in UI
-   Event handling should happen before rendering, since rendering should reflect the current state of the program, and the events for a given frame have been captured before that moment, so they determine the current state of the program.  Although this sort of contradicts the immediate mode pattern, since rendering and events are handled at the same time there.  To be more specific, the view objects should run all their event handling before the UI is run, potentially leaving some events unhandled which would be handled during the UI run.
-   The view objects should handle events.  Use a query to retrieve the cursor-addressed view object, and have it handle events.  Unhandled events should fall through to its parent, and then its parent, etc.  Thus the handle_event method must either
    -   traverse to the appropriate sub-view for each address token, or
    -   use a generic query to perform one address token's worth of querying at a time, but still be able to call handle_event on the unwind
-   Another general approach to query would be to build a stack of view objects, instead of consuming the view object to progress the query.  This would allow the handle_event fallthrough described above.
-   Because handling an event can alter the cursor address, each event must be handled one at a time.  I.e. which view is handling the event must be re-determined with a fresh query each time (it should be possible to use a "dirty" flag to indicate if the cursor address has changed and therefore needs to trigger a new query).
-   Having a generic "do one address token's worth of query" is probably not going to work since it would need to return a `Box<dyn SomeTrait>` but that trait can't necessarily capture the functionality of everything a view object is meant to do (e.g. not just value query/edit, but also handle_event and run_ui).  Could maybe use the downcast-rs crate to have it return a particular base.
-   If each view object contained its parent view, then each view would effectively have the view stack available to call.  Or each view contains what it contains now (minimum necessary to perform queries) and then the view object stack is explicitly tracked, and each element of the stack has type `Arc<RwLock<dyn T>>` for an appropriate trait `T`.

## 2023.05.19

Notes on ongoing work on query and edit
-   After a lot of experimentation, whittled things down to two traits for mutable query and edit:
    -   `ApplyEditTrait` -- applies an edit to self.
    -   `SingleQueryMut` -- performs one address token's worth of mutable query.
-   `QueryMutAndApplyEditTrait` is automatically implemented for any type that implements both of those traits (with some other minor trait bounds).
-   As for immutable query, a similar effort to decompose it into simpler components is underway.  So far, there is the trait:
    -   `SingleQuery` -- performs one address token's worth of immutable query.
-   An immutable analog to `QueryMutAndApplyEditTrait` could be created which runs a full query on an iterator of address tokens, then returns the queried value.  This could be automatically implemented for types that implement
    -   `EvalTrait` -- has a `eval` function which returns what the view evaluates to.
    -   `SingleQuery` -- performs one address token's worth of immutable query.
-   It might still be useful to have `QueryTrait` and `QueryMutTrait` to produce `Box<dyn qv::EvalTrait>` and `Box<dyn qv::ApplyEditTrait>`, on which `eval()` or `apply_edit()` can be called.  This is in contrast with having the query be bound to a particular terminal operation (calling `eval()` or `apply_edit()` respectively).  But probably best to disable this stuff and keep things as simple as possible.  It really depends on if more terminal operations are eventually needed.

## 2023.05.21

Idea regarding how to move a "standard cursor" around the screen in a way that wouldn't surprise the layman:
-   When the up/down key is pressed, instead of doing a logical cursor change to the corresponding element of the next line (e.g. the down key changing cursor address `("line", 3, "char", 5)` to `("line", 4, "char", 5)`), calculate the position of the current cursor, then create a mouse click event for the position one line height's down from it, and push that event onto the front of the remaining_event_v queue, so it can be handled in the UI pass.
-   Come to think of it, this only works if there aren't other events in the event queue after the up/down arrow key press.

## 2023.05.22

Notes on fleshing out views and event handling for remainder of types
-   Showing cursor at the end of a string, array, etc.
-   Highly visible cursor
    -   Insert mode: Make an I-beam-shaped cursor that's vertical or horizontal that visually
        divides elements before and after the cursor.
    -   Replace mode: Draw the I-beam cursor and highlight the element that will be replaced.
-   Undo/redo
    -   Need top-level event handler which can intercept top-level commands.
-   Utf8StringTerm line char view
    -   To-do
    -   Done
        -   Insertion mode typing
        -   Typing '\n'
        -   Typing '\t'
        -   Backspace
        -   Delete
        -   Escape / Alt-Enter
-   Utf8StringTerm char view
    -   To-do
    -   Done
        -   Insertion mode typing
        -   Typing '\n'
        -   Typing '\t'
        -   Backspace
        -   Delete
        -   Escape / Alt-Enter
        -   Paste
-   ArrayTerm
    -   To-do
    -   Done
-   TupleTerm
    -   To-do
    -   Done
-   OrderedMapTerm
    -   To-do
    -   Done
-   StructTerm
    -   To-do
    -   Done
-   StructTermTerm
    -   To-do
    -   Done
-   GlobalSymRefTerm
    -   To-do
    -   Done
-   LocalSymRefTerm
    -   To-do
    -   Done

