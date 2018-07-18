

<a id='definition__r7rs__bytevector-copy'></a>

# `bytevector-copy` -- `r7rs` Definition


<a id='definition__r7rs__bytevector-copy__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__bytevector-copy__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((bytevector) -> (bytevector))`
   * input: a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
   * output: a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
 * `((bytevector range-start) -> (bytevector))`
   * inputs:
     * a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
 * `((bytevector range-start range-end) -> (bytevector))`
   * inputs:
     * a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);


<a id='definition__r7rs__bytevector-copy__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__bytevector-copy__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__bytevector-copy__description'></a>

#### Description

> ````
> (bytevector-copy bytevector)
> (bytevector-copy bytevector start)
> (bytevector-copy bytevector start end)
> ````
> 
> 
> Returns a newly allocated bytevector containing the bytes in `bytevector`
> between `start` and `end`.
> 
> ````
> (define a #u8(1 2 3 4 5))
> (bytevector-copy a 2 4)) ===> #u8(3 4)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__bytevector-copy__referenced-types'></a>

#### Referenced-types

 * [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
 * [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
 * [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);


<a id='definition__r7rs__bytevector-copy__categories'></a>

#### Categories

 * [`vs:bytes`](../../r7rs/categories/vs_3a_bytes.md#category__r7rs__vs_3a_bytes);


<a id='definition__r7rs__bytevector-copy__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

