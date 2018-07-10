

<a id='definition__r7rs__integer_3f'></a>

# `integer?` -- `r7rs` Definitions


#### Kind

`type-predicate`;


#### Procedure signature

Procedure variants:
 * `((|integer|) |->| (|true|))`
   * input: a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|rational|) |->| (|false|))`
   * input: a value of type [`rational`](../../r7rs/types/rational.md#type__r7rs__rational);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|real|) |->| (|false|))`
   * input: a value of type [`real`](../../r7rs/types/real.md#type__r7rs__real);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|complex|) |->| (|false|))`
   * input: a value of type [`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|number|) |->| (|false|))`
   * input: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|any|) |->| (|false|))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|any| |...|) |->| (|boolean|))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `|vonuvoli|`


#### Referenced types

[`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`rational`](../../r7rs/types/rational.md#type__r7rs__rational);
[`false`](../../r7rs/types/false.md#type__r7rs__false);
[`real`](../../r7rs/types/real.md#type__r7rs__real);
[`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Description

> Please refer to [`number?`](../../r7rs/definitions/number_3f.md#definition__r7rs__number_3f).


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);
[`vs:types`](../../r7rs/categories/vs_3a_types.md#category__r7rs__vs_3a_types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

