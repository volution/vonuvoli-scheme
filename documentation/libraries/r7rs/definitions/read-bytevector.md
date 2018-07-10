

<a id='definition__r7rs__read-bytevector'></a>

# `read-bytevector` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|range-length-not-zero|) |->| (|bytevector-or-eof|))`
   * input: a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
   * output: a value of type [`bytevector-or-eof`](../../r7rs/types/bytevector-or-eof.md#type__r7rs__bytevector-or-eof);
 * `((|range-length-not-zero| |binary-input-port-eof|) |->| (|eof-object|))`
   * inputs:
     * a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
     * a value of type [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
   * output: a value of type [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * `((|range-length-not-zero| |binary-input-port-open|) |->| (|bytevector-or-eof|))`
   * inputs:
     * a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
     * a value of type [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);
   * output: a value of type [`bytevector-or-eof`](../../r7rs/types/bytevector-or-eof.md#type__r7rs__bytevector-or-eof);


#### Referenced types

[`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
[`bytevector-or-eof`](../../r7rs/types/bytevector-or-eof.md#type__r7rs__bytevector-or-eof);
[`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
[`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
[`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);


#### Description

> ````
> (read-bytevector k)
> (read-bytevector k port)
> ````
> 
> 
> Reads the next `k` bytes, or as many as are available before the end of file,
> from the binary
> input `port` into a newly allocated bytevector in left-to-right order
> and returns the bytevector.
> If no bytes are available before the end of file,
> an end-of-file object is returned.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:ports:input`](../../r7rs/categories/vs_3a_ports_3a_input.md#category__r7rs__vs_3a_ports_3a_input);
[`vs:bytes`](../../r7rs/categories/vs_3a_bytes.md#category__r7rs__vs_3a_bytes);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

