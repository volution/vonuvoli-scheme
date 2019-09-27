

<a id='definition__r7rs__current-input-port'></a>

# `current-input-port` -- `r7rs` Definition


<a id='definition__r7rs__current-input-port__kind'></a>

#### Kind

`parameter`;


<a id='definition__r7rs__current-input-port__implemented-by'></a>

#### Implemented by

 * [`current-input-port`](../../vonuvoli/definitions/current-input-port.md#definition__vonuvoli__current-input-port) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__current-input-port__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (input-port))`
   * inputs: none;
   * output: a value of type [`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port);


<a id='definition__r7rs__current-input-port__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__current-input-port__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__current-input-port__description'></a>

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


<a id='definition__r7rs__current-input-port__referenced-types'></a>

#### Referenced-types

 * [`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

