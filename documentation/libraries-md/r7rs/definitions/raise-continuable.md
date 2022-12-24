

<a id='definition__r7rs__raise-continuable'></a>

# `raise-continuable` -- `r7rs` Definition


<a id='definition__r7rs__raise-continuable__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__raise-continuable__implemented-by'></a>

#### Implemented by

 * [`raise-continuable`](../../vonuvoli/definitions/raise-continuable.md#definition__vonuvoli__raise-continuable) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__raise-continuable__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((any) -> (exception))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`exception`](../../r7rs/types/exception.md#type__r7rs__exception);


<a id='definition__r7rs__raise-continuable__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__raise-continuable__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__raise-continuable__description'></a>

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


<a id='definition__r7rs__raise-continuable__referenced-types'></a>

#### Referenced-types

 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`exception`](../../r7rs/types/exception.md#type__r7rs__exception);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

