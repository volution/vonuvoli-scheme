

<a id='appendix__r7rs__programs'></a>

# `r7rs` -- Programs


#### Description

> A Scheme program consists of
> one or more import declarations followed by a sequence of
> expressions and definitions.
> Import declarations specify the libraries on which a program or library depends;
> a subset of the identifiers exported by the libraries are made available to
> the program.
> Expressions are described in section on expressions.
> Definitions are either variable definitions, syntax definitions, or
> record-type definitions, all of which are explained in this chapter.
> They are valid in some, but not all, contexts where expressions
> are allowed, specifically at the outermost level of a `<program>`
> and at the beginning of a `<body>`.
> 
> At the outermost level of a program, `(begin <expression or definition> ...)` is
> equivalent to the sequence of expressions and definitions
> in the `begin`.
> Similarly, in a `<body>`, `(begin <definition_1> ...)` is equivalent
> to the sequence `<definition> ...`.
> Macros can expand into such `begin` forms.
> For the formal definition, see section on sequencing.
> 
> Import declarations and definitions
> cause bindings to be created in the global
> environment or modify the value of existing global bindings.
> The initial environment of a program is empty,
> so at least one import declaration is needed to introduce initial bindings.
> 
> Expressions occurring at the outermost level of a program
> do not create any bindings.  They are
> executed in order when the program is
> invoked or loaded, and typically perform some kind of initialization.
> 
> Programs and libraries are typically stored in files, although
> in some implementations they can be entered interactively into a running
> Scheme system.  Other paradigms are possible.
> Implementations which store libraries in files should document the
> mapping from the name of a library to its location in the file system.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

