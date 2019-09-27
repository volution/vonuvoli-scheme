

<a id='definition__r7rs__get-environment-variables'></a>

# `get-environment-variables` -- `r7rs` Definition


<a id='definition__r7rs__get-environment-variables__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__get-environment-variables__implemented-by'></a>

#### Implemented by

 * [`get-environment-variables`](../../vonuvoli/definitions/get-environment-variables.md#definition__vonuvoli__get-environment-variables) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__get-environment-variables__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (assoc-list))`
   * inputs: none;
   * output: a value of type [`assoc-list`](../../r7rs/types/assoc-list.md#type__r7rs__assoc-list);


<a id='definition__r7rs__get-environment-variables__exports'></a>

#### Exports

 * [`scheme:process-context`](../../r7rs/exports/scheme_3a_process-context.md#export__r7rs__scheme_3a_process-context) -- `(scheme process-context)`;


<a id='definition__r7rs__get-environment-variables__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__get-environment-variables__description'></a>

#### Description

> ````
> (get-environment-variables)
> ````
> 
> 
> Returns the names and values of all the environment variables as an
> alist, where the car of each entry is the name of an environment
> variable and the cdr is its value, both as strings.  The order of the list is unspecified.
> It is an error to mutate any of these strings or the alist itself.
> 
> ````
> (get-environment-variables) ===> (("USER" . "root") ("HOME" . "/"))
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__get-environment-variables__referenced-types'></a>

#### Referenced-types

 * [`assoc-list`](../../r7rs/types/assoc-list.md#type__r7rs__assoc-list);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

