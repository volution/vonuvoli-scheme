

<a id='definition__r7rs__open-input-bytevector'></a>

# `open-input-bytevector` -- `r7rs` Definitions


<a id='definition__r7rs__open-input-bytevector__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__open-input-bytevector__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((bytevector) -> (bytevector-input-port))`
   * input: a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
   * output: a value of type [`bytevector-input-port`](../../r7rs/types/bytevector-input-port.md#type__r7rs__bytevector-input-port);


<a id='definition__r7rs__open-input-bytevector__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base);


<a id='definition__r7rs__open-input-bytevector__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__open-input-bytevector__description'></a>

#### Description

> ````
> (open-input-bytevector bytevector)
> ````
> 
> 
> Takes a bytevector and returns a binary input port that delivers
> bytes from the bytevector.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__open-input-bytevector__referenced-types'></a>

#### Referenced-types

 * [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
 * [`bytevector-input-port`](../../r7rs/types/bytevector-input-port.md#type__r7rs__bytevector-input-port);


<a id='definition__r7rs__open-input-bytevector__categories'></a>

#### Categories

 * [`vs:ports:input`](../../r7rs/categories/vs_3a_ports_3a_input.md#category__r7rs__vs_3a_ports_3a_input);
 * [`vs:ports:open`](../../r7rs/categories/vs_3a_ports_3a_open.md#category__r7rs__vs_3a_ports_3a_open);
 * [`vs:bytes`](../../r7rs/categories/vs_3a_bytes.md#category__r7rs__vs_3a_bytes);


<a id='definition__r7rs__open-input-bytevector__categories-recursive'></a>

#### Categories recursive

 * [`vs:ports`](../../r7rs/categories/vs_3a_ports.md#category__r7rs__vs_3a_ports);
 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

