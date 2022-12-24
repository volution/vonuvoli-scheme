

<a id='definition__r7rs__gcd'></a>

# `gcd` -- `r7rs` Definition


<a id='definition__r7rs__gcd__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__gcd__implemented-by'></a>

#### Implemented by

 * [`gcd`](../../vonuvoli/definitions/gcd.md#definition__vonuvoli__gcd) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__gcd__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(((n integer)) -> ((n integer)))`
   * input: `n` of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
   * output: `n` of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * `((integer |2...|) -> (integer))`
   * inputs:
     * a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
     * `...` -- at least 2 times;
   * output: a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);


<a id='definition__r7rs__gcd__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__gcd__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__gcd__description'></a>

#### Description

> ````
> (gcd n_1 ...)
> (lcm n_1 ...)
> ````
> 
> 
> These procedures return the greatest common divisor or least common
> multiple of their arguments.  The result is always non-negative.
> 
> ````
> (gcd 32 -36)            ===>  4
> (gcd)                   ===>  0
> (lcm 32 -36)            ===>  288
> (lcm 32.0 -36)          ===>  288.0  ; inexact
> (lcm)                   ===>  1
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__gcd__referenced-types'></a>

#### Referenced-types

 * [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

