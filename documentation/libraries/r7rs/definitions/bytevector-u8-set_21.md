

<a id='definition__r7rs__bytevector-u8-set_21'></a>

# `bytevector-u8-set!` -- `r7rs` Definitions


#### Kind

`mutator!`;


#### Procedure signature

Procedure variants:
 * `((|bytevector| |range-offset| |byte|) |->| (|undefined|))`
   * inputs:
     * a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
     * a value of type [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
   * requires: `(|not| |vonuvoli|)`
 * `((|bytevector| |range-offset| |byte|) |->| (|any|))`
   * inputs:
     * a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
     * a value of type [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * requires: `|vonuvoli|`


#### Referenced types

[`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
[`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
[`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
[`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
[`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Description

> ````
> (bytevector-u8-set! bytevector k byte)
> ````
> 
> 
> **Domain**:  It is an error if `k` is not a valid index of `bytevector`.
> 
> Stores `byte` as the `k`th byte of `bytevector`.
> ````
> (let ((bv (bytevector 1 2 3 4)))
>   (bytevector-u8-set! bv 1 3)
>   bv) ===> #u8(1 3 3 4)
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

