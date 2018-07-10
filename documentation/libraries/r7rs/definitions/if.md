

<a id='definition__r7rs__if'></a>

# `if` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `condition`: expression;
 * `then-expression`: expression;
 * `else-expression`: expression;

Syntax variants:
 * `(|_| |condition| |then-expression|)`
 * `(|_| |condition| |then-expression| |else-expression|)`


#### Description

> ````
> (if <test> <consequent> <alternate>)
> (if <test> <consequent>)
> ````
> 
> 
> **Syntax**:
> `<Test>`, `<consequent>`, and `<alternate>` are
> expressions.
> 
> **Semantics**:
> An `if` expression is evaluated as follows: first,
> `<test>` is evaluated.  If it yields a true value (see
> section on booleans), then `<consequent>` is evaluated and
> its values are returned.  Otherwise `<alternate>` is evaluated and its
> values are returned.  If `<test>` yields a false value and no
> `<alternate>` is specified, then the result of the expression is
> unspecified.
> 
> ````
> (if (> 3 2) 'yes 'no)           ===>  yes
> (if (> 2 3) 'yes 'no)           ===>  no
> (if (> 3 2)
>     (- 3 2)
>     (+ 3 2))                    ===>  1
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

