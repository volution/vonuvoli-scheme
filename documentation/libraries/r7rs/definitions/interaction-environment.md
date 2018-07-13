

<a id='definition__r7rs__interaction-environment'></a>

# `interaction-environment` -- `r7rs` Definitions


<a id='definition__r7rs__interaction-environment__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__interaction-environment__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (eval-environment))`
   * inputs: none;
   * output: a value of type [`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);


<a id='definition__r7rs__interaction-environment__exports'></a>

#### Exports

 * [`scheme:repl`](../../r7rs/exports/scheme_3a_repl.md#export__r7rs__scheme_3a_repl) -- `(scheme repl)`;
 * [`scheme:r5rs`](../../r7rs/exports/scheme_3a_r5rs.md#export__r7rs__scheme_3a_r5rs) -- `(scheme r5rs)`;


<a id='definition__r7rs__interaction-environment__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__interaction-environment__description'></a>

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


<a id='definition__r7rs__interaction-environment__referenced-types'></a>

#### Referenced-types

 * [`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);


<a id='definition__r7rs__interaction-environment__categories'></a>

#### Categories

 * [`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);
 * [`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);


<a id='definition__r7rs__interaction-environment__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

