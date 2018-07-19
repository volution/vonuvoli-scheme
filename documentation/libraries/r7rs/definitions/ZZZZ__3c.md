

<a id='definition__r7rs__ZZZZ__3c'></a>

# `<` -- `r7rs` Definition


<a id='definition__r7rs__ZZZZ__3c__kind'></a>

#### Kind

`comparator`;


<a id='definition__r7rs__ZZZZ__3c__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number-not-nan) -> (true))`
   * input: a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((number-not-nan ...) -> (boolean))`
   * inputs:
     * a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
 * `((number ...) -> (false))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * `...` (i.e. variadic);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__ZZZZ__3c__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__ZZZZ__3c__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__ZZZZ__3c__description'></a>

#### Description

> Please refer to [`=`](../../r7rs/definitions/ZZZZ__3d.md#definition__r7rs__ZZZZ__3d).


<a id='definition__r7rs__ZZZZ__3c__referenced-types'></a>

#### Referenced-types

 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__ZZZZ__3c__categories'></a>

#### Categories

 * [`vs:arithmetic`](../../vonuvoli/categories/vs_3a_arithmetic.md#category__vonuvoli__vs_3a_arithmetic);
 * [`vs:comparisons`](../../vonuvoli/categories/vs_3a_comparisons.md#category__vonuvoli__vs_3a_comparisons);


<a id='definition__r7rs__ZZZZ__3c__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

