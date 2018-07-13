

<a id='definition__r7rs__with-exception-handler'></a>

# `with-exception-handler` -- `r7rs` Definitions


<a id='definition__r7rs__with-exception-handler__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__with-exception-handler__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(((handler . procedure) (thunk . procedure)) -> (any))`
   * inputs:
     * `handler` of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
     * `thunk` of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__with-exception-handler__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__with-exception-handler__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__with-exception-handler__description'></a>

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


<a id='definition__r7rs__with-exception-handler__examples'></a>

#### Examples

<a id='definition__r7rs__with-exception-handler__example-1'></a>

##### Examples 1

 1. evaluating:
````
(call-with-current-continuation
 (lambda (k)
  (with-exception-handler
   (lambda (x)
    (display "condition: ")
    (write x)
    (newline)
    (k 'exception))
   (lambda ()
    (+ 1 (raise 'an-error))))))
````
 2. stdout output:
````
condition: an-error
````
 3. raises:
````
exception
````

<a id='definition__r7rs__with-exception-handler__example-2'></a>

##### Examples 2

 1. evaluating:
````
(with-exception-handler
 (lambda (x)
  (display "something went wrong\n"))
 (lambda ()
  (+ 1 (raise 'an-error))))
````
 2. stdout output:
````
something went wrong
````


<a id='definition__r7rs__with-exception-handler__referenced-types'></a>

#### Referenced-types

 * [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__with-exception-handler__categories'></a>

#### Categories

 * [`vs:errors`](../../r7rs/categories/vs_3a_errors.md#category__r7rs__vs_3a_errors);
 * [`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);


<a id='definition__r7rs__with-exception-handler__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

