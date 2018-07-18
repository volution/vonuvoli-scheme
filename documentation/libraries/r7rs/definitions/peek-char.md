

<a id='definition__r7rs__peek-char'></a>

# `peek-char` -- `r7rs` Definition


<a id='definition__r7rs__peek-char__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__peek-char__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (byte-or-eof))`
   * inputs: none;
   * output: a value of type [`byte-or-eof`](../../r7rs/types/byte-or-eof.md#type__r7rs__byte-or-eof);
 * `((textual-input-port-eof) -> (eof-object))`
   * input: a value of type [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
   * output: a value of type [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * `((textual-input-port-open) -> (character-or-eof))`
   * input: a value of type [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);
   * output: a value of type [`character-or-eof`](../../r7rs/types/character-or-eof.md#type__r7rs__character-or-eof);


<a id='definition__r7rs__peek-char__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__peek-char__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__peek-char__description'></a>

#### Description

> ````
> (peek-char)
> (peek-char port)
> ````
> 
> 
> Returns the next character available from the textual input `port`,
> but **without** updating
> the `port` to point to the following character.  If no more characters
> are available, an end-of-file object is returned.
> 
> **Note**:  The value returned by a call to `peek-char` is the same as the
> value that would have been returned by a call to `read-char` with the
> same `port`.  The only difference is that the very next call to
> `read-char` or `peek-char` on that `port` will return the
> value returned by the preceding call to `peek-char`.  In particular, a call
> to `peek-char` on an interactive port will hang waiting for input
> whenever a call to `read-char` would have hung.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__peek-char__referenced-types'></a>

#### Referenced-types

 * [`byte-or-eof`](../../r7rs/types/byte-or-eof.md#type__r7rs__byte-or-eof);
 * [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
 * [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);
 * [`character-or-eof`](../../r7rs/types/character-or-eof.md#type__r7rs__character-or-eof);


<a id='definition__r7rs__peek-char__categories'></a>

#### Categories

 * [`vs:ports:input`](../../r7rs/categories/vs_3a_ports_3a_input.md#category__r7rs__vs_3a_ports_3a_input);
 * [`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);
 * [`vs:characters`](../../r7rs/categories/vs_3a_characters.md#category__r7rs__vs_3a_characters);


<a id='definition__r7rs__peek-char__categories-recursive'></a>

#### Categories recursive

 * [`vs:ports`](../../r7rs/categories/vs_3a_ports.md#category__r7rs__vs_3a_ports);
 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

