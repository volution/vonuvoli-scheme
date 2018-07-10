

<a id='definition__r7rs__open-input-string'></a>

# `open-input-string` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|string|) |->| (|string-input-port|))`
   * input: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
   * output: a value of type [`string-input-port`](../../r7rs/types/string-input-port.md#type__r7rs__string-input-port);


#### Referenced types

[`string`](../../r7rs/types/string.md#type__r7rs__string);
[`string-input-port`](../../r7rs/types/string-input-port.md#type__r7rs__string-input-port);


#### Description

> ````
> (open-input-string string)
> ````
> 
> 
> Takes a string and returns a textual input port that delivers
> characters from the string.
> If the string is modified, the effect is unspecified.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:ports:input`](../../r7rs/categories/vs_3a_ports_3a_input.md#category__r7rs__vs_3a_ports_3a_input);
[`vs:ports:open`](../../r7rs/categories/vs_3a_ports_3a_open.md#category__r7rs__vs_3a_ports_3a_open);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

