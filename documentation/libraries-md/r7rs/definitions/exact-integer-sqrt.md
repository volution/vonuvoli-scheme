

<a id='definition__r7rs__exact-integer-sqrt'></a>

# `exact-integer-sqrt` -- `r7rs` Definition


<a id='definition__r7rs__exact-integer-sqrt__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__exact-integer-sqrt__implemented-by'></a>

#### Implemented by

 * [`exact-integer-sqrt`](../../vonuvoli/definitions/exact-integer-sqrt.md#definition__vonuvoli__exact-integer-sqrt) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__exact-integer-sqrt__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((exact-integer-zero) -> (exact-integer-zero exact-integer-zero))`
   * input: a value of type [`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero);
   * outputs:
     * a value of type [`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero);
     * a value of type [`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero);
 * `((exact-integer-positive) -> (exact-integer-positive exact-integer-positive-or-zero))`
   * input: a value of type [`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
   * outputs:
     * a value of type [`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
     * a value of type [`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero);


<a id='definition__r7rs__exact-integer-sqrt__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__exact-integer-sqrt__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__exact-integer-sqrt__description'></a>

#### Description

> ````
> (exact-integer-sqrt k)
> ````
> 
> 
> Returns two non-negative exact integers `s` and `r` where
> `k = s^2 + r` and `k < (s+1)^2`.
> 
> ````
> (exact-integer-sqrt 4)  ===>  2 0
> (exact-integer-sqrt 5)  ===>  2 1
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__exact-integer-sqrt__referenced-types'></a>

#### Referenced-types

 * [`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero);
 * [`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
 * [`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

