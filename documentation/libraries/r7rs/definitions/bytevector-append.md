

<a id='definition__r7rs__bytevector-append'></a>

# `bytevector-append` -- `r7rs` Definition


<a id='definition__r7rs__bytevector-append__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__bytevector-append__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (bytevector-empty))`
   * inputs: none;
   * output: a value of type [`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
 * `((bytevector ...) -> (bytevector))`
   * inputs:
     * a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * `...` (i.e. variadic);
   * output: a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);


<a id='definition__r7rs__bytevector-append__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__bytevector-append__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__bytevector-append__description'></a>

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


<a id='definition__r7rs__bytevector-append__referenced-types'></a>

#### Referenced-types

 * [`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
 * [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);


<a id='definition__r7rs__bytevector-append__categories'></a>

#### Categories

 * [`vs:bytes`](../../vonuvoli/categories/vs_3a_bytes.md#category__vonuvoli__vs_3a_bytes);


<a id='definition__r7rs__bytevector-append__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

