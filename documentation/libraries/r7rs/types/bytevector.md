

<a id='type__r7rs__bytevector'></a>

# `bytevector` -- `r7rs` Types


#### Super-type

[(none)](../../r7rs/types/_index.md#toc__r7rs__types);


#### Sub-types

[`bytevector-empty`](../../r7rs/types/bytevector-empty.md#type__r7rs__bytevector-empty);
[`bytevector-not-empty`](../../r7rs/types/bytevector-not-empty.md#type__r7rs__bytevector-not-empty);


#### Referent definitions as input

[`bytevector?`](../../r7rs/definitions/bytevector_3f.md#definition__r7rs__bytevector_3f);
[`bytevector-append`](../../r7rs/definitions/bytevector-append.md#definition__r7rs__bytevector-append);
[`bytevector-copy`](../../r7rs/definitions/bytevector-copy.md#definition__r7rs__bytevector-copy);
[`bytevector-copy!`](../../r7rs/definitions/bytevector-copy_21.md#definition__r7rs__bytevector-copy_21);
[`bytevector-u8-ref`](../../r7rs/definitions/bytevector-u8-ref.md#definition__r7rs__bytevector-u8-ref);
[`bytevector-u8-set!`](../../r7rs/definitions/bytevector-u8-set_21.md#definition__r7rs__bytevector-u8-set_21);
[`utf8->string`](../../r7rs/definitions/utf8-_3e_string.md#definition__r7rs__utf8-_3e_string);
[`open-input-bytevector`](../../r7rs/definitions/open-input-bytevector.md#definition__r7rs__open-input-bytevector);
[`write-bytevector`](../../r7rs/definitions/write-bytevector.md#definition__r7rs__write-bytevector);


#### Referent definitions as output

[`bytevector-append`](../../r7rs/definitions/bytevector-append.md#definition__r7rs__bytevector-append);
[`bytevector-copy`](../../r7rs/definitions/bytevector-copy.md#definition__r7rs__bytevector-copy);
[`string->utf8`](../../r7rs/definitions/string-_3e_utf8.md#definition__r7rs__string-_3e_utf8);
[`get-output-bytevector`](../../r7rs/definitions/get-output-bytevector.md#definition__r7rs__get-output-bytevector);


#### Referent definitions as output (recursive)

[`bytevector`](../../r7rs/definitions/bytevector.md#definition__r7rs__bytevector);
[`make-bytevector`](../../r7rs/definitions/make-bytevector.md#definition__r7rs__make-bytevector);

Note:  These definitions produce an output that is a sub-type.


#### Description

> __Bytevectors__ represent blocks of binary data.
> They are fixed-length sequences of bytes, where
> a __byte__ is an exact integer in the range from `0` to `255` inclusive.
> A bytevector is typically more space-efficient than a vector
> containing the same values.
> 
> The __length__ of a bytevector is the number of elements that it
> contains.  This number is a non-negative integer that is fixed when
> the bytevector is created.  The __valid indexes__ of
> a bytevector are the exact non-negative integers less than the length of the
> bytevector, starting at index zero as with vectors.
> 
> Bytevectors are written using the notation `#u8(byte ...)`.
> For example, a bytevector of length `3` containing the byte `0` in element
> `0`, the byte `10` in element `1`, and the byte `5` in
> element `2` can be written as follows:
> 
> ````
> #u8(0 10 5)
> ````
> 
> Bytevector constants are self-evaluating, so they do not need to be quoted in programs.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Predicate

```
|bytevector?|
```


#### Categories

[`r7rs:types-disjoint`](../../r7rs/categories/r7rs_3a_types-disjoint.md#category__r7rs__r7rs_3a_types-disjoint);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

