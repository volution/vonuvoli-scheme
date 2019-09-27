

<a id='definition__r7rs__write-char'></a>

# `write-char` -- `r7rs` Definition


<a id='definition__r7rs__write-char__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__write-char__extended-by'></a>

#### Extended by

 * [`write-char`](../../vonuvoli/definitions/write-char.md#definition__vonuvoli__write-char) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__write-char__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((character) -> (undefined))`
   * input: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * `((character textual-output-port-open) -> (undefined))`
   * inputs:
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
     * a value of type [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


<a id='definition__r7rs__write-char__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__write-char__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__write-char__description'></a>

#### Description

> ````
> (write-char char)
> (write-char char port)
> ````
> 
> 
> Writes the character `char` (not an external representation of the
> character) to the given textual output `port` and returns an unspecified
> value.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__write-char__referenced-types'></a>

#### Referenced-types

 * [`character`](../../r7rs/types/character.md#type__r7rs__character);
 * [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

