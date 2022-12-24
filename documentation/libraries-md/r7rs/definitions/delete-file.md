

<a id='definition__r7rs__delete-file'></a>

# `delete-file` -- `r7rs` Definition


<a id='definition__r7rs__delete-file__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__delete-file__implemented-by'></a>

#### Implemented by

 * [`fs-file-delete`](../../vonuvoli/definitions/fs-file-delete.md#definition__vonuvoli__fs-file-delete) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__delete-file__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((path-string) -> (undefined))`
   * input: a value of type [`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


<a id='definition__r7rs__delete-file__exports'></a>

#### Exports

 * [`scheme:file`](../../r7rs/exports/scheme_3a_file.md#export__r7rs__scheme_3a_file) -- `(scheme file)`;


<a id='definition__r7rs__delete-file__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__delete-file__description'></a>

#### Description

> ````
> (delete-file filename)
> ````
> 
> 
> **Domain**:  It is an error if `filename` is not a string.
> 
> The `delete-file` procedure deletes the
> named file if it exists and can be deleted, and returns an unspecified
> value.  If the file does not exist or cannot be deleted, an error
> that satisfies `file-error?` is signaled.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__delete-file__referenced-types'></a>

#### Referenced-types

 * [`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
 * [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

