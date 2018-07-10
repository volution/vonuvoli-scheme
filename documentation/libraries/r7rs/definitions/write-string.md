

<a id='definition__r7rs__write-string'></a>

# `write-string` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|string|) |->| (|void|))`
   * input: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `((|string| |textual-output-port-open|) |->| (|void|))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `((|string| |textual-output-port-open| |range-start|) |->| (|void|))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `((|string| |textual-output-port-open| |range-start| |range-end|) |->| (|void|))`
   * inputs:
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * a value of type [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);


#### Referenced types

[`string`](../../r7rs/types/string.md#type__r7rs__string);
[`void`](../../r7rs/types/void.md#type__r7rs__void);
[`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
[`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
[`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);


#### Description

> ````
> (write-string string)
> (write-string string port)
> (write-string string port start)
> (write-string string port start end)
> ````
> 
> 
> Writes the characters of `string`
> from `start` to `end`
> in left-to-right order to the
> textual output `port`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:ports:output`](../../r7rs/categories/vs_3a_ports_3a_output.md#category__r7rs__vs_3a_ports_3a_output);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

