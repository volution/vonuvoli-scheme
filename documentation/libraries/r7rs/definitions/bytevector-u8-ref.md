

<a id='definition__r7rs__bytevector-u8-ref'></a>

# `bytevector-u8-ref` -- `r7rs` Definitions


#### Kind

`accessor`;


#### Procedure signature

Procedure variants:
 * `((|bytevector| |range-offset|) |->| (|byte|))`
   * inputs:
     * a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
   * output: a value of type [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);


#### Referenced types

[`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
[`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
[`byte`](../../r7rs/types/byte.md#type__r7rs__byte);


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


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:bytes`](../../r7rs/categories/vs_3a_bytes.md#category__r7rs__vs_3a_bytes);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

