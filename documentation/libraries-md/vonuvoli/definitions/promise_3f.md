

<a id='definition__vonuvoli__promise_3f'></a>

# `promise?` -- `vonuvoli` Definition


<a id='definition__vonuvoli__promise_3f__kind'></a>

#### Kind

`type-predicate`;


<a id='definition__vonuvoli__promise_3f__extends'></a>

#### Extends

 * [`promise?`](../../r7rs/definitions/promise_3f.md#definition__r7rs__promise_3f) (from [`r7rs`](../../r7rs/_index.md#library__r7rs));


<a id='definition__vonuvoli__promise_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((promise |1...|) -> (true))`
   * inputs:
     * a value of type [`promise`](../../r7rs/types/promise.md#type__r7rs__promise);
     * `...` -- at least one time;
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((any |1...|) -> (false))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__vonuvoli__promise_3f__exports'></a>

#### Exports

 * [`vs:types`](../../vonuvoli/exports/vs_3a_types.md#export__vonuvoli__vs_3a_types) -- `(vonuvoli types positive)`;
 * [`vs:promises`](../../vonuvoli/exports/vs_3a_promises.md#export__vonuvoli__vs_3a_promises) -- `(vonuvoli promises)`;


<a id='definition__vonuvoli__promise_3f__exports-recursive'></a>

#### Exports recursive

 * [`vs:types*`](../../vonuvoli/exports/vs_3a_types_2a.md#export__vonuvoli__vs_3a_types_2a) -- `(vonuvoli types)`;
 * [`vs:base`](../../vonuvoli/exports/vs_3a_base.md#export__vonuvoli__vs_3a_base) -- `(vonuvoli base)`;


<a id='definition__vonuvoli__promise_3f__referenced-types'></a>

#### Referenced-types

 * [`promise`](../../r7rs/types/promise.md#type__r7rs__promise);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__vonuvoli__promise_3f__categories'></a>

#### Categories

 * [`vs:r7rs`](../../vonuvoli/categories/vs_3a_r7rs.md#category__vonuvoli__vs_3a_r7rs);
 * [`vs:promises`](../../vonuvoli/categories/vs_3a_promises.md#category__vonuvoli__vs_3a_promises);
 * [`vs:evaluator`](../../vonuvoli/categories/vs_3a_evaluator.md#category__vonuvoli__vs_3a_evaluator);

----

Goto: [library](../../vonuvoli/_index.md#library__vonuvoli), [categories](../../vonuvoli/categories/_index.md#toc__vonuvoli__categories), [exports](../../vonuvoli/exports/_index.md#toc__vonuvoli__exports), [definitions](../../vonuvoli/definitions/_index.md#toc__vonuvoli__definitions), other [libraries](../../_libraries.md#toc__libraries).

----

