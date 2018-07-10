

<a id='definition__r7rs__char-ci_3d_3f'></a>

# `char-ci=?` -- `r7rs` Definitions


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
> (char-ci=? char_1 char_2 char_3 ...)
> (char-ci<? char_1 char_2 char_3 ...)
> (char-ci>? char_1 char_2 char_3 ...)
> (char-ci<=? char_1 char_2 char_3 ...)
> (char-ci>=? char_1 char_2 char_3 ...)
> ````
> 
> 
> These procedures are similar to `char=?` et cetera, but they treat
> upper case and lower case letters as the same.  For example,
> `(char-ci=? #\A #\a)` returns `#t`.
> 
> Specifically, these procedures behave as if `char-foldcase` were
> applied to their arguments before they were compared.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:char`](../../r7rs/categories/r7rs_3a_char.md#category__r7rs__r7rs_3a_char);
[`vs:characters`](../../r7rs/categories/vs_3a_characters.md#category__r7rs__vs_3a_characters);
[`vs:comparisons`](../../r7rs/categories/vs_3a_comparisons.md#category__r7rs__vs_3a_comparisons);
[`vs:equivalence`](../../r7rs/categories/vs_3a_equivalence.md#category__r7rs__vs_3a_equivalence);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

