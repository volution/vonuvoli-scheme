

<a id='definition__r7rs__lcm'></a>

# `lcm` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|number-not-nan|) |->| (|number-not-nan|))`
   * input: a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
   * output: a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * `((|number-not-nan| |...|) |->| (|number-not-nan|))`
   * inputs:
     * a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
     * `...` (i.e. variadic);
   * output: a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
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

> Please refer to [`gcd`](../../r7rs/definitions/gcd.md#definition__r7rs__gcd).


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

