

<a id='definition__r7rs__char_3d_3f'></a>

# `char=?` -- `r7rs` Definitions


#### Kind

`comparator`;


#### Procedure signature

Procedure variants:
 * `((|character|) |->| (|true|))`
   * input: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|character| |...|) |->| (|boolean|))`
   * inputs:
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Referenced types

[`character`](../../r7rs/types/character.md#type__r7rs__character);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Description

> ````
> (char=? char_1 char_2 char_3 ...)
> (char<? char_1 char_2 char_3 ...)
> (char>? char_1 char_2 char_3 ...)
> (char<=? char_1 char_2 char_3 ...)
> (char>=? char_1 char_2 char_3 ...)
> ````
> 
> 
> These procedures return `#t` if
> the results of passing their arguments to `char->integer`
> are respectively
> equal, monotonically increasing, monotonically decreasing,
> monotonically non-decreasing, or monotonically non-increasing.
> 
> These predicates are required to be transitive.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:characters`](../../r7rs/categories/vs_3a_characters.md#category__r7rs__vs_3a_characters);
[`vs:comparisons`](../../r7rs/categories/vs_3a_comparisons.md#category__r7rs__vs_3a_comparisons);
[`vs:equivalence`](../../r7rs/categories/vs_3a_equivalence.md#category__r7rs__vs_3a_equivalence);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

