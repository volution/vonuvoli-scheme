

<a id='definition__r7rs__open-input-string'></a>

# `open-input-string` -- `r7rs` Definition


<a id='definition__r7rs__open-input-string__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__open-input-string__implemented-by'></a>

#### Implemented by

 * [`open-input-string`](../../vonuvoli/definitions/open-input-string.md#definition__vonuvoli__open-input-string) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__open-input-string__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((string) -> (string-input-port))`
   * input: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
   * output: a value of type [`string-input-port`](../../r7rs/types/string-input-port.md#type__r7rs__string-input-port);


<a id='definition__r7rs__open-input-string__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__open-input-string__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__open-input-string__description'></a>

#### Description

> ````
> (open-input-string string)
> ````
> 
> 
> Takes a string and returns a textual input port that delivers
> characters from the string.
> If the string is modified, the effect is unspecified.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__open-input-string__referenced-types'></a>

#### Referenced-types

 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`string-input-port`](../../r7rs/types/string-input-port.md#type__r7rs__string-input-port);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

