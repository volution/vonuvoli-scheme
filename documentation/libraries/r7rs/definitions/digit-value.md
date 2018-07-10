

<a id='definition__r7rs__digit-value'></a>

# `digit-value` -- `r7rs` Definitions


#### Kind

`converter`;


#### Procedure signature

Procedure variants:
 * `((|character-ascii-numeric|) |->| (|exact-integer-positive-or-zero|))`
   * input: a value of type [`character-ascii-numeric`](../../r7rs/types/character-ascii-numeric.md#type__r7rs__character-ascii-numeric);
   * output: a value of type [`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero);
 * `((|character-numeric|) |->| (|exact-integer-positive-or-zero|))`
   * input: a value of type [`character-numeric`](../../r7rs/types/character-numeric.md#type__r7rs__character-numeric);
   * output: a value of type [`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero);
 * `((|character|) |->| (|false|))`
   * input: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


#### Referenced types

[`character-ascii-numeric`](../../r7rs/types/character-ascii-numeric.md#type__r7rs__character-ascii-numeric);
[`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero);
[`character-numeric`](../../r7rs/types/character-numeric.md#type__r7rs__character-numeric);
[`character`](../../r7rs/types/character.md#type__r7rs__character);
[`false`](../../r7rs/types/false.md#type__r7rs__false);


#### Description

> ````
> (digit-value char)
> ````
> 
> 
> This procedure returns the numeric value (`0` to `9`) of its argument
> if it is a numeric digit (that is, if `char-numeric?` returns `#t`),
> or `#f` on any other character.
> 
> ````
> (digit-value #\3)      ===>  3
> (digit-value #\x0664)  ===>  4
> (digit-value #\x0AE6)  ===>  0
> (digit-value #\x0EA6)  ===>  #f
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:char`](../../r7rs/categories/r7rs_3a_char.md#category__r7rs__r7rs_3a_char);
[`vs:characters`](../../r7rs/categories/vs_3a_characters.md#category__r7rs__vs_3a_characters);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

