

<a id='definition__r7rs__write'></a>

# `write` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|value|) |->| (|void|))`
   * input: a value of type [`value`](../../r7rs/types/value.md#type__r7rs__value);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `((|value| |textual-output-port-open|) |->| (|void|))`
   * inputs:
     * a value of type [`value`](../../r7rs/types/value.md#type__r7rs__value);
     * a value of type [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);


#### Referenced types

[`value`](../../r7rs/types/value.md#type__r7rs__value);
[`void`](../../r7rs/types/void.md#type__r7rs__void);
[`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);


#### Description

> ````
> (write obj)
> (write obj port)
> ````
> 
> 
> Writes a representation of `obj` to the given textual output
> `port`.  Strings
> that appear in the written representation are enclosed in quotation marks, and
> within those strings backslash and quotation mark characters are
> escaped by backslashes.  Symbols that contain non-ASCII characters
> are escaped with vertical lines.
> Character objects are written using the `#\` notation.
> 
> If `obj` contains cycles which would cause an infinite loop using
> the normal written representation, then at least the objects that form
> part of the cycle must be represented using datum labels as described
> in section on datum labels.  Datum labels must not be used if there
> are no cycles.
> 
> Implementations may support extended syntax to represent record types or
> other types that do not have datum representations.
> 
> The `write` procedure returns an unspecified value.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:write`](../../r7rs/categories/r7rs_3a_write.md#category__r7rs__r7rs_3a_write);
[`vs:ports:output`](../../r7rs/categories/vs_3a_ports_3a_output.md#category__r7rs__vs_3a_ports_3a_output);
[`vs:ports:values`](../../r7rs/categories/vs_3a_ports_3a_values.md#category__r7rs__vs_3a_ports_3a_values);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

