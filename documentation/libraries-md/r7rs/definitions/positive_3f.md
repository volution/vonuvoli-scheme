

<a id='definition__r7rs__positive_3f'></a>

# `positive?` -- `r7rs` Definition


<a id='definition__r7rs__positive_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__r7rs__positive_3f__extended-by'></a>

#### Extended by

 * [`positive?`](../../vonuvoli/definitions/positive_3f.md#definition__vonuvoli__positive_3f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__positive_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((real-zero) -> (false))`
   * input: a value of type [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((real-positive) -> (true))`
   * input: a value of type [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((real-negative) -> (false))`
   * input: a value of type [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((real) -> (false))`
   * input: a value of type [`real`](../../r7rs/types/real.md#type__r7rs__real);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__positive_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__positive_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__positive_3f__description'></a>

#### Description

> Please refer to [`zero?`](../../r7rs/definitions/zero_3f.md#definition__r7rs__zero_3f).


<a id='definition__r7rs__positive_3f__referenced-types'></a>

#### Referenced-types

 * [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
 * [`real`](../../r7rs/types/real.md#type__r7rs__real);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

