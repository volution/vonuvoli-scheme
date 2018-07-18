

<a id='definition__r7rs__get-output-bytevector'></a>

# `get-output-bytevector` -- `r7rs` Definition


<a id='definition__r7rs__get-output-bytevector__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__get-output-bytevector__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((bytevector-output-port) -> (bytevector))`
   * input: a value of type [`bytevector-output-port`](../../r7rs/types/bytevector-output-port.md#type__r7rs__bytevector-output-port);
   * output: a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);


<a id='definition__r7rs__get-output-bytevector__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__get-output-bytevector__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__get-output-bytevector__description'></a>

#### Description

> ````
> (get-output-bytevector port)
> ````
> 
> 
> **Domain**:  It is an error if `port` was not created with
> `open-output-bytevector`.
> 
> Returns a bytevector consisting
> of the bytes that have been output to the port so far in the
> order they were output.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__get-output-bytevector__referenced-types'></a>

#### Referenced-types

 * [`bytevector-output-port`](../../r7rs/types/bytevector-output-port.md#type__r7rs__bytevector-output-port);
 * [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);


<a id='definition__r7rs__get-output-bytevector__categories'></a>

#### Categories

 * [`vs:ports:output`](../../r7rs/categories/vs_3a_ports_3a_output.md#category__r7rs__vs_3a_ports_3a_output);
 * [`vs:bytes`](../../r7rs/categories/vs_3a_bytes.md#category__r7rs__vs_3a_bytes);


<a id='definition__r7rs__get-output-bytevector__categories-recursive'></a>

#### Categories recursive

 * [`vs:ports`](../../r7rs/categories/vs_3a_ports.md#category__r7rs__vs_3a_ports);
 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

