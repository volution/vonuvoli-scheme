

<a id='definition__vonuvoli__exact_3f'></a>

# `exact?` -- `vonuvoli` Definition


<a id='definition__vonuvoli__exact_3f__kind'></a>

#### Kind

`type-predicate`;


<a id='definition__vonuvoli__exact_3f__extends'></a>

#### Extends

 * [`exact?`](../../r7rs/definitions/exact_3f.md#definition__r7rs__exact_3f) (from [`r7rs`](../../r7rs/_index.md#library__r7rs));


<a id='definition__vonuvoli__exact_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((exact-number |1...|) -> (true))`
   * inputs:
     * a value of type [`exact-number`](../../r7rs/types/exact-number.md#type__r7rs__exact-number);
     * `...` -- at least one time;
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((inexact-number |1...|) -> (false))`
   * inputs:
     * a value of type [`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((number |1...|) -> (false))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__vonuvoli__exact_3f__exports'></a>

#### Exports

 * [`vs:arithmetic`](../../vonuvoli/exports/vs_3a_arithmetic.md#export__vonuvoli__vs_3a_arithmetic) -- `(vonuvoli arithmetic)`;


<a id='definition__vonuvoli__exact_3f__exports-recursive'></a>

#### Exports recursive

 * [`vs:base`](../../vonuvoli/exports/vs_3a_base.md#export__vonuvoli__vs_3a_base) -- `(vonuvoli base)`;


<a id='definition__vonuvoli__exact_3f__referenced-types'></a>

#### Referenced-types

 * [`exact-number`](../../r7rs/types/exact-number.md#type__r7rs__exact-number);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);


<a id='definition__vonuvoli__exact_3f__categories'></a>

#### Categories

 * [`vs:r7rs`](../../vonuvoli/categories/vs_3a_r7rs.md#category__vonuvoli__vs_3a_r7rs);
 * [`vs:arithmetic`](../../vonuvoli/categories/vs_3a_arithmetic.md#category__vonuvoli__vs_3a_arithmetic);
 * [`vs:types`](../../vonuvoli/categories/vs_3a_types.md#category__vonuvoli__vs_3a_types);

----

Goto: [library](../../vonuvoli/_index.md#library__vonuvoli), [categories](../../vonuvoli/categories/_index.md#toc__vonuvoli__categories), [exports](../../vonuvoli/exports/_index.md#toc__vonuvoli__exports), [definitions](../../vonuvoli/definitions/_index.md#toc__vonuvoli__definitions), other [libraries](../../_libraries.md#toc__libraries).

----

