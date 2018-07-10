

<a id='definition__r7rs__string-ref'></a>

# `string-ref` -- `r7rs` Definitions


#### Kind

`accessor`;


#### Procedure signature

Procedure variants:
 * `((|string| |range-offset|) |->| (|character|))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
   * output: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);


#### Referenced types

[`string`](../../r7rs/types/string.md#type__r7rs__string);
[`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
[`character`](../../r7rs/types/character.md#type__r7rs__character);


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


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

