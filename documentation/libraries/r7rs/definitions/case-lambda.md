

<a id='definition__r7rs__case-lambda'></a>

# `case-lambda` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `argument`: identifier;
 * `argument-rest`: identifier;
 * `arguments`: pattern with variants:
   * `()`;
   * `(|argument| |...|)`;
   * `(|argument| |...| . |argument-rest|)`;
   * `|argument-rest|`;
 * `expression`: expression;

Syntax variants:
 * `(|_| (|arguments| |expression|) |...|)`


#### Description

> ````
> (case-lambda <clause> ...)
> ````
> 
> 
> **Syntax**:
> Each `<clause>` is of the form
> `(<formals> <body>)`,
> where `<formals>` and `<body>` have the same syntax
> as in a `lambda` expression.
> 
> **Semantics**:
> A `case-lambda` expression evaluates to a procedure that accepts
> a variable number of arguments and is lexically scoped in the same
> manner as a procedure resulting from a `lambda` expression. When the procedure
> is called, the first `<clause>` for which the arguments agree
> with `<formals>` is selected, where agreement is specified as for
> the `<formals>` of a `lambda` expression. The variables of `<formals>` are
> bound to fresh locations, the values of the arguments are stored in those
> locations, the `<body>` is evaluated in the extended environment,
> and the results of `<body>` are returned as the results of the
> procedure call.
> 
> It is an error for the arguments not to agree with
> the `<formals>` of any `<clause>`.
> 
> ````
> (define range
>   (case-lambda
>    ((e) (range 0 e))
>    ((b e) (do ((r '() (cons e r))
>                (e (- e 1) (- e 1)))
>               ((< e b) r)))))
> 
> (range 3)    ===> (0 1 2)
> (range 3 5)  ===> (3 4)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:case-lambda`](../../r7rs/categories/r7rs_3a_case-lambda.md#category__r7rs__r7rs_3a_case-lambda);
[`vs:lambda`](../../r7rs/categories/vs_3a_lambda.md#category__r7rs__vs_3a_lambda);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

