

<a id='definition__r7rs__read-char'></a>

# `read-char` -- `r7rs` Definition


<a id='definition__r7rs__read-char__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__read-char__implemented-by'></a>

#### Implemented by

 * [`read-char`](../../vonuvoli/definitions/read-char.md#definition__vonuvoli__read-char) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__read-char__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (character-or-eof))`
   * inputs: none;
   * output: a value of type [`character-or-eof`](../../r7rs/types/character-or-eof.md#type__r7rs__character-or-eof);
 * `((textual-input-port-eof) -> (eof-object))`
   * input: a value of type [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
   * output: a value of type [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * `((textual-input-port-open) -> (character-or-eof))`
   * input: a value of type [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);
   * output: a value of type [`character-or-eof`](../../r7rs/types/character-or-eof.md#type__r7rs__character-or-eof);


<a id='definition__r7rs__read-char__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__read-char__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__read-char__description'></a>

#### Description

> ````
> (read-char)
> (read-char port)
> ````
> 
> 
> Returns the next character available from the textual input `port`,
> updating
> the `port` to point to the following character.  If no more characters
> are available, an end-of-file object is returned.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__read-char__referenced-types'></a>

#### Referenced-types

 * [`character-or-eof`](../../r7rs/types/character-or-eof.md#type__r7rs__character-or-eof);
 * [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
 * [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

