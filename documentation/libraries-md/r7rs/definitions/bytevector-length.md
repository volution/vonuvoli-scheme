

<a id='definition__r7rs__bytevector-length'></a>

# `bytevector-length` -- `r7rs` Definition


<a id='definition__r7rs__bytevector-length__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__bytevector-length__implemented-by'></a>

#### Implemented by

 * [`bytevector-length`](../../vonuvoli/definitions/bytevector-length.md#definition__vonuvoli__bytevector-length) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__bytevector-length__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((bytevector-empty) -> (range-length-zero))`
   * input: a value of type [`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
   * output: a value of type [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
 * `((bytevector-not-empty) -> (range-length-not-zero))`
   * input: a value of type [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
   * output: a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);


<a id='definition__r7rs__bytevector-length__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__bytevector-length__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__bytevector-length__description'></a>

#### Description

> ````
> (bytevector-length bytevector)
> ````
> 
> 
> Returns the length of `bytevector` in bytes as an exact integer.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__bytevector-length__referenced-types'></a>

#### Referenced-types

 * [`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
 * [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
 * [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
 * [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

