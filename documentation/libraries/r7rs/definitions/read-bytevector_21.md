

<a id='definition__r7rs__read-bytevector_21'></a>

# `read-bytevector!` -- `r7rs` Definitions


#### Kind

`procedure!`;


#### Procedure signature

Procedure variants:
 * `((|bytevector-not-empty|) |->| (|range-length-not-zero-or-eof|))`
   * input: a value of type [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
   * output: a value of type [`range-length-not-zero-or-eof`](../../r7rs/types/range-length-not-zero-or-eof.md#type__r7rs__range-length-not-zero-or-eof);
 * `((|bytevector-not-empty| |binary-input-port-eof|) |->| (|eof-object|))`
   * inputs:
     * a value of type [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
     * a value of type [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
   * output: a value of type [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * `((|bytevector-not-empty| |binary-input-port-open|) |->| (|range-length-not-zero-or-eof|))`
   * inputs:
     * a value of type [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
     * a value of type [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);
   * output: a value of type [`range-length-not-zero-or-eof`](../../r7rs/types/range-length-not-zero-or-eof.md#type__r7rs__range-length-not-zero-or-eof);
 * `((|bytevector-not-empty| |binary-input-port-eof| |range-start|) |->| (|eof-object|))`
   * inputs:
     * a value of type [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
     * a value of type [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * `((|bytevector-not-empty| |binary-input-port-open| |range-start|) |->| (|range-length-not-zero-or-eof|))`
   * inputs:
     * a value of type [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
     * a value of type [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`range-length-not-zero-or-eof`](../../r7rs/types/range-length-not-zero-or-eof.md#type__r7rs__range-length-not-zero-or-eof);
 * `((|bytevector-not-empty| |binary-input-port-eof| |range-start| |range-end|) |->| (|eof-object|))`
   * inputs:
     * a value of type [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
     * a value of type [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * `((|bytevector-not-empty| |binary-input-port-open| |range-start| |range-end|) |->| (|range-length-not-zero-or-eof|))`
   * inputs:
     * a value of type [`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
     * a value of type [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`range-length-not-zero-or-eof`](../../r7rs/types/range-length-not-zero-or-eof.md#type__r7rs__range-length-not-zero-or-eof);


#### Referenced types

[`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);
[`range-length-not-zero-or-eof`](../../r7rs/types/range-length-not-zero-or-eof.md#type__r7rs__range-length-not-zero-or-eof);
[`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
[`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
[`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);
[`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
[`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);


#### Description

> ````
> (read-bytevector! bytevector)
> (read-bytevector! bytevector port)
> (read-bytevector! bytevector port start)
> (read-bytevector! bytevector port start end)
> ````
> 
> 
> Reads the next `end - start` bytes, or as many as are available
> before the end of file,
> from the binary
> input `port` into `bytevector` in left-to-right order
> beginning at the `start` position.  If `end` is not supplied,
> reads until the end of `bytevector` has been reached.  If
> `start` is not supplied, reads beginning at position 0.
> Returns the number of bytes read.
> If no bytes are available, an end-of-file object is returned.
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

