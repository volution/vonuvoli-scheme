

<a id='definition__r7rs__unless'></a>

# `unless` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `condition`: expression;
 * `then-expression`: expression;

Syntax variants:
 * `(|_| |condition| |then-expression| |...|)`


#### Description

> ````
> (unless <test> <expression_1> <expression_2> ...)
> ````
> 
> 
> **Syntax**:
> The `<test>` is an expression.
> 
> **Semantics**:
> The test is evaluated, and if it evaluates to `#f`,
> the expressions are evaluated in order.  The result of the `unless`
> expression is unspecified.
> 
> ````
> (unless (= 1 1.0)
>   (display "1")
>   (display "2"))  ===>  #unspecified
>            ; and prints nothing
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:control`](../../r7rs/categories/vs_3a_control.md#category__r7rs__vs_3a_control);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

