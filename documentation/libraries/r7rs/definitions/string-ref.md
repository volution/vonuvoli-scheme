

<a id='definition__r7rs__string-ref'></a>

# `string-ref` -- `r7rs` Definition


<a id='definition__r7rs__string-ref__kind'></a>

#### Kind

`accessor`;


<a id='definition__r7rs__string-ref__implemented-by'></a>

#### Implemented by

 * [`string-ref`](../../vonuvoli/definitions/string-ref.md#definition__vonuvoli__string-ref) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__string-ref__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((string-not-empty range-offset) -> (character))`
   * inputs:
     * a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
   * output: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);


<a id='definition__r7rs__string-ref__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__string-ref__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__string-ref__description'></a>

#### Description

> ````
> (string-ref string k)
> ````
> 
> 
> **Domain**:  It is an error if `k` is not a valid index of `string`.
> 
> The `string-ref` procedure returns character `k` of `string` using zero-origin indexing.
> 
> 
> **Note**:  There is no requirement for this procedure to execute in constant time.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__string-ref__referenced-types'></a>

#### Referenced-types

 * [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
 * [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
 * [`character`](../../r7rs/types/character.md#type__r7rs__character);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

