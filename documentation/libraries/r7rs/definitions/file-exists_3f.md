

<a id='definition__r7rs__file-exists_3f'></a>

# `file-exists?` -- `r7rs` Definitions


<a id='definition__r7rs__file-exists_3f__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__file-exists_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((path-string) -> (boolean))`
   * input: a value of type [`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__file-exists_3f__exports'></a>

#### Exports

 * [`scheme:file`](../../r7rs/exports/scheme_3a_file.md#export__r7rs__scheme_3a_file);


<a id='definition__r7rs__file-exists_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__file-exists_3f__description'></a>

#### Description

> ````
> (file-exists? filename)
> ````
> 
> 
> **Domain**:  It is an error if `filename` is not a string.
> 
> The `file-exists?` procedure returns
> `#t` if the named file exists at the time the procedure is called,
> and `#f` otherwise.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__file-exists_3f__referenced-types'></a>

#### Referenced-types

 * [`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__file-exists_3f__categories'></a>

#### Categories

 * [`vs:file-system`](../../r7rs/categories/vs_3a_file-system.md#category__r7rs__vs_3a_file-system);


<a id='definition__r7rs__file-exists_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

