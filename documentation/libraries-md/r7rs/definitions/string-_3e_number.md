

<a id='definition__r7rs__string-_3e_number'></a>

# `string->number` -- `r7rs` Definition


<a id='definition__r7rs__string-_3e_number__kind'></a>

#### Kind

`converter`;


<a id='definition__r7rs__string-_3e_number__implemented-by'></a>

#### Implemented by

 * [`string->number`](../../vonuvoli/definitions/string-_3e_number.md#definition__vonuvoli__string-_3e_number) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__string-_3e_number__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((string-empty) -> (false))`
   * input: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((string-not-empty) -> (number-or-false))`
   * input: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
   * output: a value of type [`number-or-false`](../../r7rs/types/number-or-false.md#type__r7rs__number-or-false);
 * `((string-empty number-radix) -> (false))`
   * inputs:
     * a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
     * a value of type [`number-radix`](../../r7rs/types/number-radix.md#type__r7rs__number-radix);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((string-not-empty number-radix) -> (number-or-false))`
   * inputs:
     * a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
     * a value of type [`number-radix`](../../r7rs/types/number-radix.md#type__r7rs__number-radix);
   * output: a value of type [`number-or-false`](../../r7rs/types/number-or-false.md#type__r7rs__number-or-false);


<a id='definition__r7rs__string-_3e_number__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__string-_3e_number__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__string-_3e_number__description'></a>

#### Description

> ````
> (string->number string)
> (string->number string radix)
> ````
> 
> 
> Returns a number of the maximally precise representation expressed by the
> given `string`.
> 
> **Domain**:  It is an error if `radix` is not `2`, `8`, `10`, or `16`.
> 
> If supplied, `radix` is a default radix that will be overridden
> if an explicit radix prefix is present in `string` (e.g. `"#o177"`).  If `radix`
> is not supplied, then the default radix is `10`.  If `string` is not
> a syntactically valid notation for a number, or would result in a
> number that the implementation cannot represent, then `string->number`
> returns `#f`.
> An error is never signaled due to the content of `string`.
> 
> ````
> (string->number "100")        ===>  100
> (string->number "100" 16)     ===>  256
> (string->number "1e2")        ===>  100.0
> ````
> 
> **Note**:  The domain of `string->number` may be restricted by implementations
> in the following ways.
> If all numbers supported by an implementation are real, then
> `string->number` is permitted to return `#f` whenever
> `string` uses the polar or rectangular notations for complex
> numbers.  If all numbers are integers, then
> `string->number` may return `#f` whenever
> the fractional notation is used.  If all numbers are exact, then
> `string->number` may return `#f` whenever
> an exponent marker or explicit exactness prefix is used.
> If all inexact
> numbers are integers, then
> `string->number` may return `#f` whenever
> a decimal point is used.
> 
> **Note**:  The rules used by a particular implementation for `string->number` must
> also be applied to `read` and to the routine that reads programs, in
> order to maintain consistency between internal numeric processing, I/O,
> and the processing of programs.
> As a consequence, the __R5RS__ permission to return `#f` when
> `string` has an explicit radix prefix has been withdrawn.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__string-_3e_number__referenced-types'></a>

#### Referenced-types

 * [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
 * [`number-or-false`](../../r7rs/types/number-or-false.md#type__r7rs__number-or-false);
 * [`number-radix`](../../r7rs/types/number-radix.md#type__r7rs__number-radix);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

