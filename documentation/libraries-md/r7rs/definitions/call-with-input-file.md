

<a id='definition__r7rs__call-with-input-file'></a>

# `call-with-input-file` -- `r7rs` Definition


<a id='definition__r7rs__call-with-input-file__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__call-with-input-file__implemented-by'></a>

#### Implemented by

 * [`call-with-input-file`](../../vonuvoli/definitions/call-with-input-file.md#definition__vonuvoli__call-with-input-file) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__call-with-input-file__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((path-string procedure-1) -> (any))`
   * inputs:
     * a value of type [`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
     * a value of type [`procedure-1`](../../r7rs/types/procedure-1.md#type__r7rs__procedure-1);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__call-with-input-file__exports'></a>

#### Exports

 * [`scheme:file`](../../r7rs/exports/scheme_3a_file.md#export__r7rs__scheme_3a_file) -- `(scheme file)`;


<a id='definition__r7rs__call-with-input-file__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__call-with-input-file__description'></a>

#### Description

> ````
> (call-with-input-file string proc)
> (call-with-output-file string proc)
> ````
> 
> 
> **Domain**:  It is an error if `proc` does not accept one argument.
> 
> These procedures obtain a
> textual port obtained by opening the named file for input or output
> as if by `open-input-file` or `open-output-file`.
> The port and `proc` are then passed to a procedure equivalent
> to `call-with-port`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__call-with-input-file__referenced-types'></a>

#### Referenced-types

 * [`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
 * [`procedure-1`](../../r7rs/types/procedure-1.md#type__r7rs__procedure-1);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

