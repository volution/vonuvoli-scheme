

<a id='definition__r7rs__bytevector-append'></a>

# `bytevector-append` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|bytevector-empty|))`
   * inputs: none;
   * output: a value of type [`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
 * `((|bytevector| |...|) |->| (|bytevector|))`
   * inputs:
     * a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * `...` (i.e. variadic);
   * output: a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);


#### Referenced types

[`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
[`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);


#### Description

> ````
> (bytevector-append bytevector ...)
> ````
> 
> 
> Returns a newly allocated bytevector whose elements are the concatenation
> of the elements in the given bytevectors.
> 
> ````
> (bytevector-append #u8(0 1 2) #u8(3 4 5)) ===> #u8(0 1 2 3 4 5)
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

