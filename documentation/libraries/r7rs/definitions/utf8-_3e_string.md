

<a id='definition__r7rs__utf8-_3e_string'></a>

# `utf8->string` -- `r7rs` Definitions


#### Kind

`converter`;


#### Procedure signature

Procedure variants:
 * `((|bytevector-empty|) |->| (|string-empty|))`
   * input: a value of type [`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
   * output: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
 * `((|bytevector-not-empty|) |->| (|string-not-empty|))`
   * input: a value of type [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
   * output: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
 * `((|bytevector| |range-start|) |->| (|string|))`
   * inputs:
     * a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * `((|bytevector| |range-start| |range-end|) |->| (|string|))`
   * inputs:
     * a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);


#### Referenced types

[`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
[`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
[`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
[`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
[`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
[`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
[`string`](../../r7rs/types/string.md#type__r7rs__string);
[`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);


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
> (string->utf8 "$\lambda$") ===> #u8(#xCE #xBB)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:bytes`](../../r7rs/categories/vs_3a_bytes.md#category__r7rs__vs_3a_bytes);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

