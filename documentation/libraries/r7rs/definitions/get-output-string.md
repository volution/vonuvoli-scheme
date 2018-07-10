

<a id='definition__r7rs__get-output-string'></a>

# `get-output-string` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|string-output-port|) |->| (|string|))`
   * input: a value of type [`string-output-port`](../../r7rs/types/string-output-port.md#type__r7rs__string-output-port);
   * output: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);


#### Referenced types

[`string-output-port`](../../r7rs/types/string-output-port.md#type__r7rs__string-output-port);
[`string`](../../r7rs/types/string.md#type__r7rs__string);


#### Description

> ````
> (get-output-string port)
> ````
> 
> 
> **Domain**:  It is an error if `port` was not created with
> `open-output-string`.
> 
> Returns a string consisting of the
> characters that have been output to the port so far in the order they
> were output.
> If the result string is modified, the effect is unspecified.
> 
> ````
> (parameterize
>     ((current-output-port
>       (open-output-string)))
>     (display "piece")
>     (display " by piece ")
>     (display "by piece.")
>     (newline)
>     (get-output-string (current-output-port)))
> ===> "piece by piece by piece.\n"
> ````
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

