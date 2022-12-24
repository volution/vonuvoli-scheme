

<a id='definition__r7rs__denominator'></a>

# `denominator` -- `r7rs` Definition


<a id='definition__r7rs__denominator__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__denominator__implemented-by'></a>

#### Implemented by

 * [`denominator`](../../vonuvoli/definitions/denominator.md#definition__vonuvoli__denominator) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__denominator__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((integer) -> ((&constant +1)))`
   * input: a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
   * output: a constant with value `+1`;
 * `((rational-zero) -> ((&constant +1)))`
   * input: a value of type [`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero);
   * output: a constant with value `+1`;
 * `((rational-not-zero) -> (integer-positive))`
   * input: a value of type [`rational-not-zero`](../../r7rs/types/rational-not-zero.md#type__r7rs__rational-not-zero);
   * output: a value of type [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive);


<a id='definition__r7rs__denominator__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__denominator__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__denominator__description'></a>

#### Description

> Please refer to [`numerator`](../../r7rs/definitions/numerator.md#definition__r7rs__numerator).


<a id='definition__r7rs__denominator__referenced-types'></a>

#### Referenced-types

 * [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * [`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero);
 * [`rational-not-zero`](../../r7rs/types/rational-not-zero.md#type__r7rs__rational-not-zero);
 * [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

