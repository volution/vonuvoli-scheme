

<a id='definition__r7rs__close-port'></a>

# `close-port` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|input-port-open|) |->| (|void|))`
   * input: a value of type [`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `((|input-port-closed|) |->| (|void|))`
   * input: a value of type [`input-port-closed`](../../r7rs/types/input-port-closed.md#type__r7rs__input-port-closed);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `((|output-port-open|) |->| (|void|))`
   * input: a value of type [`output-port-open`](../../r7rs/types/output-port-open.md#type__r7rs__output-port-open);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `((|output-port-closed|) |->| (|void|))`
   * input: a value of type [`output-port-closed`](../../r7rs/types/output-port-closed.md#type__r7rs__output-port-closed);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);


#### Referenced types

[`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open);
[`void`](../../r7rs/types/void.md#type__r7rs__void);
[`input-port-closed`](../../r7rs/types/input-port-closed.md#type__r7rs__input-port-closed);
[`output-port-open`](../../r7rs/types/output-port-open.md#type__r7rs__output-port-open);
[`output-port-closed`](../../r7rs/types/output-port-closed.md#type__r7rs__output-port-closed);


#### Description

> ````
> (close-port port)
> (close-input-port port)
> (close-output-port port)
> ````
> 
> 
> Closes the resource associated with `port`, rendering the `port`
> incapable of delivering or accepting data.
> It is an error
> to apply the last two procedures to a port which is not an input
> or output port, respectively.
> Scheme implementations may provide ports which are simultaneously
> input and output ports, such as sockets; the `close-input-port`
> and `close-output-port` procedures can then be used to close the
> input and output sides of the port independently.
> 
> These routines have no effect if the port has already been closed.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:ports`](../../r7rs/categories/vs_3a_ports.md#category__r7rs__vs_3a_ports);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

