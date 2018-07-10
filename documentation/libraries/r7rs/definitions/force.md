

<a id='definition__r7rs__force'></a>

# `force` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|promise|) |->| (|any|))`
   * input: a value of type [`promise`](../../r7rs/types/promise.md#type__r7rs__promise);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Referenced types

[`promise`](../../r7rs/types/promise.md#type__r7rs__promise);
[`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Description

> ````
> (force promise)
> ````
> 
> 
> The `force` procedure forces the value of a `promise` created
> by `delay`, `delay-force`, or `make-promise`.
> If no value has been computed for the promise, then a value is
> computed and returned.  The value of the promise must be cached (or
> "memoized") so that if it is forced a second time, the previously
> computed value is returned.
> Consequently, a delayed expression is evaluated using the parameter
> values and exception handler of the call to `force` which first
> requested its value.
> If `promise` is not a promise, it may be returned unchanged.
> 
> ````
> (force (delay (+ 1 2)))   ===>  3
> (let ((p (delay (+ 1 2))))
>   (list (force p) (force p)))
>                                ===>  (3 3)
> 
> (define integers
>   (letrec ((next
>             (lambda (n)
>               (delay (cons n (next (+ n 1)))))))
>     (next 0)))
> (define head
>   (lambda (stream) (car (force stream))))
> (define tail
>   (lambda (stream) (cdr (force stream))))
> 
> (head (tail (tail integers)))
>                                ===>  2
> ````
> 
> The following example is a mechanical transformation of a lazy
> stream-filtering algorithm into Scheme.  Each call to a constructor is
> wrapped in `delay`, and each argument passed to a deconstructor is
> wrapped in `force`.  The use of `(delay-force ...)` instead of
> `(delay (force ...))` around the body of the procedure ensures that an
> ever-growing sequence of pending promises does not
> exhaust available storage,
> because `force` will in effect force such sequences iteratively.
> 
> ````
> (define (stream-filter p? s)
>   (delay-force
>    (if (null? (force s))
>        (delay '())
>        (let ((h (car (force s)))
>              (t (cdr (force s))))
>          (if (p? h)
>              (delay (cons h (stream-filter p? t)))
>              (stream-filter p? t))))))
> 
> (head (tail (tail (stream-filter odd? integers))))
>                                ===> 5
> ````
> 
> The following examples are not intended to illustrate good programming
> style, as `delay`, `force`, and `delay-force` are mainly intended
> for programs written in the functional style.
> However, they do illustrate the property that only one value is
> computed for a promise, no matter how many times it is forced.
> 
> ````
> (define count 0)
> (define p
>   (delay (begin (set! count (+ count 1))
>                 (if (> count x)
>                     count
>                     (force p)))))
> (define x 5)
> p                     ===>  #promise
> (force p)             ===>  6
> p                     ===>  #promise
> (begin (set! x 10)
>        (force p))     ===>  6
> ````
> 
> Various extensions to this semantics of `delay`, `force` and
> `delay-force` are supported in some implementations:
> 
>   * Calling `force` on an object that is not a promise may simply
> return the object.
> 
>   * It may be the case that there is no means by which a promise can be
> operationally distinguished from its forced value.  That is, expressions
> like the following may evaluate to either `#t` or to `#f`,
> depending on the implementation:
> 
> ````
> (eqv? (delay 1) 1)          ===>  #unspecified
> (pair? (delay (cons 1 2)))  ===>  #unspecified
> ````
> 
>   * Implementations may implement "implicit forcing", where
> the value of a promise is forced by procedures
> that operate only on arguments of a certain type, like `cdr`
> and `*`.  However, procedures that operate uniformly on their
> arguments, like `list`, must not force them.
> 
> ````
> (+ (delay (* 3 7)) 13)  ===>  #unspecified
> (car
>   (list (delay (* 3 7)) 13))    ===> #promise
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:lazy`](../../r7rs/categories/r7rs_3a_lazy.md#category__r7rs__r7rs_3a_lazy);
[`vs:promises`](../../r7rs/categories/vs_3a_promises.md#category__r7rs__vs_3a_promises);
[`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

