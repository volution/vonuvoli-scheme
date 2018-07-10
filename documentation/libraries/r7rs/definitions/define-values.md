

<a id='definition__r7rs__define-values'></a>

# `define-values` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `variable`: identifier;
 * `expression`: expression;

Syntax variants:
 * `(|_| (|variable| |...|) |expression|)`


#### Description

> ````
> (define-values <formals> <expression>)
> ````
> 
> 
> Another kind of definition is provided by `define-values`,
> which creates multiple definitions from a single
> expression returning multiple values.
> It is allowed wherever `define` is allowed.
> 
> It is an error if a variable appears more than once in the set of `<formals>`.
> 
> **Semantics**:
> `<Expression>` is evaluated, and the `<formals>` are bound
> to the return values in the same way that the `<formals>` in a
> `lambda` expression are matched to the arguments in a procedure
> call.
> 
> ````
> (define-values (x y) (integer-sqrt 17))
> (list x y) ===> (4 1)
> 
> (let ()
>   (define-values (x y) (values 1 2))
>   (+ x y))     ===> 3
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:contexts`](../../r7rs/categories/vs_3a_contexts.md#category__r7rs__vs_3a_contexts);
[`vs:values`](../../r7rs/categories/vs_3a_values.md#category__r7rs__vs_3a_values);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

