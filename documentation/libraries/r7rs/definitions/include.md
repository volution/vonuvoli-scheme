

<a id='definition__r7rs__include'></a>

# `include` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `path`: value of type [path-string](../../r7rs/types/path-string.md#type__r7rs__path-string);

Syntax variants:
 * `(|_| |path| |...|)`


#### Referenced types

[`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);


#### Description

> ````
> (include <string_1> <string_2> ...)
> (include-ci <string_1> <string_2> ...)
> ````
> 
> 
> **Semantics**:
> Both `include` and
> `include-ci` take one or more filenames expressed as string literals,
> apply an implementation-specific algorithm to find corresponding files,
> read the contents of the files in the specified order as if by repeated applications
> of `read`,
> and effectively replace the `include` or `include-ci` expression
> with a `begin` expression containing what was read from the files.
> The difference between the two is that `include-ci` reads each file
> as if it began with the `#!fold-case` directive, while `include`
> does not.
> 
> **Note**:
> Implementations are encouraged to search for files in the directory
> which contains the including file, and to provide a way for users to
> specify other directories to search.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:compiler`](../../r7rs/categories/vs_3a_compiler.md#category__r7rs__vs_3a_compiler);
[`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

