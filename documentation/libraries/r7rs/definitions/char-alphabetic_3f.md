

<a id='definition__r7rs__char-alphabetic_3f'></a>

# `char-alphabetic?` -- `r7rs` Definitions


#### Kind

`predicate`;


#### Procedure signature

Procedure variants:
 * `((|character-ascii-alphabetic|) |->| (|true|))`
   * input: a value of type [`character-ascii-alphabetic`](../../r7rs/types/character-ascii-alphabetic.md#type__r7rs__character-ascii-alphabetic);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|character-alphabetic|) |->| (|true|))`
   * input: a value of type [`character-alphabetic`](../../r7rs/types/character-alphabetic.md#type__r7rs__character-alphabetic);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|character|) |->| (|false|))`
   * input: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


#### Referenced types

[`character-ascii-alphabetic`](../../r7rs/types/character-ascii-alphabetic.md#type__r7rs__character-ascii-alphabetic);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`character-alphabetic`](../../r7rs/types/character-alphabetic.md#type__r7rs__character-alphabetic);
[`character`](../../r7rs/types/character.md#type__r7rs__character);
[`false`](../../r7rs/types/false.md#type__r7rs__false);


#### Description

> ````
> (char-alphabetic? char)
> (char-numeric? char)
> (char-whitespace? char)
> (char-upper-case? letter)
> (char-lower-case? letter)
> ````
> 
> 
> These procedures return `#t` if their arguments are alphabetic,
> numeric, whitespace, upper case, or lower case characters, respectively,
> otherwise they return `#f`.
> 
> Specifically, they must return `#t` when applied to characters with
> the Unicode properties Alphabetic, Numeric_Digit, White_Space, Uppercase, and
> Lowercase respectively, and `#f` when applied to any other Unicode
> characters.  Note that many Unicode characters are alphabetic but neither
> upper nor lower case.
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

