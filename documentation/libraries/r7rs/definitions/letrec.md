

<a id='definition__r7rs__letrec'></a>

# `letrec` -- `r7rs` Definitions


<a id='definition__r7rs__letrec__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__letrec__syntax-signature'></a>

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


<a id='definition__r7rs__letrec__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base);


<a id='definition__r7rs__letrec__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__letrec__description'></a>

#### Description

> ````
> (letrec <bindings> <body>)
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
> one or more expressions as described in section on `lambda`. It is an error for a `<variable>` to appear more
> than once in the list of variables being bound.
> 
> **Semantics**:
> The `<variable>`s are bound to fresh locations holding unspecified
> values, the `<init>`s are evaluated in the resulting environment (in
> some unspecified order), each `<variable>` is assigned to the result
> of the corresponding `<init>`, the `<body>` is evaluated in the
> resulting environment, and the values of the last expression in
> `<body>` are returned.  Each binding of a `<variable>` has the
> entire `letrec` expression as its region, making it possible to
> define mutually recursive procedures.
> 
> ````
> (letrec ((even?
>           (lambda (n)
>             (if (zero? n)
>                 #t
>                 (odd? (- n 1)))))
>          (odd?
>           (lambda (n)
>             (if (zero? n)
>                 #f
>                 (even? (- n 1))))))
>   (even? 88))
>                 ===>  #t
> ````
> 
> One restriction on `letrec` is very important: if it is not possible
> to evaluate each `<init>` without assigning or referring to the value of any
> `<variable>`, it is an error.  The
> restriction is necessary because
> `letrec` is defined in terms of a procedure
> call where a `lambda` expression binds the `<variable>`s to the values
> of the `<init>`s.
> In the most common uses of `letrec`, all the `<init>`s are
> `lambda` expressions and the restriction is satisfied automatically.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__letrec__categories'></a>

#### Categories

 * [`vs:contexts`](../../r7rs/categories/vs_3a_contexts.md#category__r7rs__vs_3a_contexts);


<a id='definition__r7rs__letrec__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

