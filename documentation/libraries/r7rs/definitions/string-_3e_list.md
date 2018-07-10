

<a id='definition__r7rs__string-_3e_list'></a>

# `string->list` -- `r7rs` Definitions


#### Kind

`converter`;


#### Procedure signature

Procedure variants:
 * `((|string-empty|) |->| (|null|))`
   * input: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
   * output: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * `((|string-not-empty|) |->| (|list-proper-not-null|))`
   * input: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
   * output: a value of type [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);
 * `((|string| |range-start|) |->| (|list-proper|))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
 * `((|string| |range-start| |range-end|) |->| (|list-proper|))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);


#### Referenced types

[`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
[`null`](../../r7rs/types/null.md#type__r7rs__null);
[`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
[`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);
[`string`](../../r7rs/types/string.md#type__r7rs__string);
[`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
[`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
[`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);


#### Description

> ````
> (string->list string)
> (string->list string start)
> (string->list string start end)
> (list->string list)
> ````
> 
> 
> **Domain**:  It is an error if any element of `list` is not a character.
> 
> The `string->list` procedure returns a newly allocated list of the
> characters of `string` between `start` and `end`.
> `list->string`
> returns a newly allocated string formed from the elements in the list
> `list`.
> In both procedures, order is preserved.
> `string->list`
> and `list->string` are
> inverses so far as `equal?` is concerned.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);
[`vs:lists`](../../r7rs/categories/vs_3a_lists.md#category__r7rs__vs_3a_lists);
[`vs:conversions`](../../r7rs/categories/vs_3a_conversions.md#category__r7rs__vs_3a_conversions);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

