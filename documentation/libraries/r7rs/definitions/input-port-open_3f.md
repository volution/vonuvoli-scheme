

<a id='definition__r7rs__input-port-open_3f'></a>

# `input-port-open?` -- `r7rs` Definition


<a id='definition__r7rs__input-port-open_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__r7rs__input-port-open_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((input-port-open) -> (true))`
   * input: a value of type [`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((input-port) -> (false))`
   * input: a value of type [`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((port) -> (false))`
   * input: a value of type [`port`](../../r7rs/types/port.md#type__r7rs__port);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((any) -> (false))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((any ...) -> (boolean))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `vonuvoli`


<a id='definition__r7rs__input-port-open_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__input-port-open_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__input-port-open_3f__description'></a>

#### Description

> ````
> (input-port-open? port)
> (output-port-open? port)
> ````
> 
> 
> Returns `#t` if `port` is still open and capable of
> performing input or output, respectively, and `#f` otherwise.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__input-port-open_3f__referenced-types'></a>

#### Referenced-types

 * [`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`port`](../../r7rs/types/port.md#type__r7rs__port);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__input-port-open_3f__categories'></a>

#### Categories

 * [`vs:ports:input`](../../vonuvoli/categories/vs_3a_ports_3a_input.md#category__vonuvoli__vs_3a_ports_3a_input);
 * [`vs:ports:open`](../../vonuvoli/categories/vs_3a_ports_3a_open.md#category__vonuvoli__vs_3a_ports_3a_open);


<a id='definition__r7rs__input-port-open_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs:ports`](../../vonuvoli/categories/vs_3a_ports.md#category__vonuvoli__vs_3a_ports);
 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

