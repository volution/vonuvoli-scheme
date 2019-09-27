

<a id='definition__r7rs__string-ci_3d_3f'></a>

# `string-ci=?` -- `r7rs` Definition


<a id='definition__r7rs__string-ci_3d_3f__kind'></a>

#### Kind

`comparator`;


<a id='definition__r7rs__string-ci_3d_3f__implemented-by'></a>

#### Implemented by

 * [`string-ci=?`](../../vonuvoli/definitions/string-ci_3d_3f.md#definition__vonuvoli__string-ci_3d_3f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__string-ci_3d_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((string |2...|) -> (boolean))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `...` -- at least 2 times;
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__string-ci_3d_3f__exports'></a>

#### Exports

 * [`scheme:char`](../../r7rs/exports/scheme_3a_char.md#export__r7rs__scheme_3a_char) -- `(scheme char)`;


<a id='definition__r7rs__string-ci_3d_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__string-ci_3d_3f__description'></a>

#### Description

> ````
> (string-ci=? string_1 string_2 string_3 ...)
> ````
> 
> 
> Returns `#t` if, after case-folding, all the strings are the same
> length and contain the same characters in the same positions, otherwise
> returns `#f`.  Specifically, these procedures behave as if
> `string-foldcase` were applied to their arguments before comparing them.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__string-ci_3d_3f__referenced-types'></a>

#### Referenced-types

 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

