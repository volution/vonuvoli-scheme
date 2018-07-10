

<a id='definition__r7rs__port_3f'></a>

# `port?` -- `r7rs` Definitions


#### Kind

`type-predicate`;


#### Procedure signature

Procedure variants:
 * `((|port|) |->| (|true|))`
   * input: a value of type [`port`](../../r7rs/types/port.md#type__r7rs__port);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|any|) |->| (|false|))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|any| |...|) |->| (|boolean|))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `|vonuvoli|`


#### Referenced types

[`port`](../../r7rs/types/port.md#type__r7rs__port);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`false`](../../r7rs/types/false.md#type__r7rs__false);
[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Description

> ````
> (input-port? obj)
> (output-port? obj)
> (textual-port? obj)
> (binary-port? obj)
> (port? obj)
> ````
> 
> 
> These procedures return `#t` if `obj` is an input port, output port,
> textual port, binary port, or any
> kind of port, respectively.  Otherwise they return `#f`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:ports`](../../r7rs/categories/vs_3a_ports.md#category__r7rs__vs_3a_ports);
[`vs:types`](../../r7rs/categories/vs_3a_types.md#category__r7rs__vs_3a_types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

