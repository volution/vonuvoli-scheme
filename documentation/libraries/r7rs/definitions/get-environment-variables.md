

<a id='definition__r7rs__get-environment-variables'></a>

# `get-environment-variables` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|assoc-list|))`
   * inputs: none;
   * output: a value of type [`assoc-list`](../../r7rs/types/assoc-list.md#type__r7rs__assoc-list);


#### Referenced types

[`assoc-list`](../../r7rs/types/assoc-list.md#type__r7rs__assoc-list);


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


#### Categories

[`r7rs:process-context`](../../r7rs/categories/r7rs_3a_process-context.md#category__r7rs__r7rs_3a_process-context);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

