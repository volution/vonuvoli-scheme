

<a id='definition__r7rs__number-_3e_string'></a>

# `number->string` -- `r7rs` Definition


<a id='definition__r7rs__number-_3e_string__kind'></a>

#### Kind

`converter`;


<a id='definition__r7rs__number-_3e_string__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number) -> (string-not-empty))`
   * input: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
   * output: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
 * `((number number-radix) -> (string-not-empty))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * a value of type [`number-radix`](../../r7rs/types/number-radix.md#type__r7rs__number-radix);
   * output: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);


<a id='definition__r7rs__number-_3e_string__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__number-_3e_string__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__number-_3e_string__description'></a>

#### Description

> ````
> (number->string z)
> (number->string z radix)
> ````
> 
> 
> **Domain**:  It is an error if `radix` is not one of `2`, `8`, `10`, or `16`.
> 
> The procedure `number->string` takes a
> number and a radix and returns as a string an external representation of
> the given number in the given radix such that
> ````
> (let ((number number)
>       (radix radix))
>   (eqv? number
>         (string->number (number->string number
>                                         radix)
>                         radix)))
> ````
> is true.  It is an error if no possible result makes this expression true.
> If omitted, `radix` defaults to `10`.
> 
> If `z` is inexact, the radix is `10`, and the above expression
> can be satisfied by a result that contains a decimal point,
> then the result contains a decimal point and is expressed using the
> minimum number of digits (exclusive of exponent and trailing
> zeroes) needed to make the above expression
> true;
> otherwise the format of the result is unspecified.
> 
> The result returned by `number->string`
> never contains an explicit radix prefix.
> 
> **Note**:  The error case can occur only when `z` is not a complex number
> or is a complex number with a non-rational real or imaginary part.
> 
> **Rationale**:  If `z` is an inexact number and
> the radix is `10`, then the above expression is normally satisfied by
> a result containing a decimal point.  The unspecified case
> allows for infinities, `NaN`s, and unusual representations.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__number-_3e_string__referenced-types'></a>

#### Referenced-types

 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
 * [`number-radix`](../../r7rs/types/number-radix.md#type__r7rs__number-radix);


<a id='definition__r7rs__number-_3e_string__categories'></a>

#### Categories

 * [`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);
 * [`vs:conversions`](../../r7rs/categories/vs_3a_conversions.md#category__r7rs__vs_3a_conversions);


<a id='definition__r7rs__number-_3e_string__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

