

<a id='definition__r7rs__write-shared'></a>

# `write-shared` -- `r7rs` Definition


<a id='definition__r7rs__write-shared__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__write-shared__implemented-by'></a>

#### Implemented by

 * [`write-shared`](../../vonuvoli/definitions/write-shared.md#definition__vonuvoli__write-shared) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__write-shared__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((value) -> (undefined))`
   * input: a value of type [`value`](../../r7rs/types/value.md#type__r7rs__value);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * `((value textual-output-port-open) -> (undefined))`
   * inputs:
     * a value of type [`value`](../../r7rs/types/value.md#type__r7rs__value);
     * a value of type [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


<a id='definition__r7rs__write-shared__exports'></a>

#### Exports

 * [`scheme:write`](../../r7rs/exports/scheme_3a_write.md#export__r7rs__scheme_3a_write) -- `(scheme write)`;


<a id='definition__r7rs__write-shared__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__write-shared__description'></a>

#### Description

> ````
> (write-shared obj)
> (write-shared obj port)
> ````
> 
> 
> The `write-shared` procedure is the same as `write`, except that
> shared structure must be represented using datum labels for all pairs
> and vectors that appear more than once in the output.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__write-shared__referenced-types'></a>

#### Referenced-types

 * [`value`](../../r7rs/types/value.md#type__r7rs__value);
 * [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

