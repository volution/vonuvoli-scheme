

<a id='definition__vonuvoli__odd_3f'></a>

# `odd?` -- `vonuvoli` Definition


<a id='definition__vonuvoli__odd_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__vonuvoli__odd_3f__extends'></a>

#### Extends

 * [`odd?`](../../r7rs/definitions/odd_3f.md#definition__r7rs__odd_3f) (from [`r7rs`](../../r7rs/_index.md#library__r7rs));


<a id='definition__vonuvoli__odd_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((integer-zero |1...|) -> (false))`
   * inputs:
     * a value of type [`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((integer-even |1...|) -> (false))`
   * inputs:
     * a value of type [`integer-even`](../../r7rs/types/integer-even.md#type__r7rs__integer-even);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((integer-odd |1...|) -> (true))`
   * inputs:
     * a value of type [`integer-odd`](../../r7rs/types/integer-odd.md#type__r7rs__integer-odd);
     * `...` -- at least one time;
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((integer |1...|) -> (false))`
   * inputs:
     * a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__vonuvoli__odd_3f__exports'></a>

#### Exports

 * [`vs:arithmetic`](../../vonuvoli/exports/vs_3a_arithmetic.md#export__vonuvoli__vs_3a_arithmetic) -- `(vonuvoli arithmetic)`;


<a id='definition__vonuvoli__odd_3f__exports-recursive'></a>

#### Exports recursive

 * [`vs:base`](../../vonuvoli/exports/vs_3a_base.md#export__vonuvoli__vs_3a_base) -- `(vonuvoli base)`;


<a id='definition__vonuvoli__odd_3f__referenced-types'></a>

#### Referenced-types

 * [`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`integer-even`](../../r7rs/types/integer-even.md#type__r7rs__integer-even);
 * [`integer-odd`](../../r7rs/types/integer-odd.md#type__r7rs__integer-odd);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);


<a id='definition__vonuvoli__odd_3f__categories'></a>

#### Categories

 * [`vs:r7rs`](../../vonuvoli/categories/vs_3a_r7rs.md#category__vonuvoli__vs_3a_r7rs);
 * [`vs:arithmetic`](../../vonuvoli/categories/vs_3a_arithmetic.md#category__vonuvoli__vs_3a_arithmetic);

----

Goto: [library](../../vonuvoli/_index.md#library__vonuvoli), [categories](../../vonuvoli/categories/_index.md#toc__vonuvoli__categories), [exports](../../vonuvoli/exports/_index.md#toc__vonuvoli__exports), [definitions](../../vonuvoli/definitions/_index.md#toc__vonuvoli__definitions), other [libraries](../../_libraries.md#toc__libraries).

----

