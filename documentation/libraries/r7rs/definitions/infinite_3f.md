

<a id='definition__r7rs__infinite_3f'></a>

# `infinite?` -- `r7rs` Definitions


<a id='definition__r7rs__infinite_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__r7rs__infinite_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number-nan) -> (false))`
   * input: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((number-inf) -> (true))`
   * input: a value of type [`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((number-not-inf-not-nan) -> (false))`
   * input: a value of type [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__infinite_3f__exports'></a>

#### Exports

 * [`scheme:inexact`](../../r7rs/exports/scheme_3a_inexact.md#export__r7rs__scheme_3a_inexact) -- `(scheme inexact)`;


<a id='definition__r7rs__infinite_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__infinite_3f__description'></a>

#### Description

> ````
> (infinite? z)
> ````
> 
> 
> The `infinite?` procedure returns `#t` on the real numbers
> `+inf.0` and `-inf.0`, and on complex
> numbers if their real or imaginary parts or both are infinite.
> Otherwise it returns `#f`.
> 
> ````
> (infinite? 3)            ===>  #f
> (infinite? +inf.0)       ===>  #t
> (infinite? +nan.0)       ===>  #f
> (infinite? 3.0+inf.0i)   ===>  #t
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__infinite_3f__referenced-types'></a>

#### Referenced-types

 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);


<a id='definition__r7rs__infinite_3f__categories'></a>

#### Categories

 * [`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);


<a id='definition__r7rs__infinite_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

