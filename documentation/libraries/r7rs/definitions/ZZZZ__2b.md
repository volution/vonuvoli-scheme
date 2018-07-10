

<a id='definition__r7rs__ZZZZ__2b'></a>

# `+` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|number-not-nan|) |->| (|number-not-nan|))`
   * input: a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
   * output: a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * `((|number-not-nan| |...|) |->| (|number|))`
   * inputs:
     * a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
     * `...` (i.e. variadic);
   * output: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * `((|number| |...|) |->| (|number-nan|))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * `...` (i.e. variadic);
   * output: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


#### Referenced types

[`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


#### Description

> ````
> (+ z_1 ...)
> (* z_1 ...)
> ````
> 
> 
> These procedures return the sum or product of their arguments.
> 
> ````
> (+ 3 4)                 ===>  7
> (+ 3)                   ===>  3
> (+)                     ===>  0
> (* 4)                   ===>  4
> (*)                     ===>  1
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

