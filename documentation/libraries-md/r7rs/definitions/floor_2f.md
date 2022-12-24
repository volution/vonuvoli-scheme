

<a id='definition__r7rs__floor_2f'></a>

# `floor/` -- `r7rs` Definition


<a id='definition__r7rs__floor_2f__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__floor_2f__implemented-by'></a>

#### Implemented by

 * [`floor/`](../../vonuvoli/definitions/floor_2f.md#definition__vonuvoli__floor_2f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__floor_2f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((real-not-nan real-not-zero-not-nan) -> (real-not-nan real-not-nan))`
   * inputs:
     * a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
     * a value of type [`real-not-zero-not-nan`](../../r7rs/types/real-not-zero-not-nan.md#type__r7rs__real-not-zero-not-nan);
   * outputs:
     * a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
     * a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * `((real real-not-zero) -> (real-nan real-nan))`
   * inputs:
     * a value of type [`real`](../../r7rs/types/real.md#type__r7rs__real);
     * a value of type [`real-not-zero`](../../r7rs/types/real-not-zero.md#type__r7rs__real-not-zero);
   * outputs:
     * a value of type [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);
     * a value of type [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);


<a id='definition__r7rs__floor_2f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__floor_2f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__floor_2f__description'></a>

#### Description

> ````
> (floor/ n_1 n_2)
> (floor-quotient n_1 n_2)
> (floor-remainder n_1 n_2)
> (truncate/ n_1 n_2)
> (truncate-quotient n_1 n_2)
> (truncate-remainder n_1 n_2)
> ````
> 
> 
> These procedures implement
> number-theoretic (integer) division.  It is an error if `n_2` is zero.
> The procedures ending in `/` return two integers; the other
> procedures return an integer.  All the procedures compute a
> quotient `n_q` and remainder `n_r` such that
> `n_1 = n_2 n_q + n_r`.  For each of the
> division operators, there are three procedures defined as follows:
> 
> ````
> (<operator>/ n_1 n_2)             ===>  n_q n_r
> (<operator>-quotient n_1 n_2)     ===>  n_q
> (<operator>-remainder n_1 n_2)    ===>  n_r
> ````
> 
> The remainder `n_r` is determined by the choice of integer
> `n_q`: `n_r = n_1 - n_2 n_q`.  Each set of
> operators uses a different choice of `n_q`:
> 
>  * `floor` -- `n_q = floor(n_1 / n_2)`;
>  * `truncate` -- `n_q = truncate(n_1 / n_2)`;
> 
> For any of the operators, and for integers `n_1` and `n_2`
> with `n_2` not equal to 0,
> ````
>      (= n_1 (+ (* n_2 (<operator>-quotient n_1 n_2))
>            (<operator>-remainder n_1 n_2)))
>                                  ===>  #t
> ````
> provided all numbers involved in that computation are exact.
> 
> Examples:
> 
> ````
> (floor/ 5 2)         ===>   2    1
> (floor/ -5 2)        ===>  -3    1
> (floor/ 5 -2)        ===>  -3   -1
> (floor/ -5 -2)       ===>   2   -1
> (truncate/ 5 2)      ===>   2    1
> (truncate/ -5 2)     ===>  -2   -1
> (truncate/ 5 -2)     ===>  -2    1
> (truncate/ -5 -2)    ===>   2   -1
> (truncate/ -5.0 -2)  ===>   2.0 -1.0
> ````
> 
> 
> 
> 
> ````
> (quotient n_1 n_2)
> (remainder n_1 n_2)
> (modulo n_1 n_2)
> ````
> 
> 
> The `quotient` and `remainder` procedures are equivalent to
> `truncate-quotient` and `truncate-remainder`, respectively, and
> `modulo` is equivalent to `floor-remainder`.
> 
> **Note**:  These procedures are provided for backward compatibility with earlier
> versions of this report.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__floor_2f__referenced-types'></a>

#### Referenced-types

 * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * [`real-not-zero-not-nan`](../../r7rs/types/real-not-zero-not-nan.md#type__r7rs__real-not-zero-not-nan);
 * [`real`](../../r7rs/types/real.md#type__r7rs__real);
 * [`real-not-zero`](../../r7rs/types/real-not-zero.md#type__r7rs__real-not-zero);
 * [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

