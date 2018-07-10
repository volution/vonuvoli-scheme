

<a id='definition__r7rs__string-_3e_utf8'></a>

# `string->utf8` -- `r7rs` Definitions


#### Kind

`converter`;


#### Procedure signature

Procedure variants:
 * `((|string-empty|) |->| (|bytevector-empty|))`
   * input: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
   * output: a value of type [`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
 * `((|string-not-empty|) |->| (|bytevector-not-empty|))`
   * input: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
   * output: a value of type [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
 * `((|string| |range-start|) |->| (|bytevector|))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
 * `((|string| |range-start| |range-end|) |->| (|bytevector|))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);


#### Referenced types

[`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
[`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
[`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
[`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
[`string`](../../r7rs/types/string.md#type__r7rs__string);
[`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
[`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
[`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);


#### Description

> Please refer to [`utf8->string`](../../r7rs/definitions/utf8-_3e_string.md#definition__r7rs__utf8-_3e_string).


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:bytes`](../../r7rs/categories/vs_3a_bytes.md#category__r7rs__vs_3a_bytes);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

