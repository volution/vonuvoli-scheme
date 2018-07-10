

<a id='definition__r7rs__string-fill_21'></a>

# `string-fill!` -- `r7rs` Definitions


#### Kind

`mutator!`;


#### Procedure signature

Procedure variants:
 * `((|string| |character|) |->| (|void|))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `((|string| |character| |range-start|) |->| (|void|))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `((|string| |character| |range-start| |range-end|) |->| (|void|))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);


#### Referenced types

[`string`](../../r7rs/types/string.md#type__r7rs__string);
[`character`](../../r7rs/types/character.md#type__r7rs__character);
[`void`](../../r7rs/types/void.md#type__r7rs__void);
[`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
[`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);


#### Description

> ````
> (string-fill! string fill)
> (string-fill! string fill start)
> (string-fill! string fill start end)
> ````
> 
> 
> **Domain**:  It is an error if `fill` is not a character.
> 
> The `string-fill!` procedure stores `fill`
> in the elements of `string`
> between `start` and `end`.
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

