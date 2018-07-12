

<a id='definition__r7rs__scheme-report-environment'></a>

# `scheme-report-environment` -- `r7rs` Definitions


<a id='definition__r7rs__scheme-report-environment__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__scheme-report-environment__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((eval-environment-version) -> (eval-environment))`
   * input: a value of type [`eval-environment-version`](../../r7rs/types/eval-environment-version.md#type__r7rs__eval-environment-version);
   * output: a value of type [`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);


<a id='definition__r7rs__scheme-report-environment__exports'></a>

#### Exports

 * [`scheme:r5rs`](../../r7rs/exports/scheme_3a_r5rs.md#export__r7rs__scheme_3a_r5rs);


<a id='definition__r7rs__scheme-report-environment__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__scheme-report-environment__description'></a>

#### Description

> ````
> (scheme-report-environment version)
> ````
> 
> 
> If `version` is equal to `5`,
> corresponding to __R5RS__,
> `scheme-report-environment` returns a specifier for an
> environment that contains only the bindings
> defined in the __R5RS__ library.
> Implementations must support this value of `version`.
> 
> Implementations may also support other values of `version`, in which
> case they return a specifier for an environment containing bindings corresponding to the specified version of the report.
> If `version`
> is neither `5` nor another value supported by
> the implementation, an error is signaled.
> 
> The effect of defining or assigning (through the use of `eval`)
> an identifier bound in a `scheme-report-environment` (for example
> `car`) is unspecified.  Thus both the environment and the bindings
> it contains may be immutable.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__scheme-report-environment__referenced-types'></a>

#### Referenced-types

 * [`eval-environment-version`](../../r7rs/types/eval-environment-version.md#type__r7rs__eval-environment-version);
 * [`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);


<a id='definition__r7rs__scheme-report-environment__categories'></a>

#### Categories

 * [`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);
 * [`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);


<a id='definition__r7rs__scheme-report-environment__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

