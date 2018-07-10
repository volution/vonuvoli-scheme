

<a id='definition__r7rs__write-shared'></a>

# `write-shared` -- `r7rs` Definitions


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
> (write-shared obj)
> (write-shared obj port)
> ````
> 
> 
> The `write-shared` procedure is the same as `write`, except that
> shared structure must be represented using datum labels for all pairs
> and vectors that appear more than once in the output.
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

