

<a id='definition__r7rs__string-append'></a>

# `string-append` -- `r7rs` Definitions


#### Kind

`constructor`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|string-empty|))`
   * inputs: none;
   * output: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * `((|string| |...|) |->| (|string|))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `...` (i.e. variadic);
   * output: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);


#### Referenced types

[`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
[`string`](../../r7rs/types/string.md#type__r7rs__string);


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


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

