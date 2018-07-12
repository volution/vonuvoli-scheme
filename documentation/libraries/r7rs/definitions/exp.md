

<a id='definition__r7rs__exp'></a>

# `exp` -- `r7rs` Definitions


<a id='definition__r7rs__exp__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__exp__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((real-not-nan) -> (real-not-nan))`
   * input: a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
   * output: a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);


<a id='definition__r7rs__exp__exports'></a>

#### Exports

 * [`scheme:inexact`](../../r7rs/exports/scheme_3a_inexact.md#export__r7rs__scheme_3a_inexact);


<a id='definition__r7rs__exp__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__exp__description'></a>

#### Description

> ````
> (exp z)
> (log z)
> (log z_1 z_2)
> (sin z)
> (cos z)
> (tan z)
> (asin z)
> (acos z)
> (atan z)
> (atan y x)
> ````
> 
> 
> These procedures
> compute the usual transcendental functions.  The `log` procedure
> computes the natural logarithm of `z` (not the base ten logarithm)
> if a single argument is given, or the base-`z_2` logarithm of `z_1`
> if two arguments are given.
> The `asin`, `acos`, and `atan` procedures compute arcsine (`sin^-1`),
> arc-cosine (`cos^-1`), and arctangent (`tan^-1`), respectively.
> The two-argument variant of `atan` computes
> `(angle (make-rectangular x y))`
> (see below), even in implementations
> that don't support complex numbers.
> 
> In general, the mathematical functions log, arcsine, arc-cosine, and
> arctangent are multiply defined.
> The value of `log z` is defined to be the one whose imaginary part
> lies in the range from `-pi` (inclusive if `-0.0` is distinguished,
> exclusive otherwise) to `pi` (inclusive).
> The value of `log 0` is mathematically undefined.
> With `log` defined this way, the values of `sin^-1 z`, `cos^-1 z`,
> and `tan^-1 z` are according to the following formulae:
> `sin^-1 z = -i log(i z + sqrt(1 - z^2))`
> `cos^-1 z = pi / 2 - sin^-1 z`
> `tan^-1 z = (log(1 + i z) - log(1 - i z)) / (2 i)`
> 
> However, `(log 0.0)` returns `-inf.0`
> (and `(log -0.0)` returns `-inf.0+pi*i`) if the
> implementation supports infinities (and `-0.0`).
> 
> The range of `(atan y x)` is as in the
> following table. The asterisk (`*`) indicates that the entry applies to
> implementations that distinguish minus zero.
> 
> ````
> |     | `y` condition | `x` condition | range of result `r` |
> |     |   `y =  0.0`  |   `x >  0.0`  |  ` 0.0`             |
> | `*` |   `y = +0.0`  |   `x >  0.0`  |  `+0.0`             |
> | `*` |   `y = -0.0`  |   `x >  0.0`  |  `-0.0`             |
> |     |   `y >  0.0`  |   `x >  0.0`  |  ` 0.0 < r < pi/2`  |
> |     |   `y >  0.0`  |   `x =  0.0`  |  ` pi/2`            |
> |     |   `y >  0.0`  |   `x <  0.0`  |  ` pi/2 < r < pi`   |
> |     |   `y =  0.0`  |   `x <  0`    |  ` pi`              |
> | `*` |   `y = +0.0`  |   `x <  0.0`  |  ` pi`              |
> | `*` |   `y = -0.0`  |   `x <  0.0`  |  `-pi`              |
> |     |   `y <  0.0`  |   `x <  0.0`  |  `-pi< r < -pi/2`   |
> |     |   `y <  0.0`  |   `x =  0.0`  |  `-pi/2`            |
> |     |   `y <  0.0`  |   `x >  0.0`  |  `-pi/2 < r < 0.0`  |
> |     |   `y =  0.0`  |   `x =  0.0`  |  undefined          |
> | `*` |   `y = +0.0`  |   `x = +0.0`  |  `+0.0`             |
> | `*` |   `y = -0.0`  |   `x = +0.0`  |  `-0.0`             |
> | `*` |   `y = +0.0`  |   `x = -0.0`  |  ` pi`              |
> | `*` |   `y = -0.0`  |   `x = -0.0`  |  `-pi`              |
> | `*` |   `y = +0.0`  |   `x =  0`    |  ` pi/2`            |
> | `*` |   `y = -0.0`  |   `x =  0`    |  `-pi/2`            |
> ````
> 
> The above specification follows __Common Lisp: The Language, second edition__, which in turn
> cites __Principal values and branch cuts in complex APL__; refer to these sources for more detailed
> discussion of branch cuts, boundary conditions, and implementation of
> these functions.  When it is possible, these procedures produce a real
> result from a real argument.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__exp__referenced-types'></a>

#### Referenced-types

 * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);


<a id='definition__r7rs__exp__categories'></a>

#### Categories

 * [`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);


<a id='definition__r7rs__exp__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

