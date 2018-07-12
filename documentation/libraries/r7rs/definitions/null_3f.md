

<a id='definition__r7rs__null_3f'></a>

# `null?` -- `r7rs` Definitions


#### Kind

`type-predicate`;


#### Procedure signature

Procedure variants:
 * `((|null|) |->| (|true|))`
   * input: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|pair|) |->| (|false|))`
   * input: a value of type [`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|any|) |->| (|false|))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|any| |...|) |->| (|boolean|))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `|vonuvoli|`


#### Referenced types

[`null`](../../r7rs/types/null.md#type__r7rs__null);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
[`false`](../../r7rs/types/false.md#type__r7rs__false);
[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Description

> ````
> (null? obj)
> ````
> 
> 
> Returns `#t` if `obj` is the __empty list__,
> otherwise returns `#f`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:lists`](../../r7rs/categories/vs_3a_lists.md#category__r7rs__vs_3a_lists);
[`vs:types`](../../r7rs/categories/vs_3a_types.md#category__r7rs__vs_3a_types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----
