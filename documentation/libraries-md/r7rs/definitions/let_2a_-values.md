

<a id='definition__r7rs__let_2a_-values'></a>

# `let*-values` -- `r7rs` Definition


<a id='definition__r7rs__let_2a_-values__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__let_2a_-values__implemented-by'></a>

#### Implemented by

 * [`let*-values`](../../vonuvoli/definitions/let_2a_-values.md#definition__vonuvoli__let_2a_-values) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__let_2a_-values__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `variable`: identifier;
 * `initializer`: identifier;
 * `binding`: pattern with variants:
   * `((variable |...|) initializer)`;
 * `bindings`: pattern with variants:
   * `()`;
   * `(binding |...|)`;
 * `expression`: expression;

Syntax variants:
 * `(_ bindings)`
 * `(_ bindings expression |...|)`


<a id='definition__r7rs__let_2a_-values__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__let_2a_-values__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__let_2a_-values__description'></a>

#### Description

> ````
> (let*-values <mv-binding-spec> <body>)
> ````
> 
> 
> **Syntax**:
> `<Mv-binding-spec>` has the form
> ````
> ((<formals> <init>) ...)
> ````
> and `<body>` is a sequence of zero or more
> definitions followed by one or more expressions as described in section `lambda`.  In each `<formals>`,
> it is an error if any variable appears more than once.
> 
> **Semantics**:
> The `let*-values` construct is similar to `let-values`, but the
> `<init>`s are evaluated and bindings created sequentially from
> left to right, with the region of the bindings of each `<formals>`
> including the `<init>`s to its right as well as `<body>`.  Thus the
> second `<init>` is evaluated in an environment in which the first
> set of bindings is visible and initialized, and so on.
> 
> ````
> (let ((a 'a) (b 'b) (x 'x) (y 'y))
>   (let*-values (((a b) (values x y))
>                 ((x y) (values a b)))
>     (list a b x y)))     ===> (x y x y)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

