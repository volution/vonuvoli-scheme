

<a id='definition__r7rs__flush-output-port'></a>

# `flush-output-port` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|void|))`
   * inputs: none;
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `((|output-port-open|) |->| (|void|))`
   * input: a value of type [`output-port-open`](../../r7rs/types/output-port-open.md#type__r7rs__output-port-open);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);


#### Referenced types

[`void`](../../r7rs/types/void.md#type__r7rs__void);
[`output-port-open`](../../r7rs/types/output-port-open.md#type__r7rs__output-port-open);


#### Description

> ````
> (flush-output-port)
> (flush-output-port port)
> ````
> 
> 
> Flushes any buffered output from the buffer of output-port to the
> underlying file or device and returns an unspecified value.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:ports:output`](../../r7rs/categories/vs_3a_ports_3a_output.md#category__r7rs__vs_3a_ports_3a_output);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

