

<a id='definition__r7rs__ZZZZ__2d'></a>

# `-` -- `r7rs` Definition


<a id='definition__r7rs__ZZZZ__2d__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__ZZZZ__2d__implemented-by'></a>

#### Implemented by

 * [`-`](../../vonuvoli/definitions/ZZZZ__2d.md#definition__vonuvoli__ZZZZ__2d) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__ZZZZ__2d__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number-zero) -> (number-zero))`
   * input: a value of type [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
   * output: a value of type [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
 * `((number-positive) -> (number-negative))`
   * input: a value of type [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
   * output: a value of type [`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative);
 * `((number-negative) -> (number-positive))`
   * input: a value of type [`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative);
   * output: a value of type [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
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


<a id='definition__r7rs__ZZZZ__2d__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__ZZZZ__2d__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__ZZZZ__2d__description'></a>

#### Description

> ````
> (- z)
> (- z_1 z_2 ...)
> (/ z)
> (/ z_1 z_2 ...)
> ````
> 
> 
> With two or more arguments, these procedures return the difference or
> quotient of their arguments, associating to the left.  With one argument,
> however, they return the additive or multiplicative inverse of their argument.
> 
> It is an error if any argument of `/` other than the first is an exact zero.
> If the first argument is an exact zero, an implementation may return an
> exact zero unless one of the other arguments is a `NaN`.
> 
> ````
> (- 3 4)                 ===>  -1
> (- 3 4 5)               ===>  -6
> (- 3)                   ===>  -3
> (/ 3 4 5)               ===>   3/20
> (/ 3)                   ===>   1/3
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__ZZZZ__2d__referenced-types'></a>

#### Referenced-types

 * [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
 * [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
 * [`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative);
 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

