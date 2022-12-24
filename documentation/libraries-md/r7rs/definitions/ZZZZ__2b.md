

<a id='definition__r7rs__ZZZZ__2b'></a>

# `+` -- `r7rs` Definition


<a id='definition__r7rs__ZZZZ__2b__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__ZZZZ__2b__implemented-by'></a>

#### Implemented by

 * [`+`](../../vonuvoli/definitions/ZZZZ__2b.md#definition__vonuvoli__ZZZZ__2b) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__ZZZZ__2b__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> ((&constant 0)))`
   * inputs: none;
   * output: a constant with value `0`;
 * `(((z number-not-nan)) -> ((z number-not-nan)))`
   * input: `z` of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
   * output: `z` of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * `((number-not-nan |2...|) -> (number))`
   * inputs:
     * a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
     * `...` -- at least 2 times;
   * output: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * `((number |1...|) -> (number-nan))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * `...` -- at least one time;
   * output: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


<a id='definition__r7rs__ZZZZ__2b__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__ZZZZ__2b__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__ZZZZ__2b__description'></a>

#### Description

> ````
> (+ z_1 ...)
> (* z_1 ...)
> ````
> 
> 
> These procedures return the sum or product of their arguments.
> 
> ````
> (+ 3 4)                 ===>  7
> (+ 3)                   ===>  3
> (+)                     ===>  0
> (* 4)                   ===>  4
> (*)                     ===>  1
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__ZZZZ__2b__referenced-types'></a>

#### Referenced-types

 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

