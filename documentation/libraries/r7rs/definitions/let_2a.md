

<a id='definition__r7rs__let_2a'></a>

# `let*` -- `r7rs` Definitions


<a id='definition__r7rs__let_2a__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__let_2a__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `variable`: identifier;
 * `initializer`: identifier;
 * `binding`: pattern with variants:
   * `(variable initializer)`;
 * `bindings`: pattern with variants:
   * `()`;
   * `(binding ...)`;
 * `expression`: expression;

Syntax variants:
 * `(_ bindings)`
 * `(_ bindings expression ...)`


<a id='definition__r7rs__let_2a__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__let_2a__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__let_2a__description'></a>

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


<a id='definition__r7rs__let_2a__categories'></a>

#### Categories

 * [`vs:contexts`](../../r7rs/categories/vs_3a_contexts.md#category__r7rs__vs_3a_contexts);


<a id='definition__r7rs__let_2a__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

