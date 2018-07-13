

<a id='definition__r7rs__let-values'></a>

# `let-values` -- `r7rs` Definitions


<a id='definition__r7rs__let-values__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__let-values__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `variable`: identifier;
 * `initializer`: identifier;
 * `binding`: pattern with variants:
   * `((variable ...) initializer)`;
 * `bindings`: pattern with variants:
   * `()`;
   * `(binding ...)`;
 * `expression`: expression;

Syntax variants:
 * `(_ bindings)`
 * `(_ bindings expression ...)`


<a id='definition__r7rs__let-values__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__let-values__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__let-values__description'></a>

#### Description

> ````
> (let-values <mv-binding-spec> <body>)
> ````
> 
> 
> **Syntax**:
> `<Mv-binding-spec>` has the form
> ````
> ((<formals_1> <init_1>) ...)
> ````
> 
> where each `<init>` is an expression, and `<body>` is
> zero or more definitions followed by a sequence of one or
> more expressions as described in section on `lambda`.  It is an error for a variable to appear more than
> once in the set of `<formals>`.
> 
> **Semantics**:
> The `<init>`s are evaluated in the current environment (in some
> unspecified order) as if by invoking `call-with-values`, and the
> variables occurring in the `<formals>` are bound to fresh locations
> holding the values returned by the `<init>`s, where the
> `<formals>` are matched to the return values in the same way that
> the `<formals>` in a `lambda` expression are matched to the
> arguments in a procedure call.  Then, the `<body>` is evaluated in
> the extended environment, and the values of the last expression of
> `<body>` are returned.  Each binding of a `<variable>` has
> `<body>` as its region.
> 
> It is an error if the `<formals>` do not match the number of
> values returned by the corresponding `<init>`.
> 
> ````
> (let-values (((root rem) (exact-integer-sqrt 32)))
>   (* root rem))                ===>  35
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__let-values__categories'></a>

#### Categories

 * [`vs:contexts`](../../r7rs/categories/vs_3a_contexts.md#category__r7rs__vs_3a_contexts);
 * [`vs:values`](../../r7rs/categories/vs_3a_values.md#category__r7rs__vs_3a_values);


<a id='definition__r7rs__let-values__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

