

<a id='category__r7rs__r7rs_3a_expressions-bindings'></a>

# `r7rs:expressions-bindings` -- `r7rs` Categories


#### Description

> ##### Binding constructs
> 
> The binding constructs `let`, `let*`, `letrec`, `letrec*`,
> `let-values`, and `let*-values`
> give Scheme a block structure, like Algol 60.  The syntax of the first four
> constructs is identical, but they differ in the regions they establish
> for their variable bindings.  In a `let` expression, the initial
> values are computed before any of the variables become bound; in a
> `let*` expression, the bindings and evaluations are performed
> sequentially; while in `letrec` and `letrec*` expressions,
> all the bindings are in
> effect while their initial values are being computed, thus allowing
> mutually recursive definitions.
> The `let-values` and `let*-values` constructs are analogous to `let` and `let*`
> respectively, but are designed to handle multiple-valued expressions, binding
> different identifiers to the returned values.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Super-category

[`r7rs:expressions`](../../r7rs/categories/r7rs_3a_expressions.md#category__r7rs__r7rs_3a_expressions);


##### Super-categories recursive

[`r7rs:expressions`](../../r7rs/categories/r7rs_3a_expressions.md#category__r7rs__r7rs_3a_expressions);
[`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

