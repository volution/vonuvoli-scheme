

<a id='definition__r7rs__make-rectangular'></a>

# `make-rectangular` -- `r7rs` Definitions


<a id='definition__r7rs__make-rectangular__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__make-rectangular__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((real-not-inf-not-nan real-zero) -> (real-not-inf-not-nan))`
   * inputs:
     * a value of type [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
     * a value of type [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero);
   * output: a value of type [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
 * `((real-not-inf-not-nan real-not-inf-not-nan) -> (complex-not-inf-not-nan))`
   * inputs:
     * a value of type [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
     * a value of type [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
   * output: a value of type [`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan);


<a id='definition__r7rs__make-rectangular__exports'></a>

#### Exports

 * [`scheme:complex`](../../r7rs/exports/scheme_3a_complex.md#export__r7rs__scheme_3a_complex) -- `(scheme complex)`;


<a id='definition__r7rs__make-rectangular__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__make-rectangular__description'></a>

#### Description

> ````
> (make-rectangular x_1 x_2)
> (make-polar x_3 x_4)
> (real-part z)
> (imag-part z)
> (magnitude z)
> (angle z)
> ````
> 
> 
> Let `x_1`, `x_2`, `x_3`, and `x_4` be
> real numbers and `z` be a complex number such that
> `z = x_1 + x_2*i = x_3 * e^(x_4*i)`
> Then all of
> ````
> (make-rectangular x_1 x_2)     ===>  z
> (make-polar x_3 x_4)           ===>  z
> (real-part z)                  ===>  x_1
> (imag-part z)                  ===>  x_2
> (magnitude z)                  ===>  | x_3 |
> (angle z)                      ===>  x_angle
> ````
> are true, where `-pi <= x_angle <= pi` with `x_angle = x_4 + 2 pi n`
> for some integer `n`.
> 
> The `make-polar` procedure may return an inexact complex number even if its
> arguments are exact.
> The `real-part` and `imag-part` procedures may return exact real
> numbers when applied to an inexact complex number if the corresponding
> argument passed to `make-rectangular` was exact.
> 
> 
> **Rationale**:  The `magnitude` procedure is the same as `abs` for a real argument,
> but `abs` is in the base library, whereas
> `magnitude` is in the optional complex library.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__make-rectangular__referenced-types'></a>

#### Referenced-types

 * [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
 * [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero);
 * [`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan);


<a id='definition__r7rs__make-rectangular__categories'></a>

#### Categories

 * [`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);
 * [`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);


<a id='definition__r7rs__make-rectangular__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

