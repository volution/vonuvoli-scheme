

<a id='definition__r7rs__eval'></a>

# `eval` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|eval-expression| |eval-environment|) |->| (|any|))`
   * inputs:
     * a value of type [`eval-expression`](../../r7rs/types/eval-expression.md#type__r7rs__eval-expression);
     * a value of type [`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Referenced types

[`eval-expression`](../../r7rs/types/eval-expression.md#type__r7rs__eval-expression);
[`eval-environment`](../../r7rs/types/eval-environment.md#type__r7rs__eval-environment);
[`any`](../../r7rs/types/any.md#type__r7rs__any);


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


#### Categories

[`r7rs:eval`](../../r7rs/categories/r7rs_3a_eval.md#category__r7rs__r7rs_3a_eval);
[`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);
[`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

