

<a id='definition__r7rs__interaction-environment'></a>

# `interaction-environment` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|eval-environment|))`
   * inputs: none;
   * output: a value of type [`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);


#### Referenced types

[`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);


#### Description

> ````
> (interaction-environment)
> ````
> 
> 
> This procedure returns a specifier for a mutable environment that contains an
> implementation-defined set of bindings, typically a superset of
> those exported by `(scheme base)`.  The intent is that this procedure
> will return the environment in which the implementation would evaluate
> expressions entered by the user into a REPL.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:r5rs`](../../r7rs/categories/r7rs_3a_r5rs.md#category__r7rs__r7rs_3a_r5rs);
[`r7rs:repl`](../../r7rs/categories/r7rs_3a_repl.md#category__r7rs__r7rs_3a_repl);
[`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);
[`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

