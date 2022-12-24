

<a id='definition__r7rs__utf8-_3e_string'></a>

# `utf8->string` -- `r7rs` Definition


<a id='definition__r7rs__utf8-_3e_string__kind'></a>

#### Kind

`converter`;


<a id='definition__r7rs__utf8-_3e_string__implemented-by'></a>

#### Implemented by

 * [`utf8->string`](../../vonuvoli/definitions/utf8-_3e_string.md#definition__vonuvoli__utf8-_3e_string) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__utf8-_3e_string__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((bytevector-empty) -> (string-empty))`
   * input: a value of type [`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
   * output: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * `((bytevector-not-empty) -> (string-not-empty))`
   * input: a value of type [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
   * output: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
 * `((bytevector range-start) -> (string))`
   * inputs:
     * a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * `((bytevector range-start range-end) -> (string))`
   * inputs:
     * a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);


<a id='definition__r7rs__utf8-_3e_string__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__utf8-_3e_string__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__utf8-_3e_string__description'></a>

#### Description

> ````
> (utf8->string bytevector)
> (utf8->string bytevector start)
> (utf8->string bytevector start end)
> (string->utf8 string)
> (string->utf8 string start)
> (string->utf8 string start end)
> ````
> 
> 
> **Domain**:  It is an error for `bytevector` to contain invalid __UTF-8__ byte sequences.
> 
> These procedures translate between strings and bytevectors
> that encode those strings using the __UTF-8__ encoding.
> The `utf8->string` procedure decodes the bytes of
> a bytevector between `start` and `end`
> and returns the corresponding string;
> the `string->utf8` procedure encodes the characters of a
> string between `start` and `end`
> and returns the corresponding bytevector.
> 
> ````
> (utf8->string #u8(#x41)) ===> "A"
> (string->utf8 "λ") ===> #u8(#xCE #xBB)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__utf8-_3e_string__referenced-types'></a>

#### Referenced-types

 * [`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
 * [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
 * [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
 * [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
 * [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

