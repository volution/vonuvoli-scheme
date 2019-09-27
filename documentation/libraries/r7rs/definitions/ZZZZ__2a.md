

<a id='definition__r7rs__ZZZZ__2a'></a>

# `*` -- `r7rs` Definition


<a id='definition__r7rs__ZZZZ__2a__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__ZZZZ__2a__implemented-by'></a>

#### Implemented by

 * [`*`](../../vonuvoli/definitions/ZZZZ__2a.md#definition__vonuvoli__ZZZZ__2a) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__ZZZZ__2a__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> ((&constant +1)))`
   * inputs: none;
   * output: a constant with value `+1`;
 * `(((z number-not-nan)) -> ((z number-not-nan)))`
   * input: `z` of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
   * output: `z` of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * `((number-not-nan |2...|) -> (number))`
   * inputs:
     * a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
     * `...` -- at least 2 times;
   * output: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * `((number |1...|) -> (number-nan))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * `...` -- at least one time;
   * output: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


<a id='definition__r7rs__ZZZZ__2a__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__ZZZZ__2a__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__ZZZZ__2a__description'></a>

#### Description

> Please refer to [`+`](../../r7rs/definitions/ZZZZ__2b.md#definition__r7rs__ZZZZ__2b).


<a id='definition__r7rs__ZZZZ__2a__referenced-types'></a>

#### Referenced-types

 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

