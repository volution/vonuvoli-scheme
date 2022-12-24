

<a id='definition__r7rs__ceiling'></a>

# `ceiling` -- `r7rs` Definition


<a id='definition__r7rs__ceiling__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__ceiling__implemented-by'></a>

#### Implemented by

 * [`ceiling`](../../vonuvoli/definitions/ceiling.md#definition__vonuvoli__ceiling) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__ceiling__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(((n integer)) -> ((n integer)))`
   * input: `n` of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
   * output: `n` of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * `((real-not-inf-not-nan) -> (integer))`
   * input: a value of type [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
   * output: a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * `((real-inf) -> (real-inf))`
   * input: a value of type [`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
   * output: a value of type [`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
 * `((real-nan) -> (real-nan))`
   * input: a value of type [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);
   * output: a value of type [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);


<a id='definition__r7rs__ceiling__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__ceiling__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__ceiling__description'></a>

#### Description

> Please refer to [`floor`](../../r7rs/definitions/floor.md#definition__r7rs__floor).


<a id='definition__r7rs__ceiling__referenced-types'></a>

#### Referenced-types

 * [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
 * [`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
 * [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

