

<a id='definition__r7rs__open-input-bytevector'></a>

# `open-input-bytevector` -- `r7rs` Definition


<a id='definition__r7rs__open-input-bytevector__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__open-input-bytevector__implemented-by'></a>

#### Implemented by

 * [`open-input-bytevector`](../../vonuvoli/definitions/open-input-bytevector.md#definition__vonuvoli__open-input-bytevector) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__open-input-bytevector__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((bytevector) -> (bytevector-input-port))`
   * input: a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
   * output: a value of type [`bytevector-input-port`](../../r7rs/types/bytevector-input-port.md#type__r7rs__bytevector-input-port);


<a id='definition__r7rs__open-input-bytevector__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__open-input-bytevector__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


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

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

