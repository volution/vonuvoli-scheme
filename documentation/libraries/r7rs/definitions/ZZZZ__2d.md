

<a id='definition__r7rs__ZZZZ__2d'></a>

# `-` -- `r7rs` Definitions


<a id='definition__r7rs__ZZZZ__2d__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__ZZZZ__2d__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number-not-nan) -> (number-not-nan))`
   * input: a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
   * output: a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * `((number-not-nan ...) -> (number))`
   * inputs:
     * a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
     * `...` (i.e. variadic);
   * output: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * `((number ...) -> (number-nan))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * `...` (i.e. variadic);
   * output: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


<a id='definition__r7rs__ZZZZ__2d__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base);


<a id='definition__r7rs__ZZZZ__2d__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


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

 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


<a id='definition__r7rs__ZZZZ__2d__categories'></a>

#### Categories

 * [`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);


<a id='definition__r7rs__ZZZZ__2d__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

