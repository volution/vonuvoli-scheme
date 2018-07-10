

<a id='definition__r7rs__make-bytevector'></a>

# `make-bytevector` -- `r7rs` Definitions


#### Kind

`constructor`;


#### Procedure signature

Procedure variants:
 * `((|range-length-zero|) |->| (|bytevector-empty|))`
   * input: a value of type [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
   * output: a value of type [`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
 * `((|range-length-zero| |byte|) |->| (|bytevector-empty|))`
   * inputs:
     * a value of type [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
     * a value of type [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
   * output: a value of type [`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
 * `((|range-length-not-zero|) |->| (|bytevector-not-empty|))`
   * input: a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
   * output: a value of type [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
 * `((|range-length-not-zero| |byte|) |->| (|bytevector-not-empty|))`
   * inputs:
     * a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
     * a value of type [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
   * output: a value of type [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);


#### Referenced types

[`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
[`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
[`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
[`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
[`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);


#### Description

> ````
> (make-bytevector k)
> (make-bytevector k byte)
> ````
> 
> 
> The `make-bytevector` procedure returns a newly allocated bytevector of
> length `k`.  If `byte` is given, then all elements of the bytevector
> are initialized to `byte`, otherwise the contents of each
> element are unspecified.
> 
> ````
> (make-bytevector 2 12)  ===>  #u8(12 12)
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

