

<a id='definition__r7rs__infinite_3f'></a>

# `infinite?` -- `r7rs` Definitions


#### Kind

`predicate`;


#### Procedure signature

Procedure variants:
 * `((|number-nan|) |->| (|false|))`
   * input: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|number-inf|) |->| (|true|))`
   * input: a value of type [`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|number-not-inf-not-nan|) |->| (|false|))`
   * input: a value of type [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


#### Referenced types

[`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
[`false`](../../r7rs/types/false.md#type__r7rs__false);
[`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);


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


#### Categories

[`r7rs:inexact`](../../r7rs/categories/r7rs_3a_inexact.md#category__r7rs__r7rs_3a_inexact);
[`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

