

<a id='definition__r7rs__textual-port_3f'></a>

# `textual-port?` -- `r7rs` Definitions


#### Kind

`predicate`;


#### Procedure signature

Procedure variants:
 * `((|textual-port|) |->| (|true|))`
   * input: a value of type [`textual-port`](../../r7rs/types/textual-port.md#type__r7rs__textual-port);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|port|) |->| (|false|))`
   * input: a value of type [`port`](../../r7rs/types/port.md#type__r7rs__port);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
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

[`textual-port`](../../r7rs/types/textual-port.md#type__r7rs__textual-port);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`port`](../../r7rs/types/port.md#type__r7rs__port);
[`false`](../../r7rs/types/false.md#type__r7rs__false);
[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Description

> Please refer to [`port?`](../../r7rs/definitions/port_3f.md#definition__r7rs__port_3f).


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:ports`](../../r7rs/categories/vs_3a_ports.md#category__r7rs__vs_3a_ports);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

