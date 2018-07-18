

<a id='definition__r7rs__write-u8'></a>

# `write-u8` -- `r7rs` Definition


<a id='definition__r7rs__write-u8__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__write-u8__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((byte) -> (undefined))`
   * input: a value of type [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
   * requires: `(not vonuvoli)`
 * `((byte) -> (void))`
   * input: a value of type [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
   * requires: `vonuvoli`
 * `((byte binary-output-port-open) -> (undefined))`
   * inputs:
     * a value of type [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
     * a value of type [`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
   * requires: `(not vonuvoli)`
 * `((byte binary-output-port-open) -> (void))`
   * inputs:
     * a value of type [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
     * a value of type [`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
   * requires: `vonuvoli`


<a id='definition__r7rs__write-u8__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__write-u8__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__write-u8__description'></a>

#### Description

> ````
> (write-u8 byte)
> (write-u8 byte port)
> ````
> 
> 
> Writes the `byte` to
> the given binary output `port` and returns an unspecified value.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__write-u8__referenced-types'></a>

#### Referenced-types

 * [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
 * [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * [`binary-output-port-open`](../../r7rs/types/binary-output-port-open.md#type__r7rs__binary-output-port-open);


<a id='definition__r7rs__write-u8__categories'></a>

#### Categories

 * [`vs:ports:output`](../../r7rs/categories/vs_3a_ports_3a_output.md#category__r7rs__vs_3a_ports_3a_output);
 * [`vs:bytes`](../../r7rs/categories/vs_3a_bytes.md#category__r7rs__vs_3a_bytes);


<a id='definition__r7rs__write-u8__categories-recursive'></a>

#### Categories recursive

 * [`vs:ports`](../../r7rs/categories/vs_3a_ports.md#category__r7rs__vs_3a_ports);
 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

