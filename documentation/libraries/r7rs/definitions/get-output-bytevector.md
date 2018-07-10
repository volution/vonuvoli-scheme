

<a id='definition__r7rs__get-output-bytevector'></a>

# `get-output-bytevector` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|bytevector-output-port|) |->| (|bytevector|))`
   * input: a value of type [`bytevector-output-port`](../../r7rs/types/bytevector-output-port.md#type__r7rs__bytevector-output-port);
   * output: a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);


#### Referenced types

[`bytevector-output-port`](../../r7rs/types/bytevector-output-port.md#type__r7rs__bytevector-output-port);
[`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);


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


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:ports:output`](../../r7rs/categories/vs_3a_ports_3a_output.md#category__r7rs__vs_3a_ports_3a_output);
[`vs:bytes`](../../r7rs/categories/vs_3a_bytes.md#category__r7rs__vs_3a_bytes);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

