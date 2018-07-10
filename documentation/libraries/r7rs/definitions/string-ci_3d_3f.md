

<a id='definition__r7rs__string-ci_3d_3f'></a>

# `string-ci=?` -- `r7rs` Definitions


#### Kind

`comparator`;


#### Procedure signature

Procedure variants:
 * `((|string|) |->| (|true|))`
   * input: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|string| |...|) |->| (|boolean|))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Referenced types

[`string`](../../r7rs/types/string.md#type__r7rs__string);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Description

> ````
> (string-ci=? string_1 string_2 string_3 ...)
> ````
> 
> 
> Returns `#t` if, after case-folding, all the strings are the same
> length and contain the same characters in the same positions, otherwise
> returns `#f`.  Specifically, these procedures behave as if
> `string-foldcase` were applied to their arguments before comparing them.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:char`](../../r7rs/categories/r7rs_3a_char.md#category__r7rs__r7rs_3a_char);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);
[`vs:comparisons`](../../r7rs/categories/vs_3a_comparisons.md#category__r7rs__vs_3a_comparisons);
[`vs:equivalence`](../../r7rs/categories/vs_3a_equivalence.md#category__r7rs__vs_3a_equivalence);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

