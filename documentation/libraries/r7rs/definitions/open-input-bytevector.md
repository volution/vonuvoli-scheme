

<a id='definition__r7rs__open-input-bytevector'></a>

# `open-input-bytevector` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|bytevector|) |->| (|bytevector-input-port|))`
   * input: a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
   * output: a value of type [`bytevector-input-port`](../../r7rs/types/bytevector-input-port.md#type__r7rs__bytevector-input-port);


#### Referenced types

[`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
[`bytevector-input-port`](../../r7rs/types/bytevector-input-port.md#type__r7rs__bytevector-input-port);


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


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:ports:input`](../../r7rs/categories/vs_3a_ports_3a_input.md#category__r7rs__vs_3a_ports_3a_input);
[`vs:ports:open`](../../r7rs/categories/vs_3a_ports_3a_open.md#category__r7rs__vs_3a_ports_3a_open);
[`vs:bytes`](../../r7rs/categories/vs_3a_bytes.md#category__r7rs__vs_3a_bytes);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

