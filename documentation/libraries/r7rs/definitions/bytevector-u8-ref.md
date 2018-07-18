

<a id='definition__r7rs__bytevector-u8-ref'></a>

# `bytevector-u8-ref` -- `r7rs` Definition


<a id='definition__r7rs__bytevector-u8-ref__kind'></a>

#### Kind

`accessor`;


<a id='definition__r7rs__bytevector-u8-ref__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((bytevector range-offset) -> (byte))`
   * inputs:
     * a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
   * output: a value of type [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);


<a id='definition__r7rs__bytevector-u8-ref__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__bytevector-u8-ref__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__bytevector-u8-ref__description'></a>

#### Description

> ````
> (bytevector-u8-ref bytevector k)
> ````
> 
> 
> **Domain**:  It is an error if `k` is not a valid index of `bytevector`.
> 
> Returns the `k`th byte of `bytevector`.
> 
> ````
> (bytevector-u8-ref '#u8(1 1 2 3 5 8 13 21) 5)  ===>  8
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__bytevector-u8-ref__referenced-types'></a>

#### Referenced-types

 * [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
 * [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
 * [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);


<a id='definition__r7rs__bytevector-u8-ref__categories'></a>

#### Categories

 * [`vs:bytes`](../../r7rs/categories/vs_3a_bytes.md#category__r7rs__vs_3a_bytes);


<a id='definition__r7rs__bytevector-u8-ref__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

