

<a id='definition__r7rs__let_2a'></a>

# `let*` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `variable`: identifier;
 * `initializer`: identifier;
 * `binding`: pattern with variants:
   * `(|variable| |initializer|)`;
 * `bindings`: pattern with variants:
   * `()`;
   * `(|binding| |...|)`;
 * `expression`: expression;

Syntax variants:
 * `(|_| |bindings|)`
 * `(|_| |bindings| |expression| |...|)`


#### Description

> ````
> (let* <bindings> <body>)
> ````
> 
> 
> **Syntax**:
> `<Bindings>` has the form
> ````
> ((<variable_1> <init_1>) ...)
> ````
> and `<body>` is a sequence of
> zero or more definitions followed by
> one or more expressions as described in section on `lambda`.
> 
> **Semantics**:
> The `let*` binding construct is similar to `let`, but the bindings are performed
> sequentially from left to right, and the region of a binding indicated
> by `(<variable> <init>)` is that part of the `let*`
> expression to the right of the binding.  Thus the second binding is done
> in an environment in which the first binding is visible, and so on.
> The `<variable>`s need not be distinct.
> 
> ````
> (let ((x 2) (y 3))
>   (let* ((x 7)
>          (z (+ x y)))
>     (* z x)))             ===>  70
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

