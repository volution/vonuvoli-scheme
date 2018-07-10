

<a id='definition__r7rs__current-input-port'></a>

# `current-input-port` -- `r7rs` Definitions


#### Kind

`parameter`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|input-port|))`
   * inputs: none;
   * output: a value of type [`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port);


#### Referenced types

[`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port);


#### Description

> ````
> (current-input-port)
> (current-output-port)
> (current-error-port)
> ````
> 
> 
> Returns the current default input port, output port, or error port (an
> output port), respectively.  These procedures are parameter objects, which can be
> overridden with `parameterize` (see
> section on `make-parameter`).  The initial bindings for these
> are implementation-defined textual ports.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:parameters`](../../r7rs/categories/vs_3a_parameters.md#category__r7rs__vs_3a_parameters);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

