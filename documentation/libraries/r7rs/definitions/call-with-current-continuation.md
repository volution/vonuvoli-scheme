

<a id='definition__r7rs__call-with-current-continuation'></a>

# `call-with-current-continuation` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|procedure|) |->| (|any|))`
   * input: a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Referenced types

[`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
[`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Description

> ````
> (call-with-current-continuation proc)
> (call/cc proc)
> ````
> 
> 
> **Domain**:  It is an error if `proc` does not accept one
> argument.
> 
> The procedure `call-with-current-continuation` (or its
> equivalent abbreviation `call/cc`) packages
> the current continuation (see the rationale below) as an
> __escape procedure__ and passes it as an argument to
> `proc`.
> The escape procedure is a Scheme procedure that, if it is
> later called, will abandon whatever continuation is in effect at that later
> time and will instead use the continuation that was in effect
> when the escape procedure was created.  Calling the escape procedure
> will cause the invocation of `before` and `after` thunks installed using
> `dynamic-wind`.
> 
> The escape procedure accepts the same number of arguments as the continuation to
> the original call to `call/cc`.
> Most continuations take only one value.
> Continuations created by the `call-with-values`
> procedure (including the initialization expressions of
> `define-values`, `let-values`, and `let*-values` expressions),
> take the number of values that the consumer expects.
> The continuations of all non-final expressions within a sequence
> of expressions, such as in `lambda`, `case-lambda`, `begin`,
> `let`, `let*`, `letrec`, `letrec*`, `let-values`,
> `let*-values`, `let-syntax`, `letrec-syntax`, `parameterize`,
> `guard`, `case`, `cond`, `when`, and `unless` expressions,
> take an arbitrary number of values because they discard the values passed
> to them in any event.
> The effect of passing no values or more than one value to continuations
> that were not created in one of these ways is unspecified.
> 
> 
> The escape procedure that is passed to `proc` has
> unlimited extent just like any other procedure in Scheme.  It can be stored
> in variables or data structures and can be called as many times as desired.
> However, like the `raise` and `error` procedures, it never
> returns to its caller.
> 
> The following examples show only the simplest ways in which
> `call-with-current-continuation` is used.  If all real uses were as
> simple as these examples, there would be no need for a procedure with
> the power of `call-with-current-continuation`.
> 
> ````
> (call-with-current-continuation
>   (lambda (exit)
>     (for-each (lambda (x)
>                 (if (negative? x)
>                     (exit x)))
>               '(54 0 37 -3 245 19))
>     #t))                        ===>  -3
> 
> (define list-length
>   (lambda (obj)
>     (call-with-current-continuation
>       (lambda (return)
>         (letrec ((r
>                   (lambda (obj)
>                     (cond ((null? obj) 0)
>                           ((pair? obj)
>                            (+ (r (cdr obj)) 1))
>                           (else (return #f))))))
>           (r obj))))))
> 
> (list-length '(1 2 3 4))            ===>  4
> 
> (list-length '(a b . c))            ===>  #f
> ````
> 
> **Rationale**: A common use of `call-with-current-continuation` is for
> structured, non-local exits from loops or procedure bodies, but in fact
> `call-with-current-continuation` is useful for implementing a
> wide variety of advanced control structures.
> In fact, `raise` and `guard` provide a more structured mechanism
> for non-local exits.
> 
> **Rationale**: Whenever a Scheme expression is evaluated there is a
> __continuation__ wanting the result of the expression.  The continuation
> represents an entire (default) future for the computation.  If the expression is
> evaluated at the REPL, for example, then the continuation might take the
> result, print it on the screen, prompt for the next input, evaluate it, and
> so on forever.  Most of the time the continuation includes actions
> specified by user code, as in a continuation that will take the result,
> multiply it by the value stored in a local variable, add seven, and give
> the answer to the REPL's continuation to be printed.  Normally these
> ubiquitous continuations are hidden behind the scenes and programmers do not
> think much about them.  On rare occasions, however, a programmer
> needs to deal with continuations explicitly.
> The `call-with-current-continuation` procedure allows Scheme programmers to do
> that by creating a procedure that acts just like the current
> continuation.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Aliases

`call/cc`;


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:continuations`](../../r7rs/categories/vs_3a_continuations.md#category__r7rs__vs_3a_continuations);
[`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

