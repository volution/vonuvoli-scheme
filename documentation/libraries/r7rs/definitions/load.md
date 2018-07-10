

<a id='definition__r7rs__load'></a>

# `load` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|path-string|) |->| (|undefined|))`
   * input: a value of type [`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * `((|path-string| |eval-environment|) |->| (|undefined|))`
   * inputs:
     * a value of type [`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
     * a value of type [`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


#### Referenced types

[`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
[`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
[`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);


#### Description

> ````
> (load filename)
> (load filename environment-specifier)
> ````
> 
> 
> **Domain**:  It is an error if `filename` is not a string.
> 
> An implementation-dependent operation is used to transform
> `filename` into the name of an existing file
> containing Scheme source code.  The `load` procedure reads
> expressions and definitions from the file and evaluates them
> sequentially in the environment specified by `environment-specifier`.
> If `environment-specifier` is omitted, `(interaction-environment)`
> is assumed.
> 
> It is unspecified whether the results of the expressions
> are printed.  The `load` procedure does not affect the values
> returned by `current-input-port` and `current-output-port`.
> It returns an unspecified value.
> 
> 
> **Rationale**:  For portability, `load` must operate on source files.
> Its operation on other kinds of files necessarily varies among
> implementations.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:load`](../../r7rs/categories/r7rs_3a_load.md#category__r7rs__r7rs_3a_load);
[`vs:compiler`](../../r7rs/categories/vs_3a_compiler.md#category__r7rs__vs_3a_compiler);
[`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

