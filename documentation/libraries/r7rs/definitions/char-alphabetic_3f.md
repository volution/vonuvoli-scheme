

<a id='definition__r7rs__char-alphabetic_3f'></a>

# `char-alphabetic?` -- `r7rs` Definitions


<a id='definition__r7rs__char-alphabetic_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__r7rs__char-alphabetic_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((character-ascii-alphabetic) -> (true))`
   * input: a value of type [`character-ascii-alphabetic`](../../r7rs/types/character-ascii-alphabetic.md#type__r7rs__character-ascii-alphabetic);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((character-alphabetic) -> (true))`
   * input: a value of type [`character-alphabetic`](../../r7rs/types/character-alphabetic.md#type__r7rs__character-alphabetic);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((character) -> (false))`
   * input: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__char-alphabetic_3f__exports'></a>

#### Exports

 * [`scheme:char`](../../r7rs/exports/scheme_3a_char.md#export__r7rs__scheme_3a_char) -- `(scheme char)`;


<a id='definition__r7rs__char-alphabetic_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__char-alphabetic_3f__description'></a>

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


<a id='definition__r7rs__char-alphabetic_3f__referenced-types'></a>

#### Referenced-types

 * [`character-ascii-alphabetic`](../../r7rs/types/character-ascii-alphabetic.md#type__r7rs__character-ascii-alphabetic);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`character-alphabetic`](../../r7rs/types/character-alphabetic.md#type__r7rs__character-alphabetic);
 * [`character`](../../r7rs/types/character.md#type__r7rs__character);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__char-alphabetic_3f__categories'></a>

#### Categories

 * [`vs:characters`](../../r7rs/categories/vs_3a_characters.md#category__r7rs__vs_3a_characters);


<a id='definition__r7rs__char-alphabetic_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

