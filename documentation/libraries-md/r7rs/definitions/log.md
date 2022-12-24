

<a id='definition__r7rs__log'></a>

# `log` -- `r7rs` Definition


<a id='definition__r7rs__log__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__log__implemented-by'></a>

#### Implemented by

 * [`log`](../../vonuvoli/definitions/log.md#definition__vonuvoli__log) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__log__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((real-positive) -> (real-not-nan))`
   * input: a value of type [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
   * output: a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * `((real-negative) -> (complex-not-nan))`
   * input: a value of type [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
   * output: a value of type [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * `((complex-not-nan) -> (complex-not-nan))`
   * input: a value of type [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
   * output: a value of type [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * `((number) -> (number-nan))`
   * input: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
   * output: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
 * `((real-positive real-positive) -> (real-not-nan))`
   * inputs:
     * a value of type [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
     * a value of type [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
   * output: a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * `((real-positive real-negative) -> (complex-not-nan))`
   * inputs:
     * a value of type [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
     * a value of type [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
   * output: a value of type [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * `((real-negative real-not-nan) -> (complex-not-nan))`
   * inputs:
     * a value of type [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
     * a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
   * output: a value of type [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * `((complex-not-nan complex-not-nan) -> (complex-not-nan))`
   * inputs:
     * a value of type [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
     * a value of type [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
   * output: a value of type [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * `((number number) -> (number-nan))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
   * output: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


<a id='definition__r7rs__log__exports'></a>

#### Exports

 * [`scheme:inexact`](../../r7rs/exports/scheme_3a_inexact.md#export__r7rs__scheme_3a_inexact) -- `(scheme inexact)`;


<a id='definition__r7rs__log__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__log__description'></a>

#### Description

> Please refer to [`exp`](../../r7rs/definitions/exp.md#definition__r7rs__exp).


<a id='definition__r7rs__log__referenced-types'></a>

#### Referenced-types

 * [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
 * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
 * [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

