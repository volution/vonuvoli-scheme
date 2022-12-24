

<a id='definition__r7rs__digit-value'></a>

# `digit-value` -- `r7rs` Definition


<a id='definition__r7rs__digit-value__kind'></a>

#### Kind

`converter`;


<a id='definition__r7rs__digit-value__implemented-by'></a>

#### Implemented by

 * [`digit-value`](../../vonuvoli/definitions/digit-value.md#definition__vonuvoli__digit-value) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__digit-value__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((character-ascii-numeric) -> (exact-integer-positive-or-zero))`
   * input: a value of type [`character-ascii-numeric`](../../r7rs/types/character-ascii-numeric.md#type__r7rs__character-ascii-numeric);
   * output: a value of type [`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero);
 * `((character-numeric) -> (exact-integer-positive-or-zero))`
   * input: a value of type [`character-numeric`](../../r7rs/types/character-numeric.md#type__r7rs__character-numeric);
   * output: a value of type [`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero);
 * `((character) -> (false))`
   * input: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__digit-value__exports'></a>

#### Exports

 * [`scheme:char`](../../r7rs/exports/scheme_3a_char.md#export__r7rs__scheme_3a_char) -- `(scheme char)`;


<a id='definition__r7rs__digit-value__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__digit-value__description'></a>

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


<a id='definition__r7rs__digit-value__referenced-types'></a>

#### Referenced-types

 * [`character-ascii-numeric`](../../r7rs/types/character-ascii-numeric.md#type__r7rs__character-ascii-numeric);
 * [`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero);
 * [`character-numeric`](../../r7rs/types/character-numeric.md#type__r7rs__character-numeric);
 * [`character`](../../r7rs/types/character.md#type__r7rs__character);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

