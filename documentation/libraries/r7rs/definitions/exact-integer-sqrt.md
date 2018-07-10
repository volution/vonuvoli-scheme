

<a id='definition__r7rs__exact-integer-sqrt'></a>

# `exact-integer-sqrt` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|number-zero|) |->| (|number-zero| |number-zero|))`
   * input: a value of type [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
   * outputs:
     * a value of type [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
     * a value of type [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
 * `((|number-positive|) |->| (|number-positive| |number-positive-or-zero|))`
   * input: a value of type [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
   * outputs:
     * a value of type [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
     * a value of type [`number-positive-or-zero`](../../r7rs/types/number-positive-or-zero.md#type__r7rs__number-positive-or-zero);


#### Referenced types

[`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
[`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
[`number-positive-or-zero`](../../r7rs/types/number-positive-or-zero.md#type__r7rs__number-positive-or-zero);


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


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

