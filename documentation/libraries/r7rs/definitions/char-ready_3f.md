

<a id='definition__r7rs__char-ready_3f'></a>

# `char-ready?` -- `r7rs` Definitions


#### Kind

`predicate`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|boolean|))`
   * inputs: none;
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
 * `((|textual-input-port-eof|) |->| (|true|))`
   * input: a value of type [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|textual-input-port-open|) |->| (|boolean|))`
   * input: a value of type [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Referenced types

[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
[`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);


#### Description

> ````
> (char-ready?)
> (char-ready? port)
> ````
> 
> 
> Returns `#t` if a character is ready on the textual input `port` and
> returns `#f` otherwise.  If `char-ready` returns `#t` then
> the next `read-char` operation on the given `port` is guaranteed
> not to hang.  If the `port` is at end of file then `char-ready?`
> returns `#t`.
> 
> **Rationale**:  The `char-ready?` procedure exists to make it possible for a program to
> accept characters from interactive ports without getting stuck waiting for
> input.  Any input editors associated with such ports must ensure that
> characters whose existence has been asserted by `char-ready?` cannot
> be removed from the input.  If `char-ready?` were to return `#f` at end of
> file, a port at end of file would be indistinguishable from an interactive
> port that has no ready characters.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:ports:input`](../../r7rs/categories/vs_3a_ports_3a_input.md#category__r7rs__vs_3a_ports_3a_input);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);
[`vs:characters`](../../r7rs/categories/vs_3a_characters.md#category__r7rs__vs_3a_characters);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

