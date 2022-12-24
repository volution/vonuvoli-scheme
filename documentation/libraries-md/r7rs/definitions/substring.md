

<a id='definition__r7rs__substring'></a>

# `substring` -- `r7rs` Definition


<a id='definition__r7rs__substring__kind'></a>

#### Kind

`accessor`;


<a id='definition__r7rs__substring__extended-by'></a>

#### Extended by

 * [`string-copy`](../../r7rs/definitions/string-copy.md#definition__r7rs__string-copy) (from [`r7rs`](../../r7rs/_index.md#library__r7rs));


<a id='definition__r7rs__substring__implemented-by'></a>

#### Implemented by

 * [`string-copy`](../../vonuvoli/definitions/string-copy.md#definition__vonuvoli__string-copy) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__substring__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((string range-start range-end) -> (string))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);


<a id='definition__r7rs__substring__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__substring__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__substring__description'></a>

#### Description

> ````
> (substring string start end)
> ````
> 
> 
> The `substring` procedure returns a newly allocated string formed from the characters of
> `string` beginning with index `start` and ending with index
> `end`.
> This is equivalent to calling `string-copy` with the same arguments,
> but is provided for backward compatibility and
> stylistic flexibility.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__substring__referenced-types'></a>

#### Referenced-types

 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
 * [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

