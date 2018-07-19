

<a id='definition__r7rs__environment'></a>

# `environment` -- `r7rs` Definition


<a id='definition__r7rs__environment__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__environment__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (eval-environment))`
   * inputs: none;
   * output: a value of type [`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);
 * `((eval-environment-import ...) -> (eval-environment))`
   * inputs:
     * a value of type [`eval-environment-import`](../../r7rs/types/eval-environment-import.md#type__r7rs__eval-environment-import);
     * `...` (i.e. variadic);
   * output: a value of type [`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);


<a id='definition__r7rs__environment__exports'></a>

#### Exports

 * [`scheme:eval`](../../r7rs/exports/scheme_3a_eval.md#export__r7rs__scheme_3a_eval) -- `(scheme eval)`;


<a id='definition__r7rs__environment__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__environment__description'></a>

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


<a id='definition__r7rs__environment__referenced-types'></a>

#### Referenced-types

 * [`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);
 * [`eval-environment-import`](../../r7rs/types/eval-environment-import.md#type__r7rs__eval-environment-import);


<a id='definition__r7rs__environment__categories'></a>

#### Categories

 * [`vs:evaluator`](../../vonuvoli/categories/vs_3a_evaluator.md#category__vonuvoli__vs_3a_evaluator);
 * [`vs:unsupported`](../../vonuvoli/categories/vs_3a_unsupported.md#category__vonuvoli__vs_3a_unsupported);


<a id='definition__r7rs__environment__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

