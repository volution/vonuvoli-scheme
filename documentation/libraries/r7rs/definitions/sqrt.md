

<a id='definition__r7rs__sqrt'></a>

# `sqrt` -- `r7rs` Definition


<a id='definition__r7rs__sqrt__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__sqrt__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number-zero) -> (number-zero))`
   * input: a value of type [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
   * output: a value of type [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
 * `((real-positive-not-inf) -> (real-positive-not-inf))`
   * input: a value of type [`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf);
   * output: a value of type [`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf);
 * `((real-negative-not-inf) -> (complex-not-inf-not-nan))`
   * input: a value of type [`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf);
   * output: a value of type [`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan);


<a id='definition__r7rs__sqrt__exports'></a>

#### Exports

 * [`scheme:inexact`](../../r7rs/exports/scheme_3a_inexact.md#export__r7rs__scheme_3a_inexact) -- `(scheme inexact)`;


<a id='definition__r7rs__sqrt__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__sqrt__description'></a>

#### Description

> ````
> (sqrt z)
> ````
> 
> 
> Returns the principal square root of `z`.  The result will have
> either a positive real part, or a zero real part and a non-negative imaginary
> part.
> 
> ````
> (sqrt 9)   ===>   3
> (sqrt -1)  ===>  +i
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__sqrt__referenced-types'></a>

#### Referenced-types

 * [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
 * [`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf);
 * [`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf);
 * [`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan);


<a id='definition__r7rs__sqrt__categories'></a>

#### Categories

 * [`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);


<a id='definition__r7rs__sqrt__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

