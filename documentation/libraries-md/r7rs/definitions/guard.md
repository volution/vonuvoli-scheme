

<a id='definition__r7rs__guard'></a>

# `guard` -- `r7rs` Definition


<a id='definition__r7rs__guard__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__guard__implemented-by'></a>

#### Implemented by

 * [`guard`](../../vonuvoli/definitions/guard.md#definition__vonuvoli__guard) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__guard__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `variable`: identifier;
 * `else`: literal;
 * `clause-condition`: expression;
 * `clause-expression`: expression;
 * `clause`: pattern with variants:
   * `(clause-condition)`;
   * `(clause-condition clause-expression |...|)`;
   * `(else clause-expression |...|)`;
 * `guarded-expression`: expression;

Syntax variants:
 * `(_ (variable clause |...|) guarded-expression |...|)`


<a id='definition__r7rs__guard__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__guard__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__guard__description'></a>

#### Description

> ````
> (guard (<variable>
>         <cond-clause_1> <cond-clause_2> ...)
>     <body>)
> ````
> 
> **Syntax**:
> Each `<cond-clause>` is as in the specification of `cond`.
> 
> **Semantics**:
> The `<body>` is evaluated with an exception
> handler that binds the raised object (see `raise` in section on exceptions)
> to `<variable>` and, within the scope of
> that binding, evaluates the clauses as if they were the clauses of a
> `cond` expression. That implicit `cond` expression is evaluated with the
> continuation and dynamic environment of the `guard` expression. If every
> `<cond-clause>`'s `<test>` evaluates to `#f` and there
> is no else clause, then
> `raise-continuable` is invoked on the raised object within the dynamic
> environment of the original call to `raise`
> or `raise-continuable`, except that the current
> exception handler is that of the `guard` expression.
> 
> 
> See section on exceptions for a more complete discussion of
> exceptions.
> 
> ````
> (guard (condition
>          ((assq 'a condition) => cdr)
>          ((assq 'b condition)))
>   (raise (list (cons 'a 42))))
> ===> 42
> 
> (guard (condition
>          ((assq 'a condition) => cdr)
>          ((assq 'b condition)))
>   (raise (list (cons 'b 23))))
> ===> (b . 23)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

