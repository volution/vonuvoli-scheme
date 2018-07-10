

<a id='definition__r7rs__and'></a>

# `and` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `expression`: expression;

Syntax variants:
 * `(|_|)`
 * `(|_| |expression| |...|)`


#### Description

> ````
> (and <test_1> ...)
> ````
> 
> 
> **Semantics**:
> The `<test>` expressions are evaluated from left to right, and if
> any expression evaluates to `#f` (see
> section on booleans), then `#f` is returned.  Any remaining
> expressions are not evaluated.  If all the expressions evaluate to
> true values, the values of the last expression are returned.  If there
> are no expressions, then `#t` is returned.
> 
> ````
> (and (= 2 2) (> 2 1))           ===>  #t
> (and (= 2 2) (< 2 1))           ===>  #f
> (and 1 2 'c '(f g))             ===>  (f g)
> (and)                           ===>  #t
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

