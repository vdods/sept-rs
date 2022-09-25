# sept-rs

Rust SDK for Structured Expression Project Toolkit (SEPT)

Copyright 2021 by Victor Dods, licensed under Apache 2.0 -- see [LICENSE.txt].

## SEPT Mission Statement

The mission of `sept` is to replace text as the lingua franca for computers as used by humans.

This Rust SDK is a practical attempt at implementing the ideas in https://itdont.work/2020/06/29/structured-expression-project-thesis-1-0/ and has a corresponding [SDK in C++](https://github.com/vdods/sept).  The `sept` SDK(s) are still in early experimental stages.

Text is a very versatile representation for data as it relates to humans, and is made possible by humans' huge capacity for parsing meaning and structure out of relatively unstructured content. All of our tools are essentially text-based, even when it comes at a great cost in complexity. We conflate the representation of data with its "real", abstract form (program source code being rendered as/parsed from text being the prime example) and then forget that we've conflated the two.  We become mired in the abstraction inversion that is now invisible to us because of literal decades of thinking in a text-centric way.

Aside from the philosophical reasons for `sept`, there are plain matters of economy at play here. The more-vague and less-structured a data representation is, the more intelligence (e.g. parser, serializer) is needed to understand it.  This effectively siloes each locus of computing context, separating them with tiny little text pipes, at whose ends are complex, often ad-hoc encoding/decoding schemes.  At a basic level, all computer programs use structured data, and the it's absurd that there isn't a lingua franca for data with the following properties:
-   Allows computer programs to communicate near-natively; no need to accomodate the human.
-   Has ubiquitous tools (analogous to the text editor, the CLI, programming IDEs, etc) for viewing, navigating, searching, creating, and editing it.

## Goals

The practical goals of `sept` are:
-   Make types first class data, thereby making APIs, function signatures, and other schemas first class data.
-   Create a type system rich enough to represent basically any programming language data/type.
-   Substantially remove barriers to communication/interaction between data/computational contexts. In particular, facilitate conceptually lossless communication (i.e. serialization) between different programming languages (and programs).  This is achievable by virtue of the sufficiency of the type system and by the `sept` serialization scheme.
-   Create the analog of all the text-based tools (text editor, text diffs, text merging, search/replace/grep, etc) that will become the ubiquitous companions to what will be the lingua franca for data.  Use the [model-view-controller pattern](https://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller) to decouple data representation from content, and be able to quickly slice and dice the data for efficient and specific viewing/navigation/editing.
-   Bring data (and everything it represents) closer to being formalizable, in, for example:
    -   API definition.
    -   Versioning.
    -   Rich, canonical representations used for data authenticity/validation.
    -   Formal verification of software.

## Benchmarks of Success

Some measurable indicators of the success of `sept` are:
-   Broad elimination/automation of certain components of conventional programs, such as:
    -   Serialization
    -   API definition
    -   Arbitrary text-based program syntax and formatting conventions (see model-view-controller)
-   Near-replacement of many domain-specific editors (e.g. spreadsheets, programming IDEs, article-style content) with the generic `sept` editing tool.  Closer-to-full or even full replacement can be achieved through use of domain-specific plugins in the `sept` editing tool.
-   Qualitatively better command over creating/searching/editing data.  In particular, being able to alter the visible representation of the data in order to view/edit the specific aspects of the data.

## Structure of This Repo

Subdirs generally correspond to individual crates.
-   [`sept`](sept) : The `sept` SDK itself.

## Links to Docs/Notes

Generally, see [sept/doc].

Specifically:
-   [doc/sept/to-dos.md]
