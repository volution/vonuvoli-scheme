

<a id='definition__r7rs__raise-continuable'></a>

# `raise-continuable` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|any|) |->| (|exception|))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`exception`](../../r7rs/types/exception.md#type__r7rs__exception);


#### Referenced types

[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`exception`](../../r7rs/types/exception.md#type__r7rs__exception);


#### Description

> ````
> (raise-continuable obj)
> ````
> 
> 
> Raises an exception by invoking the current
> exception handler on `obj`. The handler is called with
> the same dynamic environment as the call to
> `raise-continuable`, except that: the current
> exception handler is the one that was in place when the handler being
> called was installed, and if the handler being called returns,
> then it will again become the current exception handler.  If the
> handler returns, the values it returns become the values returned by
> the call to `raise-continuable`.
> 
> ````
> (with-exception-handler
>   (lambda (con)
>     (cond
>       ((string? con)
>        (display con))
>       (else
>        (display "a warning has been issued")))
>     42)
>   (lambda ()
>     (+ (raise-continuable "should be a number")
>        23)))
> ;   prints:  should be a number
>        ===>  65
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:errors`](../../r7rs/categories/vs_3a_errors.md#category__r7rs__vs_3a_errors);
[`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);
[`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

