

<a id='definition__r7rs__even_3f'></a>

# `even?` -- `r7rs` Definitions


#### Kind

`predicate`;


#### Procedure signature

Procedure variants:
 * `((|number-zero|) |->| (|true|))`
   * input: a value of type [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|number-even|) |->| (|true|))`
   * input: a value of type [`number-even`](../../r7rs/types/number-even.md#type__r7rs__number-even);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|number-odd|) |->| (|false|))`
   * input: a value of type [`number-odd`](../../r7rs/types/number-odd.md#type__r7rs__number-odd);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
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
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`number-even`](../../r7rs/types/number-even.md#type__r7rs__number-even);
[`number-odd`](../../r7rs/types/number-odd.md#type__r7rs__number-odd);
[`false`](../../r7rs/types/false.md#type__r7rs__false);
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

