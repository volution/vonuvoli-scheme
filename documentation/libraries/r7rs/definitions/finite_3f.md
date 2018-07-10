

<a id='definition__r7rs__finite_3f'></a>

# `finite?` -- `r7rs` Definitions


#### Kind

`predicate`;


#### Procedure signature

Procedure variants:
 * `((|number-nan|) |->| (|false|))`
   * input: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|number-inf|) |->| (|false|))`
   * input: a value of type [`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|number-not-inf-not-nan|) |->| (|true|))`
   * input: a value of type [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);


#### Referenced types

[`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
[`false`](../../r7rs/types/false.md#type__r7rs__false);
[`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
[`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
[`true`](../../r7rs/types/true.md#type__r7rs__true);


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


#### Categories

[`r7rs:inexact`](../../r7rs/categories/r7rs_3a_inexact.md#category__r7rs__r7rs_3a_inexact);
[`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

