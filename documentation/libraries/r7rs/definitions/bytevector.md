

<a id='definition__r7rs__bytevector'></a>

# `bytevector` -- `r7rs` Definitions


#### Kind

`constructor`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|bytevector-empty|))`
   * inputs: none;
   * output: a value of type [`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
 * `((|byte| |...|) |->| (|bytevector-not-empty|))`
   * inputs:
     * a value of type [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
     * `...` (i.e. variadic);
   * output: a value of type [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);


#### Referenced types

[`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
[`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
[`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);


#### Description

> ````
> (bytevector byte ...)
> ````
> 
> 
> Returns a newly allocated bytevector containing its arguments.
> 
> ````
> (bytevector 1 3 5 1 3 5)         ===>  #u8(1 3 5 1 3 5)
> (bytevector)                     ===>  #u8()
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

