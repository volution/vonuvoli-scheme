

<a id='definition__r7rs__make-string'></a>

# `make-string` -- `r7rs` Definition


<a id='definition__r7rs__make-string__kind'></a>

#### Kind

`constructor`;


<a id='definition__r7rs__make-string__implemented-by'></a>

#### Implemented by

 * [`make-string`](../../vonuvoli/definitions/make-string.md#definition__vonuvoli__make-string) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__make-string__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((range-length-zero) -> (string-empty))`
   * input: a value of type [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
   * output: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * `((range-length-zero character) -> (string-empty))`
   * inputs:
     * a value of type [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * `((range-length-not-zero) -> (string-not-empty))`
   * input: a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
   * output: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
 * `((range-length-not-zero character) -> (string-not-empty))`
   * inputs:
     * a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);


<a id='definition__r7rs__make-string__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__make-string__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__make-string__description'></a>

#### Description

> ````
> (make-string k)
> (make-string k char)
> ````
> 
> 
> The `make-string` procedure returns a newly allocated string of
> length `k`.  If `char` is given, then all the characters of the string
> are initialized to `char`, otherwise the contents of the
> string are unspecified.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__make-string__referenced-types'></a>

#### Referenced-types

 * [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
 * [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * [`character`](../../r7rs/types/character.md#type__r7rs__character);
 * [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
 * [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

