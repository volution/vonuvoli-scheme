

<a id='definition__r7rs__import'></a>

# `import` -- `r7rs` Definition


<a id='definition__r7rs__import__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__import__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `import`: value of type [eval-environment-import](../../r7rs/types/eval-environment-import.md#type__r7rs__eval-environment-import);

Syntax variants:
 * `(_ import ...)`


<a id='definition__r7rs__import__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__import__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__import__description'></a>

#### Description

> ````
> (import <import-set> ...)
> ````
> 
> 
> An import declaration provides a way to import identifiers
> exported by a library.  Each `<import-set>` names a set of bindings
> from a library and possibly specifies local names for the
> imported bindings. It takes one of the following forms:
> 
>   * `<library-name>`
>   * `(only <import-set> <identifier> ...)`
>   * `(except <import-set> <identifier> ...)`
>   * `(prefix <import-set> <identifier>)`
>   * `(rename <import-set> (<identifier_1> <identifier_2>) ...)`
> 
> In the first form, all of the identifiers in the named library's export
> clauses are imported with the same names (or the exported names if
> exported with `rename`).  The additional `<import-set>`
> forms modify this set as follows:
> 
>   * `only` produces a subset of the given
>   `<import-set>` including only the listed identifiers (after any
>   renaming).  It is an error if any of the listed identifiers are
>   not found in the original set.
> 
>   * `except` produces a subset of the given
>   `<import-set>`, excluding the listed identifiers (after any
>   renaming). It is an error if any of the listed identifiers are not
>   found in the original set.
> 
>   * `rename` modifies the given `<import-set>`,
>   replacing each instance of `<identifier_1>` with
>   `<identifier_2>`. It is an error if any of the listed
>   `<identifier_1>`s are not found in the original set.
> 
>   * `prefix` automatically renames all identifiers in
>   the given `<import-set>`, prefixing each with the specified
>   `<identifier>`.
> 
> In a program or library declaration, it is an error to import the same
> identifier more than once with different bindings, or to redefine or
> mutate an imported binding with a definition
> or with `set!`, or to refer to an identifier before it is imported.
> However, a REPL should permit these actions.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__import__referenced-types'></a>

#### Referenced-types

 * [`eval-environment-import`](../../r7rs/types/eval-environment-import.md#type__r7rs__eval-environment-import);


<a id='definition__r7rs__import__categories'></a>

#### Categories

 * [`vs:compiler`](../../r7rs/categories/vs_3a_compiler.md#category__r7rs__vs_3a_compiler);
 * [`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);


<a id='definition__r7rs__import__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

