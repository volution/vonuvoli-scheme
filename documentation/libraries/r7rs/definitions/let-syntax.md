

<a id='definition__r7rs__let-syntax'></a>

# `let-syntax` -- `r7rs` Definitions


<a id='definition__r7rs__let-syntax__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__let-syntax__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `keyword`: identifier;
 * `syntaxes`: pattern with variants:
   * `()`;
   * `((keyword @syntax-transformer) ...)`;
 * `expression`: expression;

Syntax variants:
 * `(_ syntaxes)`
 * `(_ syntaxes expression ...)`


<a id='definition__r7rs__let-syntax__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__let-syntax__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__let-syntax__description'></a>

#### Description

> ````
> (let-syntax <bindings> <body>)
> ````
> 
> 
> **Syntax**:
> `<Bindings>` has the form
> ````
> ((<keyword> <transformer-spec>) ...)
> ````
> Each `<keyword>` is an identifier,
> each `<transformer-spec>` is an instance of `syntax-rules`, and
> `<body>` is a sequence of one or more definitions followed
> by one or more expressions.  It is an error
> for a `<keyword>` to appear more than once in the list of keywords
> being bound.
> 
> **Semantics**:
> The `<body>` is expanded in the syntactic environment
> obtained by extending the syntactic environment of the
> `let-syntax` expression with macros whose keywords are
> the `<keyword>`s, bound to the specified transformers.
> Each binding of a `<keyword>` has `<body>` as its region.
> 
> ````
> (let-syntax ((given-that (syntax-rules ()
>                      ((given-that test stmt1 stmt2 ...)
>                       (if test
>                           (begin stmt1
>                                  stmt2 ...))))))
>   (let ((if #t))
>     (given-that if (set! if 'now))
>     if))                           ===>  now
> 
> (let ((x 'outer))
>   (let-syntax ((m (syntax-rules () ((m) x))))
>     (let ((x 'inner))
>       (m))))                       ===>  outer
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__let-syntax__categories'></a>

#### Categories

 * [`vs:syntaxes`](../../r7rs/categories/vs_3a_syntaxes.md#category__r7rs__vs_3a_syntaxes);
 * [`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);


<a id='definition__r7rs__let-syntax__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

