

<a id='definition__r7rs__letrec-syntax'></a>

# `letrec-syntax` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `keyword`: identifier;
 * `syntaxes`: pattern with variants:
   * `()`;
   * `((|keyword| |@syntax-transformer|) |...|)`;
 * `expression`: expression;

Syntax variants:
 * `(|_| |syntaxes|)`
 * `(|_| |syntaxes| |expression| |...|)`


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


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:syntaxes`](../../r7rs/categories/vs_3a_syntaxes.md#category__r7rs__vs_3a_syntaxes);
[`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

