

<a id='definition__r7rs__open-output-string'></a>

# `open-output-string` -- `r7rs` Definition


<a id='definition__r7rs__open-output-string__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__open-output-string__implemented-by'></a>

#### Implemented by

 * [`open-output-string`](../../vonuvoli/definitions/open-output-string.md#definition__vonuvoli__open-output-string) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__open-output-string__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (string-output-port))`
   * inputs: none;
   * output: a value of type [`string-output-port`](../../r7rs/types/string-output-port.md#type__r7rs__string-output-port);


<a id='definition__r7rs__open-output-string__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__open-output-string__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__open-output-string__description'></a>

#### Description

> ````
> (open-output-string)
> ````
> 
> 
> Returns a textual output port that will accumulate characters for
> retrieval by `get-output-string`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__open-output-string__referenced-types'></a>

#### Referenced-types

 * [`string-output-port`](../../r7rs/types/string-output-port.md#type__r7rs__string-output-port);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

