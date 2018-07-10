

<a id='definition__r7rs__string-length'></a>

# `string-length` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|string-empty|) |->| (|range-length-zero|))`
   * input: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
   * output: a value of type [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
 * `((|string-not-empty|) |->| (|range-length-not-zero|))`
   * input: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
   * output: a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);


#### Referenced types

[`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
[`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
[`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
[`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);


#### Description

> ````
> (string-length string)
> ````
> 
> 
> Returns the number of characters in the given `string`.
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

