

<a id='definition__r7rs__read-u8'></a>

# `read-u8` -- `r7rs` Definition


<a id='definition__r7rs__read-u8__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__read-u8__implemented-by'></a>

#### Implemented by

 * [`read-u8`](../../vonuvoli/definitions/read-u8.md#definition__vonuvoli__read-u8) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__read-u8__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (byte-or-eof))`
   * inputs: none;
   * output: a value of type [`byte-or-eof`](../../r7rs/types/byte-or-eof.md#type__r7rs__byte-or-eof);
 * `((binary-input-port-eof) -> (eof-object))`
   * input: a value of type [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
   * output: a value of type [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * `((binary-input-port-open) -> (byte-or-eof))`
   * input: a value of type [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);
   * output: a value of type [`byte-or-eof`](../../r7rs/types/byte-or-eof.md#type__r7rs__byte-or-eof);


<a id='definition__r7rs__read-u8__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__read-u8__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__read-u8__description'></a>

#### Description

> ````
> (read-u8)
> (read-u8 port)
> ````
> 
> 
> Returns the next byte available from the binary input `port`,
> updating the `port` to point to the following byte.
> If no more bytes are
> available, an end-of-file object is returned.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__read-u8__referenced-types'></a>

#### Referenced-types

 * [`byte-or-eof`](../../r7rs/types/byte-or-eof.md#type__r7rs__byte-or-eof);
 * [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
 * [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

