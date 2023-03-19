# Notes

## 2023.03.09

### De/Serialization Feature

Currently working to complete the de/serialization feature, but running into problems regarding the well-definedness of de/serializing GlobalSymRefTerm and LocalSymRefTerm.  The problems generally have to do with the transparent reference semantics of the sym ref types.  I.e. the value of a sym ref is the value that it dereferences to.  However, when de/serializing, the literal sym ref itself is what should be handled.  This introduces two concerns:
-   The referred-to content of the relevant symbol tables must somehow exist on the deserialization side.
    -   This comes with either:
        -   a "promise" that the relevant symbol table content will exist on the other side, or
        -   serializing the referred-to content in a kind of symbol table within the serialized output.
-   Need special case code for the sym ref types so that
    -   they don't dereference when serializing, so that they serialize as a literal GlobalSymRefTerm/LocalSymRefTerm, and
    -   upon deserialization, aren't ever dereferenced, thereby splitting off the concern of ensuring the existence of the referred-to symbol table content.

In order to discuss de/serialization of GlobalSymRefTerm/LocalSymRefTerm, it's necessary to outline the current behavior of de/serialization for non-reference types.
-   Serialization of a term `x` implemented by Rust type `T`
    -   `T` must implement `Serializable`, which has a single method `fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize>`, returning the number of bytes written to the writer.  The specific content written is totally type-specific.
    -   Examples of serialization
        -   `bool` simply writes a single byte `0` or `1` to represent `false` or `true`.
        -   `i32` simply writes the little-endian byte representation of the value.
        -   `ArrayTerm` writes the number of elements in the array as a `u64` value, then calls `serialize` on each of its elements.
        -   `StructTerm` writes the number of fields in the struct as a `u64` value, then calls `serialize` on the field name and then the field type.
        -   `StructTermTerm` calls `serialize` on the direct type and then the field tuple.  Note that including the direct type here is not a canonical choice, but simplifies some logic.  It might complicate things elsewhere.
        -   `GlobalSymRefTerm` serializes the `symbol_id` field.
        -   `LocalSymRefTerm` *should* serialize a value which uniquely identifies which local symbol table is being referred to, and then should serialize the `symbol_id` field.
        -   `Value` acts as a kind of `Any` and therefore must also encode the type information, and therefore can store reference and non-reference types, *should*:
            -   write top-level-code "construction"
            -   serialize the constructor for the value (currently this is value.nondereferencing_abstract_type)
            -   don't reference the value, and just directly serialize.
-   Deserialization of a term `x` implemented by Rust type `T`
    -   If the type `T` is known in advance, then `T::deserialize` can be called.
    -   If the type `T` is not known in advance, then a `Value` should be read, which will either:
        -   read top-level-code "non-parametric term" and then the `NonParametricTermCode`, and construct that, or
        -   read top-level-code "construction", then the constructor, and then will call `deserialize_parameters_and_construct` on the constructor.
    -   There are certain actions deserialization wants to take that have to be deferred in order to conform to the design criteria of "no sym ref resolution before/during deserialization" and the corollary "can't [necessarily] check well-formedness of values during deserialization".  This suggests that there has to be "deferred" versions of these actions which would be performed in a second pass after deserialization is complete.

## 2023.03.17

### De/Serialization Feature

After thinking about this for a while, serialization should also serialize the entries in the relevant symbol tables that are referred to by sym refs in the serialized data.  Later, there can be a function that applies a "mask" to which entries are serialized, e.g. for when it's known that that symbol table content will already be present on the deserialization side, such as when there are "standard entries" always pre-loaded.  For "masked out" symbol table entries, they should probably still be present in the serialized symbol table, just marked as "extern", and perhaps their type included, so that formal reasoning about all symbolic references in the serialized data can be done easily.

Serializing static types
-   Directly serializing an instance of a particular Rust struct type, call it `S`, will only serialize its contents, not its type.
-   Serializing anything wrapped in `sept::dy::Value` will cause its type to be serialized.  Thus serializing an instance of `S` inside a `Value` must include serializing `S` in some form.  Because `S` might occur many times in the serialization, the type could simply be a sym ref to a serialization of `S` in a/the symbol table of the serialization.  Then, when deserializing that data, either:
    -   It's being deserialized into `Value`, in which case the type is necessary in order to construct the value, so the serialized `S` is deserialized and is used to construct the value, or
    -   It's being deserialized into `S` (the Rust type), in which case the definition in the Rust program of `S` is checked against the definition in the serialized symbol table just to ensure it's actually equal/compatible.

Having a separate segment in the serialization for the symbol tables sort of paves the way to have another, separate segment in the serialization for a "heap", which would give a more direct serialization analog for data structures in memory, and could potentially make it so that (assuming the endianness and data layouts match exactly), the deserialization could work by simply reading a whole block into memory and then linking the appropriate pointers in that block.  This would allow efficient queries to be done on serialized data without deserializing the whole thing.

Writing symbol tables
-   Ideally, each symbol table is written in its entirety in a contiguous block, though that may not be possible when producing serialization symbol tables during traversal of the data to be serialized.
-   It's not actually required to write symbol table entries contiguously, though having the symbol table entries disorganized makes queries involving them much slower.
-   There could be a post-processing pass on the serialized data where the symbol table entries are sorted first by which symbol table, then by symbol name.
-   Analogous logic applies to a heap segment; the heap would involve "allocations" for each bit of serialization, and would potentially suffer from memory fragmentation, but could use a post-processing step to defragment them to improve query time.

Regarding the heap
-   The point of the heap is to have the "main" data have a known layout so that precise, constant-time queries can be done on it, and dynamic portions of the data are put on the heap.
-   In order for serialized symbol table lookups to be efficient, serialized symbol table entries should have a fixed size, and their dynamically-sized data can be on the serialized heap.

Writing to different segments is trickier than serialization of conventional data such as JSON.  Here are a few options:
-   Have three different output streams
    -   They could be combined after output concludes.
    -   They could be kept separate so that modifications to the file require little extra processing.
-   Have one output stream, but buffer the two others in memory, apply post-processing, and then write the two memory buffers to the stream.

