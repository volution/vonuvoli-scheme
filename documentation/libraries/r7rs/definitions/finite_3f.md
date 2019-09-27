

<a id='definition__r7rs__finite_3f'></a>

# `finite?` -- `r7rs` Definition


<a id='definition__r7rs__finite_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__r7rs__finite_3f__implemented-by'></a>

#### Implemented by

 * [`finite?`](../../vonuvoli/definitions/finite_3f.md#definition__vonuvoli__finite_3f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__finite_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number-nan) -> (false))`
   * input: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((number-inf) -> (false))`
   * input: a value of type [`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((number-not-inf-not-nan) -> (true))`
   * input: a value of type [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);


<a id='definition__r7rs__finite_3f__exports'></a>

#### Exports

 * [`scheme:inexact`](../../r7rs/exports/scheme_3a_inexact.md#export__r7rs__scheme_3a_inexact) -- `(scheme inexact)`;


<a id='definition__r7rs__finite_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__finite_3f__description'></a>

#### Description

> ````
> (finite? z)
> ````
> 
> 
> The `finite?` procedure returns `#t` on all real numbers except
> `+inf.0`, `-inf.0`, and `+nan.0`, and on complex
> numbers if their real and imaginary parts are both finite.
> Otherwise it returns `#f`.
> 
> ````
> (finite? 3)            ===>  #t
> (finite? +inf.0)       ===>  #f
> (finite? 3.0+inf.0i)   ===>  #f
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__finite_3f__referenced-types'></a>

#### Referenced-types

 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
 * [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

