

<a id='definition__r7rs__close-port'></a>

# `close-port` -- `r7rs` Definition


<a id='definition__r7rs__close-port__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__close-port__implemented-by'></a>

#### Implemented by

 * [`close-port`](../../vonuvoli/definitions/close-port.md#definition__vonuvoli__close-port) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__close-port__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((input-port-open) -> (undefined))`
   * input: a value of type [`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * `((input-port-closed) -> (undefined))`
   * input: a value of type [`input-port-closed`](../../r7rs/types/input-port-closed.md#type__r7rs__input-port-closed);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * `((output-port-open) -> (undefined))`
   * input: a value of type [`output-port-open`](../../r7rs/types/output-port-open.md#type__r7rs__output-port-open);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * `((output-port-closed) -> (undefined))`
   * input: a value of type [`output-port-closed`](../../r7rs/types/output-port-closed.md#type__r7rs__output-port-closed);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


<a id='definition__r7rs__close-port__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__close-port__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__close-port__description'></a>

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


<a id='definition__r7rs__close-port__referenced-types'></a>

#### Referenced-types

 * [`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open);
 * [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * [`input-port-closed`](../../r7rs/types/input-port-closed.md#type__r7rs__input-port-closed);
 * [`output-port-open`](../../r7rs/types/output-port-open.md#type__r7rs__output-port-open);
 * [`output-port-closed`](../../r7rs/types/output-port-closed.md#type__r7rs__output-port-closed);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

