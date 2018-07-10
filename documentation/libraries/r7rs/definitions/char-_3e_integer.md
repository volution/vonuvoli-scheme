

<a id='definition__r7rs__char-_3e_integer'></a>

# `char->integer` -- `r7rs` Definitions


#### Kind

`converter`;


#### Procedure signature

Procedure variants:
 * `((|character-ascii|) |->| (|code-point-ascii|))`
   * input: a value of type [`character-ascii`](../../r7rs/types/character-ascii.md#type__r7rs__character-ascii);
   * output: a value of type [`code-point-ascii`](../../r7rs/types/code-point-ascii.md#type__r7rs__code-point-ascii);
 * `((|character|) |->| (|code-point-unicode|))`
   * input: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`code-point-unicode`](../../r7rs/types/code-point-unicode.md#type__r7rs__code-point-unicode);


#### Referenced types

[`character-ascii`](../../r7rs/types/character-ascii.md#type__r7rs__character-ascii);
[`code-point-ascii`](../../r7rs/types/code-point-ascii.md#type__r7rs__code-point-ascii);
[`character`](../../r7rs/types/character.md#type__r7rs__character);
[`code-point-unicode`](../../r7rs/types/code-point-unicode.md#type__r7rs__code-point-unicode);


#### Description

> ````
> (char->integer char)
> (integer->char n)
> ````
> 
> 
> Given a Unicode character,
> `char->integer` returns an exact integer
> between `0` and `#xD7FF` or
> between `#xE000` and `#x10FFFF`
> which is equal to the Unicode scalar value of that character.
> Given a non-Unicode character,
> it returns an exact integer greater than `#x10FFFF`.
> This is true independent of whether the implementation uses
> the Unicode representation internally.
> 
> Given an exact integer that is the value returned by
> a character when `char->integer` is applied to it, `integer->char`
> returns that character.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:characters`](../../r7rs/categories/vs_3a_characters.md#category__r7rs__vs_3a_characters);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

