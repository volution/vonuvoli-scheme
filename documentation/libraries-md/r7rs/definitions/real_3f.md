

<a id='definition__r7rs__real_3f'></a>

# `real?` -- `r7rs` Definition


<a id='definition__r7rs__real_3f__kind'></a>

#### Kind

`type-predicate`;


<a id='definition__r7rs__real_3f__extended-by'></a>

#### Extended by

 * [`real?`](../../vonuvoli/definitions/real_3f.md#definition__vonuvoli__real_3f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__real_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((integer) -> (true))`
   * input: a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((rational) -> (true))`
   * input: a value of type [`rational`](../../r7rs/types/rational.md#type__r7rs__rational);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((real) -> (true))`
   * input: a value of type [`real`](../../r7rs/types/real.md#type__r7rs__real);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((complex) -> (false))`
   * input: a value of type [`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((number) -> (false))`
   * input: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((any) -> (false))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__real_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__real_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__real_3f__description'></a>

#### Description

> Please refer to [`number?`](../../r7rs/definitions/number_3f.md#definition__r7rs__number_3f).


<a id='definition__r7rs__real_3f__referenced-types'></a>

#### Referenced-types

 * [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`rational`](../../r7rs/types/rational.md#type__r7rs__rational);
 * [`real`](../../r7rs/types/real.md#type__r7rs__real);
 * [`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

