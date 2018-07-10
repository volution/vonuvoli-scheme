

<a id='definition__r7rs__command-line'></a>

# `command-line` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|list-proper-not-null|))`
   * inputs: none;
   * output: a value of type [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);


#### Referenced types

[`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);


#### Description

> ````
> (command-line)
> ````
> 
> 
> Returns the command line passed to the process as a list of
> strings.  The first string corresponds to the command name, and is
> implementation-dependent.  It is an error to mutate any of these strings.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:process-context`](../../r7rs/categories/r7rs_3a_process-context.md#category__r7rs__r7rs_3a_process-context);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

