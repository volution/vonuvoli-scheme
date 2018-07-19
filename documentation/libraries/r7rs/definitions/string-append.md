

<a id='definition__r7rs__string-append'></a>

# `string-append` -- `r7rs` Definition


<a id='definition__r7rs__string-append__kind'></a>

#### Kind

`constructor`;


<a id='definition__r7rs__string-append__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (string-empty))`
   * inputs: none;
   * output: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * `((string ...) -> (string))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `...` (i.e. variadic);
   * output: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);


<a id='definition__r7rs__string-append__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__string-append__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__string-append__description'></a>

#### Description

> ````
> (string-append string ...)
> ````
> 
> 
> Returns a newly allocated string whose characters are the concatenation of the
> characters in the given strings.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__string-append__referenced-types'></a>

#### Referenced-types

 * [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * [`string`](../../r7rs/types/string.md#type__r7rs__string);


<a id='definition__r7rs__string-append__categories'></a>

#### Categories

 * [`vs:strings`](../../vonuvoli/categories/vs_3a_strings.md#category__vonuvoli__vs_3a_strings);


<a id='definition__r7rs__string-append__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

