

<a id='definition__r7rs__char-upcase'></a>

# `char-upcase` -- `r7rs` Definitions


<a id='definition__r7rs__char-upcase__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__char-upcase__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((character) -> (character))`
   * input: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);


<a id='definition__r7rs__char-upcase__exports'></a>

#### Exports

 * [`scheme:char`](../../r7rs/exports/scheme_3a_char.md#export__r7rs__scheme_3a_char) -- `(scheme char)`;


<a id='definition__r7rs__char-upcase__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__char-upcase__description'></a>

#### Description

> ````
> (char-upcase char)
> (char-downcase char)
> (char-foldcase char)
> ````
> 
> 
> The `char-upcase` procedure, given an argument that is the
> lowercase part of a Unicode casing pair, returns the uppercase member
> of the pair, provided that both characters are supported by the Scheme
> implementation.  Note that language-sensitive casing pairs are not used.  If the
> argument is not the lowercase member of such a pair, it is returned.
> 
> The `char-downcase` procedure, given an argument that is the
> uppercase part of a Unicode casing pair, returns the lowercase member
> of the pair, provided that both characters are supported by the Scheme
> implementation.  Note that language-sensitive casing pairs are not used.  If the
> argument is not the uppercase member of such a pair, it is returned.
> 
> The `char-foldcase` procedure applies the Unicode simple
> case-folding algorithm to its argument and returns the result.  Note that
> language-sensitive folding is not used.  If the argument is an uppercase
> letter, the result will be either a lowercase letter
> or the same as the argument if the lowercase letter does not exist or
> is not supported by the implementation.
> See __UAX #29__ (part of the __Unicode Standard__) for details.
> 
> Note that many Unicode lowercase characters do not have uppercase
> equivalents.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__char-upcase__referenced-types'></a>

#### Referenced-types

 * [`character`](../../r7rs/types/character.md#type__r7rs__character);


<a id='definition__r7rs__char-upcase__categories'></a>

#### Categories

 * [`vs:characters`](../../r7rs/categories/vs_3a_characters.md#category__r7rs__vs_3a_characters);


<a id='definition__r7rs__char-upcase__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

