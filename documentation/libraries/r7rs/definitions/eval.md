

<a id='definition__r7rs__eval'></a>

# `eval` -- `r7rs` Definitions


<a id='definition__r7rs__eval__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__eval__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((eval-expression eval-environment) -> (any))`
   * inputs:
     * a value of type [`eval-expression`](../../r7rs/types/eval-expression.md#type__r7rs__eval-expression);
     * a value of type [`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__eval__exports'></a>

#### Exports

 * [`scheme:eval`](../../r7rs/exports/scheme_3a_eval.md#export__r7rs__scheme_3a_eval) -- `(scheme eval)`;


<a id='definition__r7rs__eval__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__eval__description'></a>

#### Description

> ````
> (eval expr-or-def environment-specifier)
> ````
> 
> 
> If `expr-or-def` is an expression, it is evaluated in the
> specified environment and its values are returned.
> If it is a definition, the specified identifier(s) are defined in the specified
> environment, provided the environment is not immutable.
> Implementations may extend `eval` to allow other objects.
> 
> ````
> (eval '(* 7 3) (environment '(scheme base)))
>                                                    ===>  21
> 
> (let ((f (eval '(lambda (f x) (f x x))
>                (null-environment 5))))
>   (f + 10))
>                                                    ===>  20
> (eval '(define foo 32)
>       (environment '(scheme base)))
>                                                    ===>  #error
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__eval__referenced-types'></a>

#### Referenced-types

 * [`eval-expression`](../../r7rs/types/eval-expression.md#type__r7rs__eval-expression);
 * [`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__eval__categories'></a>

#### Categories

 * [`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);
 * [`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);


<a id='definition__r7rs__eval__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

