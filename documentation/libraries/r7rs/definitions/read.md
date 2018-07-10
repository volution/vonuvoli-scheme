

<a id='definition__r7rs__read'></a>

# `read` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|value-or-eof|))`
   * inputs: none;
   * output: a value of type [`value-or-eof`](../../r7rs/types/value-or-eof.md#type__r7rs__value-or-eof);
 * `((|textual-input-port-eof|) |->| (|eof-object|))`
   * input: a value of type [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
   * output: a value of type [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * `((|textual-input-port-open|) |->| (|value-or-eof|))`
   * input: a value of type [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);
   * output: a value of type [`value-or-eof`](../../r7rs/types/value-or-eof.md#type__r7rs__value-or-eof);


#### Referenced types

[`value-or-eof`](../../r7rs/types/value-or-eof.md#type__r7rs__value-or-eof);
[`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
[`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
[`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);


#### Description

> ````
> (read)
> (read port)
> ````
> 
> 
> The `read` procedure converts external representations of Scheme objects into the
> objects themselves.  That is, it is a parser for the non-terminal
> `<datum>` (see sections on external representations and
> on pairs and lists).  It returns the next
> object parsable from the given textual input `port`, updating
> `port` to point to
> the first character past the end of the external representation of the object.
> 
> Implementations may support extended syntax to represent record types or
> other types that do not have datum representations.
> 
> If an end of file is encountered in the input before any
> characters are found that can begin an object, then an end-of-file
> object is returned.  The port remains open, and further attempts
> to read will also return an end-of-file object.  If an end of file is
> encountered after the beginning of an object's external representation,
> but the external representation is incomplete and therefore not parsable,
> an error that satisfies `read-error?` is signaled.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:read`](../../r7rs/categories/r7rs_3a_read.md#category__r7rs__r7rs_3a_read);
[`vs:ports:input`](../../r7rs/categories/vs_3a_ports_3a_input.md#category__r7rs__vs_3a_ports_3a_input);
[`vs:ports:values`](../../r7rs/categories/vs_3a_ports_3a_values.md#category__r7rs__vs_3a_ports_3a_values);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

