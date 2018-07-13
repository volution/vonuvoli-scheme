

<a id='appendix__r7rs__internal-definitions'></a>

# `r7rs` -- Internal definitions


<a id='appendix__r7rs__internal-definitions__description'></a>

#### Description

> Definitions can occur at the
> beginning of a `<body>` (that is, the body of a `lambda`,
> `let`, `let*`, `letrec`, `letrec*`,
> `let-values`, `let*-values`, `let-syntax`, `letrec-syntax`,
> `parameterize`, `guard`, or `case-lambda`).  Note that
> such a body might not be apparent until after expansion of other syntax.
> Such definitions are known as __internal definitions__
> as opposed to the global definitions described above.
> The variables defined by internal definitions are local to the
> `<body>`.  That is, `<variable>` is bound rather than assigned,
> and the region of the binding is the entire `<body>`.  For example,
> 
> ````
> (let ((x 5))
>   (define foo (lambda (y) (bar x y)))
>   (define bar (lambda (a b) (+ (* a b) a)))
>   (foo (+ x 3)))                ===>  45
> ````
> 
> An expanded `<body>` containing internal definitions
> can always be
> converted into a completely equivalent `letrec*` expression.  For
> example, the `let` expression in the above example is equivalent
> to
> 
> ````
> (let ((x 5))
>   (letrec* ((foo (lambda (y) (bar x y)))
>             (bar (lambda (a b) (+ (* a b) a))))
>     (foo (+ x 3))))
> ````
> 
> Just as for the equivalent `letrec*` expression, it is an error if it is not
> possible to evaluate each `<expression>` of every internal
> definition in a `<body>` without assigning or referring to
> the value of the corresponding `<variable>` or the `<variable>`
> of any of the definitions that follow it in `<body>`.
> 
> It is an error to define the same identifier more than once in the
> same `<body>`.
> 
> Wherever an internal definition can occur,
> `(begin <definition_1> ...)`
> is equivalent to the sequence of definitions
> that form the body of the `begin`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

