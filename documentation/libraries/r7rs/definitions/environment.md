

<a id='definition__r7rs__environment'></a>

# `environment` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|eval-environment|))`
   * inputs: none;
   * output: a value of type [`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);
 * `((|eval-environment-import| |...|) |->| (|eval-environment|))`
   * inputs:
     * a value of type [`eval-environment-import`](../../r7rs/types/eval-environment-import.md#type__r7rs__eval-environment-import);
     * `...` (i.e. variadic);
   * output: a value of type [`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);


#### Referenced types

[`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);
[`eval-environment-import`](../../r7rs/types/eval-environment-import.md#type__r7rs__eval-environment-import);


#### Description

> ````
> (environment list_1 ...)
> ````
> 
> 
> This procedure returns a specifier for the environment that results by
> starting with an empty environment and then importing each `list`,
> considered as an import set, into it.  (See section on libraries for
> a description of import sets.)  The bindings of the environment
> represented by the specifier are immutable, as is the environment itself.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:eval`](../../r7rs/categories/r7rs_3a_eval.md#category__r7rs__r7rs_3a_eval);
[`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);
[`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

