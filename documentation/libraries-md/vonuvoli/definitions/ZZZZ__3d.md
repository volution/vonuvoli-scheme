

<a id='definition__vonuvoli__ZZZZ__3d'></a>

# `=` -- `vonuvoli` Definition


<a id='definition__vonuvoli__ZZZZ__3d__kind'></a>

#### Kind

`comparator`;


<a id='definition__vonuvoli__ZZZZ__3d__extends'></a>

#### Extends

 * [`=`](../../r7rs/definitions/ZZZZ__3d.md#definition__r7rs__ZZZZ__3d) (from [`r7rs`](../../r7rs/_index.md#library__r7rs));


<a id='definition__vonuvoli__ZZZZ__3d__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number-not-nan) -> (true))`
   * input: a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((number-not-nan |2...|) -> (boolean))`
   * inputs:
     * a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
     * `...` -- at least 2 times;
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
 * `((number |1...|) -> (false))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__vonuvoli__ZZZZ__3d__exports'></a>

#### Exports

 * [`vs:comparisons`](../../vonuvoli/exports/vs_3a_comparisons.md#export__vonuvoli__vs_3a_comparisons) -- `(vonuvoli comparisons positive)`;
 * [`vs:arithmetic`](../../vonuvoli/exports/vs_3a_arithmetic.md#export__vonuvoli__vs_3a_arithmetic) -- `(vonuvoli arithmetic)`;


<a id='definition__vonuvoli__ZZZZ__3d__exports-recursive'></a>

#### Exports recursive

 * [`vs:comparisons*`](../../vonuvoli/exports/vs_3a_comparisons_2a.md#export__vonuvoli__vs_3a_comparisons_2a) -- `(vonuvoli comparisons)`;
 * [`vs:base`](../../vonuvoli/exports/vs_3a_base.md#export__vonuvoli__vs_3a_base) -- `(vonuvoli base)`;


<a id='definition__vonuvoli__ZZZZ__3d__referenced-types'></a>

#### Referenced-types

 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__vonuvoli__ZZZZ__3d__categories'></a>

#### Categories

 * [`vs:r7rs`](../../vonuvoli/categories/vs_3a_r7rs.md#category__vonuvoli__vs_3a_r7rs);
 * [`vs:arithmetic`](../../vonuvoli/categories/vs_3a_arithmetic.md#category__vonuvoli__vs_3a_arithmetic);
 * [`vs:comparisons`](../../vonuvoli/categories/vs_3a_comparisons.md#category__vonuvoli__vs_3a_comparisons);
 * [`vs:equivalence`](../../vonuvoli/categories/vs_3a_equivalence.md#category__vonuvoli__vs_3a_equivalence);

----

Goto: [library](../../vonuvoli/_index.md#library__vonuvoli), [categories](../../vonuvoli/categories/_index.md#toc__vonuvoli__categories), [exports](../../vonuvoli/exports/_index.md#toc__vonuvoli__exports), [definitions](../../vonuvoli/definitions/_index.md#toc__vonuvoli__definitions), other [libraries](../../_libraries.md#toc__libraries).

----

