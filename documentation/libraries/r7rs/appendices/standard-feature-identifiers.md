

<a id='appendix__r7rs__standard-feature-identifiers'></a>

# `r7rs` -- Standard Feature Identifiers


<a id='appendix__r7rs__standard-feature-identifiers__description'></a>

#### Description

> An implementation may provide any or all of the feature identifiers
> listed below for use by `cond-expand` and `features`,
> but must not provide a feature identifier if it does not
> provide the corresponding feature.
> 
>   * `r7rs` -- All __R7RS__ Scheme implementations have this feature.
> 
>   * `exact-closed` -- All algebraic operations except `/` produce exact values given exact inputs.
> 
>   * `exact-complex` -- Exact complex numbers are provided.
> 
>   * `ieee-float` -- Inexact numbers are __IEEE 754__ binary floating point values.
> 
>   * `full-unicode` -- All Unicode characters present in __Unicode version 6.0__ are supported as Scheme characters.
> 
>   * `ratios` -- `/` with exact arguments produces an exact result when the divisor is nonzero.
> 
>   * `posix` -- This implementation is running on a __POSIX__ system.
> 
>   * `windows` -- This implementation is running on __Windows__.
> 
>   * `unix`, `darwin`, `gnu-linux`, `bsd`, `freebsd`, `solaris`, ... -- Operating system flags (perhaps more than one).
> 
>   * `i386`, `x86-64`, `ppc`, `sparc`, `jvm`, `clr`, `llvm`, ... -- CPU architecture flags.
> 
>   * `ilp32`, `lp64`, `ilp64`, ... -- C memory model flags.
> 
>   * `big-endian`, `little-endian` -- Byte order flags.
> 
>   * `<name>` -- The name of this implementation.
> 
>   * `<name-version>` -- The name and version of this implementation.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

