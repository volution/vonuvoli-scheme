

<a id='definition__r7rs__negative_3f'></a>

# `negative?` -- `r7rs` Definitions


#### Kind

`predicate`;


#### Procedure signature

Procedure variants:
 * `((|number-zero|) |->| (|false|))`
   * input: a value of type [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|number-positive|) |->| (|false|))`
   * input: a value of type [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|number-negative|) |->| (|true|))`
   * input: a value of type [`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|number|) |->| (|false|))`
   * input: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|number| |...|) |->| (|boolean|))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `|vonuvoli|`


#### Referenced types

[`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
[`false`](../../r7rs/types/false.md#type__r7rs__false);
[`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
[`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Description

> Please refer to [`zero?`](../../r7rs/definitions/zero_3f.md#definition__r7rs__zero_3f).


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

