

<a id='definition__r7rs__dynamic-wind'></a>

# `dynamic-wind` -- `r7rs` Definition


<a id='definition__r7rs__dynamic-wind__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__dynamic-wind__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(((before . procedure) (thunk . procedure) (after . procedure)) -> (any))`
   * inputs:
     * `before` of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
     * `thunk` of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
     * `after` of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__dynamic-wind__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__dynamic-wind__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__dynamic-wind__description'></a>

#### Description

> ````
> (dynamic-wind before thunk after)
> ````
> 
> 
> Calls `thunk` without arguments, returning the result(s) of this call.
> `Before` and `after` are called, also without arguments, as required
> by the following rules.  Note that, in the absence of calls to continuations
> captured using `call-with-current-continuation`, the three arguments are
> called once each, in order.  `Before` is called whenever execution
> enters the dynamic extent of the call to `thunk` and `after` is called
> whenever it exits that dynamic extent.  The dynamic extent of a procedure
> call is the period between when the call is initiated and when it
> returns.
> The `before` and `after` thunks are called in the same dynamic
> environment as the call to `dynamic-wind`.
> In Scheme, because of `call-with-current-continuation`, the
> dynamic extent of a call is not always a single, connected time period.
> It is defined as follows:
> 
>   * The dynamic extent is entered when execution of the body of the
> called procedure begins.
> 
>   * The dynamic extent is also entered when execution is not within
> the dynamic extent and a continuation is invoked that was captured
> (using `call-with-current-continuation`) during the dynamic extent.
> 
>   * It is exited when the called procedure returns.
> 
>   * It is also exited when execution is within the dynamic extent and
> a continuation is invoked that was captured while not within the
> dynamic extent.
> 
> If a second call to `dynamic-wind` occurs within the dynamic extent of the
> call to `thunk` and then a continuation is invoked in such a way that the
> `after`s from these two invocations of `dynamic-wind` are both to be
> called, then the `after` associated with the second (inner) call to
> `dynamic-wind` is called first.
> 
> If a second call to `dynamic-wind` occurs within the dynamic extent of the
> call to `thunk` and then a continuation is invoked in such a way that the
> `before`s from these two invocations of `dynamic-wind` are both to be
> called, then the `before` associated with the first (outer) call to
> `dynamic-wind` is called first.
> 
> If invoking a continuation requires calling the `before` from one call
> to `dynamic-wind` and the `after` from another, then the `after`
> is called first.
> 
> The effect of using a captured continuation to enter or exit the dynamic
> extent of a call to `before` or `after` is unspecified.
> 
> ````
> (let ((path '())
>       (c #f))
>   (let ((add (lambda (s)
>                (set! path (cons s path)))))
>     (dynamic-wind
>       (lambda () (add 'connect))
>       (lambda ()
>         (add (call-with-current-continuation
>                (lambda (c0)
>                  (set! c c0)
>                  'talk1))))
>       (lambda () (add 'disconnect)))
>     (if (< (length path) 4)
>         (c 'talk2)
>         (reverse path))))
>     ===> (connect talk1 disconnect
>                connect talk2 disconnect)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__dynamic-wind__referenced-types'></a>

#### Referenced-types

 * [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__dynamic-wind__categories'></a>

#### Categories

 * [`vs:continuations`](../../r7rs/categories/vs_3a_continuations.md#category__r7rs__vs_3a_continuations);
 * [`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);


<a id='definition__r7rs__dynamic-wind__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

