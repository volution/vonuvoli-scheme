

<a id='definition__r7rs__set_21'></a>

# `set!` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `variable`: identifier;
 * `expression`: expression;

Syntax variants:
 * `(|_| |variable| |expression|)`


#### Description

> ````
> (set! <variable> <expression>)
> ````
> 
> 
> 
> **Semantics**:
> `<Expression>` is evaluated, and the resulting value is stored in
> the location to which `<variable>` is bound.  It is an error if `<variable>` is not
> bound either in some region enclosing the `set!` expression
> or else globally.
> The result of the `set!` expression is
> unspecified.
> 
> ````
> (define x 2)
> (+ x 1)                 ===>  3
> (set! x 4)              ===>  #unspecified
> (+ x 1)                 ===>  5
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

