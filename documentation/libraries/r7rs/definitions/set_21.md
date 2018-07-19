

<a id='definition__r7rs__set_21'></a>

# `set!` -- `r7rs` Definition


<a id='definition__r7rs__set_21__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__set_21__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `variable`: identifier;
 * `expression`: expression;

Syntax variants:
 * `(_ variable expression)`


<a id='definition__r7rs__set_21__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__set_21__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__set_21__description'></a>

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


<a id='definition__r7rs__set_21__categories'></a>

#### Categories

 * [`vs:contexts`](../../vonuvoli/categories/vs_3a_contexts.md#category__vonuvoli__vs_3a_contexts);


<a id='definition__r7rs__set_21__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

