

<a id='definition__vonuvoli__integer_3f'></a>

# `integer?` -- `vonuvoli` Definition


<a id='definition__vonuvoli__integer_3f__kind'></a>

#### Kind

`type-predicate`;


<a id='definition__vonuvoli__integer_3f__extends'></a>

#### Extends

 * [`integer?`](../../r7rs/definitions/integer_3f.md#definition__r7rs__integer_3f) (from [`r7rs`](../../r7rs/_index.md#library__r7rs));


<a id='definition__vonuvoli__integer_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((integer |1...|) -> (true))`
   * inputs:
     * a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
     * `...` -- at least one time;
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((rational |1...|) -> (false))`
   * inputs:
     * a value of type [`rational`](../../r7rs/types/rational.md#type__r7rs__rational);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((real |1...|) -> (false))`
   * inputs:
     * a value of type [`real`](../../r7rs/types/real.md#type__r7rs__real);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((complex |1...|) -> (false))`
   * inputs:
     * a value of type [`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((number |1...|) -> (false))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((any |1...|) -> (false))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__vonuvoli__integer_3f__exports'></a>

#### Exports

 * [`vs:types`](../../vonuvoli/exports/vs_3a_types.md#export__vonuvoli__vs_3a_types) -- `(vonuvoli types positive)`;
 * [`vs:arithmetic`](../../vonuvoli/exports/vs_3a_arithmetic.md#export__vonuvoli__vs_3a_arithmetic) -- `(vonuvoli arithmetic)`;


<a id='definition__vonuvoli__integer_3f__exports-recursive'></a>

#### Exports recursive

 * [`vs:types*`](../../vonuvoli/exports/vs_3a_types_2a.md#export__vonuvoli__vs_3a_types_2a) -- `(vonuvoli types)`;
 * [`vs:base`](../../vonuvoli/exports/vs_3a_base.md#export__vonuvoli__vs_3a_base) -- `(vonuvoli base)`;


<a id='definition__vonuvoli__integer_3f__referenced-types'></a>

#### Referenced-types

 * [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`rational`](../../r7rs/types/rational.md#type__r7rs__rational);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`real`](../../r7rs/types/real.md#type__r7rs__real);
 * [`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__vonuvoli__integer_3f__categories'></a>

#### Categories

 * [`vs:r7rs`](../../vonuvoli/categories/vs_3a_r7rs.md#category__vonuvoli__vs_3a_r7rs);
 * [`vs:arithmetic`](../../vonuvoli/categories/vs_3a_arithmetic.md#category__vonuvoli__vs_3a_arithmetic);
 * [`vs:types`](../../vonuvoli/categories/vs_3a_types.md#category__vonuvoli__vs_3a_types);

----

Goto: [library](../../vonuvoli/_index.md#library__vonuvoli), [categories](../../vonuvoli/categories/_index.md#toc__vonuvoli__categories), [exports](../../vonuvoli/exports/_index.md#toc__vonuvoli__exports), [definitions](../../vonuvoli/definitions/_index.md#toc__vonuvoli__definitions), other [libraries](../../_libraries.md#toc__libraries).

----

