

<a id='definition__r7rs__nan_3f'></a>

# `nan?` -- `r7rs` Definition


<a id='definition__r7rs__nan_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__r7rs__nan_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number-nan) -> (true))`
   * input: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((number-inf) -> (false))`
   * input: a value of type [`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((number-not-inf-not-nan) -> (false))`
   * input: a value of type [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__nan_3f__exports'></a>

#### Exports

 * [`scheme:inexact`](../../r7rs/exports/scheme_3a_inexact.md#export__r7rs__scheme_3a_inexact) -- `(scheme inexact)`;


<a id='definition__r7rs__nan_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__nan_3f__description'></a>

#### Description

> ````
> (nan? z)
> ````
> 
> 
> The `nan?` procedure returns `#t` on `+nan.0`, and on complex
> numbers if their real or imaginary parts or both are `+nan.0`.
> Otherwise it returns `#f`.
> 
> ````
> (nan? +nan.0)          ===>  #t
> (nan? 32)              ===>  #f
> (nan? +nan.0+5.0i)     ===>  #t
> (nan? 1+2i)            ===>  #f
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__nan_3f__referenced-types'></a>

#### Referenced-types

 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);


<a id='definition__r7rs__nan_3f__categories'></a>

#### Categories

 * [`vs:arithmetic`](../../vonuvoli/categories/vs_3a_arithmetic.md#category__vonuvoli__vs_3a_arithmetic);


<a id='definition__r7rs__nan_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

