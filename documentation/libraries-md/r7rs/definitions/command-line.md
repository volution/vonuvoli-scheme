

<a id='definition__r7rs__command-line'></a>

# `command-line` -- `r7rs` Definition


<a id='definition__r7rs__command-line__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__command-line__implemented-by'></a>

#### Implemented by

 * [`command-line`](../../vonuvoli/definitions/command-line.md#definition__vonuvoli__command-line) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__command-line__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (list-proper-not-null))`
   * inputs: none;
   * output: a value of type [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);


<a id='definition__r7rs__command-line__exports'></a>

#### Exports

 * [`scheme:process-context`](../../r7rs/exports/scheme_3a_process-context.md#export__r7rs__scheme_3a_process-context) -- `(scheme process-context)`;


<a id='definition__r7rs__command-line__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__command-line__description'></a>

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


<a id='definition__r7rs__command-line__referenced-types'></a>

#### Referenced-types

 * [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

