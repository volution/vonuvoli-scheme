

<a id='definition__vonuvoli__positive_3f'></a>

# `positive?` -- `vonuvoli` Definition


<a id='definition__vonuvoli__positive_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__vonuvoli__positive_3f__extends'></a>

#### Extends

 * [`positive?`](../../r7rs/definitions/positive_3f.md#definition__r7rs__positive_3f) (from [`r7rs`](../../r7rs/_index.md#library__r7rs));


<a id='definition__vonuvoli__positive_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((real-zero |1...|) -> (false))`
   * inputs:
     * a value of type [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((real-positive |1...|) -> (true))`
   * inputs:
     * a value of type [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
     * `...` -- at least one time;
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((real-negative |1...|) -> (false))`
   * inputs:
     * a value of type [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((real |1...|) -> (false))`
   * inputs:
     * a value of type [`real`](../../r7rs/types/real.md#type__r7rs__real);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__vonuvoli__positive_3f__exports'></a>

#### Exports

 * [`vs:arithmetic`](../../vonuvoli/exports/vs_3a_arithmetic.md#export__vonuvoli__vs_3a_arithmetic) -- `(vonuvoli arithmetic)`;


<a id='definition__vonuvoli__positive_3f__exports-recursive'></a>

#### Exports recursive

 * [`vs:base`](../../vonuvoli/exports/vs_3a_base.md#export__vonuvoli__vs_3a_base) -- `(vonuvoli base)`;


<a id='definition__vonuvoli__positive_3f__referenced-types'></a>

#### Referenced-types

 * [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
 * [`real`](../../r7rs/types/real.md#type__r7rs__real);


<a id='definition__vonuvoli__positive_3f__categories'></a>

#### Categories

 * [`vs:r7rs`](../../vonuvoli/categories/vs_3a_r7rs.md#category__vonuvoli__vs_3a_r7rs);
 * [`vs:arithmetic`](../../vonuvoli/categories/vs_3a_arithmetic.md#category__vonuvoli__vs_3a_arithmetic);

----

Goto: [library](../../vonuvoli/_index.md#library__vonuvoli), [categories](../../vonuvoli/categories/_index.md#toc__vonuvoli__categories), [exports](../../vonuvoli/exports/_index.md#toc__vonuvoli__exports), [definitions](../../vonuvoli/definitions/_index.md#toc__vonuvoli__definitions), other [libraries](../../_libraries.md#toc__libraries).

----

