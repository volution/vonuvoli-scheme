

<a id='definition__r7rs__with-exception-handler'></a>

# `with-exception-handler` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(((|handler| . |procedure|) (|thunk| . |procedure|)) |->| (|any|))`
   * inputs:
     * `handler` of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
     * `thunk` of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Referenced types

[`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
[`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Description

> ````
> (with-exception-handler handler thunk)
> ````
> 
> 
> **Domain**:  It is an error if `handler` does not accept one argument.
> It is also an error if `thunk` does not accept zero arguments.
> 
> The `with-exception-handler` procedure returns the results of invoking
> `thunk`.  `Handler` is installed as the current
> exception handler
> in the dynamic environment used for the invocation of `thunk`.
> 
> ````
> (call-with-current-continuation
>  (lambda (k)
>   (with-exception-handler
>    (lambda (x)
>     (display "condition: ")
>     (write x)
>     (newline)
>     (k 'exception))
>    (lambda ()
>     (+ 1 (raise 'an-error))))))
>          ===>  exception
> ; and prints:  condition: an-error
> 
> (with-exception-handler
>  (lambda (x)
>   (display "something went wrong\n"))
>  (lambda ()
>   (+ 1 (raise 'an-error))))
> ; prints:      something went wrong
> ````
> 
> After printing, the second example then raises another exception.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:errors`](../../r7rs/categories/vs_3a_errors.md#category__r7rs__vs_3a_errors);
[`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

