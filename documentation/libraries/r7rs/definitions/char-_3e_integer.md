

<a id='definition__r7rs__char-_3e_integer'></a>

# `char->integer` -- `r7rs` Definition


<a id='definition__r7rs__char-_3e_integer__kind'></a>

#### Kind

`converter`;


<a id='definition__r7rs__char-_3e_integer__implemented-by'></a>

#### Implemented by

 * [`char->integer`](../../vonuvoli/definitions/char-_3e_integer.md#definition__vonuvoli__char-_3e_integer) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__char-_3e_integer__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((character-ascii) -> (code-point-ascii))`
   * input: a value of type [`character-ascii`](../../r7rs/types/character-ascii.md#type__r7rs__character-ascii);
   * output: a value of type [`code-point-ascii`](../../r7rs/types/code-point-ascii.md#type__r7rs__code-point-ascii);
 * `((character) -> (code-point-unicode))`
   * input: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`code-point-unicode`](../../r7rs/types/code-point-unicode.md#type__r7rs__code-point-unicode);


<a id='definition__r7rs__char-_3e_integer__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__char-_3e_integer__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__char-_3e_integer__description'></a>

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


<a id='definition__r7rs__char-_3e_integer__referenced-types'></a>

#### Referenced-types

 * [`character-ascii`](../../r7rs/types/character-ascii.md#type__r7rs__character-ascii);
 * [`code-point-ascii`](../../r7rs/types/code-point-ascii.md#type__r7rs__code-point-ascii);
 * [`character`](../../r7rs/types/character.md#type__r7rs__character);
 * [`code-point-unicode`](../../r7rs/types/code-point-unicode.md#type__r7rs__code-point-unicode);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

