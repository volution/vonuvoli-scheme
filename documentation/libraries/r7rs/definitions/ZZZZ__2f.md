

<a id='definition__r7rs__ZZZZ__2f'></a>

# `/` -- `r7rs` Definition


<a id='definition__r7rs__ZZZZ__2f__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__ZZZZ__2f__implemented-by'></a>

#### Implemented by

 * [`/`](../../vonuvoli/definitions/ZZZZ__2f.md#definition__vonuvoli__ZZZZ__2f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__ZZZZ__2f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number-not-zero-not-nan) -> (number-not-zero-not-nan))`
   * input: a value of type [`number-not-zero-not-nan`](../../r7rs/types/number-not-zero-not-nan.md#type__r7rs__number-not-zero-not-nan);
   * output: a value of type [`number-not-zero-not-nan`](../../r7rs/types/number-not-zero-not-nan.md#type__r7rs__number-not-zero-not-nan);
 * `((number-not-nan number-not-zero-not-nan |1...|) -> (number))`
   * inputs:
     * a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
     * a value of type [`number-not-zero-not-nan`](../../r7rs/types/number-not-zero-not-nan.md#type__r7rs__number-not-zero-not-nan);
     * `...` -- at least one time;
   * output: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * `((number number-not-zero |1...|) -> (number-nan))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * a value of type [`number-not-zero`](../../r7rs/types/number-not-zero.md#type__r7rs__number-not-zero);
     * `...` -- at least one time;
   * output: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


<a id='definition__r7rs__ZZZZ__2f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__ZZZZ__2f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__ZZZZ__2f__description'></a>

#### Description

> Please refer to [`-`](../../r7rs/definitions/ZZZZ__2d.md#definition__r7rs__ZZZZ__2d).


<a id='definition__r7rs__ZZZZ__2f__referenced-types'></a>

#### Referenced-types

 * [`number-not-zero-not-nan`](../../r7rs/types/number-not-zero-not-nan.md#type__r7rs__number-not-zero-not-nan);
 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-not-zero`](../../r7rs/types/number-not-zero.md#type__r7rs__number-not-zero);
 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

