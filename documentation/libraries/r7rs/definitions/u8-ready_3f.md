

<a id='definition__r7rs__u8-ready_3f'></a>

# `u8-ready?` -- `r7rs` Definition


<a id='definition__r7rs__u8-ready_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__r7rs__u8-ready_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (boolean))`
   * inputs: none;
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
 * `((binary-input-port-eof) -> (true))`
   * input: a value of type [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((binary-input-port-open) -> (boolean))`
   * input: a value of type [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__u8-ready_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__u8-ready_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__u8-ready_3f__description'></a>

#### Description

> ````
> (u8-ready?)
> (u8-ready? port)
> ````
> 
> 
> Returns `#t` if a byte is ready on the binary input `port`
> and returns `#f` otherwise.  If `u8-ready?` returns
> `#t` then the next `read-u8` operation on the given
> `port` is guaranteed not to hang.  If the `port` is at end of
> file then `u8-ready?` returns `#t`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__u8-ready_3f__referenced-types'></a>

#### Referenced-types

 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
 * [`binary-input-port-eof`](../../r7rs/types/binary-input-port-eof.md#type__r7rs__binary-input-port-eof);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`binary-input-port-open`](../../r7rs/types/binary-input-port-open.md#type__r7rs__binary-input-port-open);


<a id='definition__r7rs__u8-ready_3f__categories'></a>

#### Categories

 * [`vs:ports:input`](../../r7rs/categories/vs_3a_ports_3a_input.md#category__r7rs__vs_3a_ports_3a_input);
 * [`vs:bytes`](../../r7rs/categories/vs_3a_bytes.md#category__r7rs__vs_3a_bytes);


<a id='definition__r7rs__u8-ready_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs:ports`](../../r7rs/categories/vs_3a_ports.md#category__r7rs__vs_3a_ports);
 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

