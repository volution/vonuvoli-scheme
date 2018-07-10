

<a id='definition__r7rs__let'></a>

# `let` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `function`: identifier;
 * `variable`: identifier;
 * `initializer`: identifier;
 * `binding`: pattern with variants:
   * `(|variable| |initializer|)`;
 * `bindings`: pattern with variants:
   * `()`;
   * `(|binding| |...|)`;
 * `expression`: expression;

Syntax variants:
 * `(|_| |bindings|)`
 * `(|_| |bindings| |expression| |...|)`
 * `(|_| |function| |bindings| |expression| |...|)`


#### Description

> ````
> (let <bindings> <body>)
> ````
> 
> 
> **Syntax**:
> `<Bindings>` has the form
> ````
> ((<variable_1> <init_1>) ...)
> ````
> where each `<init>` is an expression, and `<body>` is a
> sequence of zero or more definitions followed by a
> sequence of one or more expressions as described in section on `lambda`.  It is
> an error for a `<variable>` to appear more than once in the list of variables
> being bound.
> 
> **Semantics**:
> The `<init>`s are evaluated in the current environment (in some
> unspecified order), the `<variable>`s are bound to fresh locations
> holding the results, the `<body>` is evaluated in the extended
> environment, and the values of the last expression of `<body>`
> are returned.  Each binding of a `<variable>` has `<body>` as its
> region.
> 
> ````
> (let ((x 2) (y 3))
>   (* x y))                      ===>  6
> 
> (let ((x 2) (y 3))
>   (let ((x 7)
>         (z (+ x y)))
>     (* z x)))                   ===>  35
> ````
> 
> See also "named `let`", section on iteration.
> 
> 
> ----
> 
> 
> ````
> (let <variable> <bindings> <body>)
> ````
> 
> 
> **Semantics**:
> "Named `let`" is a variant on the syntax of `let` which provides
> a more general looping construct than `do` and can also be used to express
> recursion.
> It has the same syntax and semantics as ordinary `let`
> except that `<variable>` is bound within `<body>` to a procedure
> whose formal arguments are the bound variables and whose body is
> `<body>`.  Thus the execution of `<body>` can be repeated by
> invoking the procedure named by `<variable>`.
> 
> ````
> (let loop ((numbers '(3 -2 1 6 -5))
>            (nonneg '())
>            (neg '()))
>   (cond ((null? numbers) (list nonneg neg))
>         ((>= (car numbers) 0)
>          (loop (cdr numbers)
>                (cons (car numbers) nonneg)
>                neg))
>         ((< (car numbers) 0)
>          (loop (cdr numbers)
>                nonneg
>                (cons (car numbers) neg)))))
>   ===>  ((6 1 3) (-5 -2))
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

