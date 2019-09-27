

<a id='definition__r7rs__letrec-syntax'></a>

# `letrec-syntax` -- `r7rs` Definition


<a id='definition__r7rs__letrec-syntax__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__letrec-syntax__implemented-by'></a>

#### Implemented by

 * [`letrec-syntax`](../../vonuvoli/definitions/letrec-syntax.md#definition__vonuvoli__letrec-syntax) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__letrec-syntax__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `keyword`: identifier;
 * `syntaxes`: pattern with variants:
   * `()`;
   * `((keyword @syntax-transformer) |...|)`;
 * `expression`: expression;

Syntax variants:
 * `(_ syntaxes)`
 * `(_ syntaxes expression |...|)`


<a id='definition__r7rs__letrec-syntax__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__letrec-syntax__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__letrec-syntax__description'></a>

#### Description

> ````
> (letrec-syntax <bindings> <body>)
> ````
> 
> 
> **Syntax**:
> Same as for `let-syntax`.
> 
> **Semantics**:
>  The `<body>` is expanded in the syntactic environment obtained by
> extending the syntactic environment of the `letrec-syntax`
> expression with macros whose keywords are the
> `<keyword>`s, bound to the specified transformers.
> Each binding of a `<keyword>` has the `<transformer-spec>`s
> as well as the `<body>` within its region,
> so the transformers can
> transcribe expressions into uses of the macros
> introduced by the `letrec-syntax` expression.
> 
> ````
> (letrec-syntax
>     ((my-or (syntax-rules ()
>               ((my-or) #f)
>               ((my-or e) e)
>               ((my-or e1 e2 ...)
>                (let ((temp e1))
>                  (if temp
>                      temp
>                      (my-or e2 ...)))))))
>   (let ((x #f)
>         (y 7)
>         (temp 8)
>         (let odd?)
>         (if even?))
>     (my-or x
>            (let temp)
>            (if y)
>            y)))        ===>  7
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

