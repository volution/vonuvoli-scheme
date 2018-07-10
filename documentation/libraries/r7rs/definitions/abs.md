

<a id='definition__r7rs__abs'></a>

# `abs` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|number-zero|) |->| (|number-zero|))`
   * input: a value of type [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
   * output: a value of type [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
 * `((|number-positive|) |->| (|number-positive|))`
   * input: a value of type [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
   * output: a value of type [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
 * `((|number-negative|) |->| (|number-positive|))`
   * input: a value of type [`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative);
   * output: a value of type [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
 * `((|number-nan|) |->| (|number-nan|))`
   * input: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
   * output: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


#### Referenced types

[`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
[`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
[`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative);
[`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


#### Description

> ````
> (abs x)
> ````
> 
> 
> The `abs` procedure returns the absolute value of its argument.
> ````
> (abs -7)                ===>  7
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

