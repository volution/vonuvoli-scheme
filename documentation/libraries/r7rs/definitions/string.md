

<a id='definition__r7rs__string'></a>

# `string` -- `r7rs` Definitions


#### Kind

`constructor`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|string-empty|))`
   * inputs: none;
   * output: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * `((|character| |...|) |->| (|string-not-empty|))`
   * inputs:
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
     * `...` (i.e. variadic);
   * output: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);


#### Referenced types

[`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
[`character`](../../r7rs/types/character.md#type__r7rs__character);
[`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);


#### Description

> ````
> (string char ...)
> ````
> 
> 
> Returns a newly allocated string composed of the arguments.
> It is analogous to `list`.
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

