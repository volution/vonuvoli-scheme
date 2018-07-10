

<a id='definition__r7rs__eof-object_3f'></a>

# `eof-object?` -- `r7rs` Definitions


#### Kind

`predicate`;


#### Procedure signature

Procedure variants:
 * `((|eof-object|) |->| (|true|))`
   * input: a value of type [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
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

[`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`false`](../../r7rs/types/false.md#type__r7rs__false);
[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Description

> ````
> (eof-object? obj)
> ````
> 
> 
> Returns `#t` if `obj` is an end-of-file object, otherwise returns
> `#f`.  The precise set of end-of-file objects will vary among
> implementations, but in any case no end-of-file object will ever be an object
> that can be read in using `read`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:ports`](../../r7rs/categories/vs_3a_ports.md#category__r7rs__vs_3a_ports);
[`vs:globals`](../../r7rs/categories/vs_3a_globals.md#category__r7rs__vs_3a_globals);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

