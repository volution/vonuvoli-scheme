

<a id='definition__r7rs__with-input-from-file'></a>

# `with-input-from-file` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|path-string| |procedure|) |->| (|any|))`
   * inputs:
     * a value of type [`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
     * a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Referenced types

[`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
[`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
[`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Description

> ````
> (with-input-from-file string thunk)
> (with-output-to-file string thunk)
> ````
> 
> 
> The file is opened for input or output
> as if by `open-input-file` or `open-output-file`,
> and the new port is made to be the value returned by
> `current-input-port` or `current-output-port`
> (as used by `(read)`, `(write obj)`, and so forth).
> The `thunk` is then called with no arguments.  When the `thunk` returns,
> the port is closed and the previous default is restored.
> It is an error if `thunk` does not accept zero arguments.
> Both procedures return the values yielded by `thunk`.
> If an escape procedure
> is used to escape from the continuation of these procedures, they
> behave exactly as if the current input or output port had been bound
> dynamically with `parameterize`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:file`](../../r7rs/categories/r7rs_3a_file.md#category__r7rs__r7rs_3a_file);
[`vs:parameters`](../../r7rs/categories/vs_3a_parameters.md#category__r7rs__vs_3a_parameters);
[`vs:functions`](../../r7rs/categories/vs_3a_functions.md#category__r7rs__vs_3a_functions);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

