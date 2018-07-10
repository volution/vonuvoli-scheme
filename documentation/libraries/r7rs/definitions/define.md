

<a id='definition__r7rs__define'></a>

# `define` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `variable`: identifier;
 * `argument`: identifier;
 * `argument-rest`: identifier;
 * `expression`: expression;

Syntax variants:
 * `(|_| |variable| |expression|)`
 * `(|_| (|variable|) |expression| |...|)`
 * `(|_| (|variable| |argument| |...|) |expression| |...|)`
 * `(|_| (|variable| |argument| |...| . |argument-rest|) |expression| |...|)`
 * `(|_| (|variable| . |argument-rest|) |expression| |...|)`


#### Description

> ````
> (define <variable> <expression>)
> (define (<variable> <formals>) <body>)
> ````
> 
> 
> ##### Variable definitions
> 
> A variable definition binds one or more identifiers and specifies an initial
> value for each of them.
> The simplest kind of variable definition
> takes one of the following forms:
> 
>   * `(define <variable> <expression>)`
> 
>   * `(define (<variable> <formals>) <body>)`
> 
> `<Formals>` are either a
> sequence of zero or more variables, or a sequence of one or more
> variables followed by a space-delimited period and another variable (as
> in a lambda expression).  This form is equivalent to
> ````
> (define <variable>
>   (lambda (<formals>) <body>))
> ````
> 
>   * `(define (<variable> . <formal>) <body>)`
> 
> `<Formal>` is a single
> variable.  This form is equivalent to
> ````
> (define <variable>
>   (lambda <formal> <body>))
> ````
> 
> 
> ##### Top level definitions
> 
> At the outermost level of a program, a definition
> ````
> (define <variable> <expression>)
> ````
> has essentially the same effect as the assignment expression
> ````
> (set! <variable> <expression>)
> ````
> if `<variable>` is bound to a non-syntax value.  However, if
> `<variable>` is not bound,
> or is a syntactic keyword,
> then the definition will bind
> `<variable>` to a new location before performing the assignment,
> whereas it would be an error to perform a `set!` on an
> unbound variable.
> 
> ````
> (define add3
>   (lambda (x) (+ x 3)))
> (add3 3)                            ===>  6
> (define first car)
> (first '(1 2))                      ===>  1
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:contexts`](../../r7rs/categories/vs_3a_contexts.md#category__r7rs__vs_3a_contexts);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

