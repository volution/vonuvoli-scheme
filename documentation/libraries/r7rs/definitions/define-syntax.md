

<a id='definition__r7rs__define-syntax'></a>

# `define-syntax` -- `r7rs` Definition


<a id='definition__r7rs__define-syntax__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__define-syntax__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `keyword`: identifier;

Syntax variants:
 * `(_ keyword @syntax-transformer)`


<a id='definition__r7rs__define-syntax__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__define-syntax__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__define-syntax__description'></a>

#### Description

> ````
> (define-syntax <keyword> <transformer-spec>)
> ````
> 
> 
> `<Keyword>` is an identifier, and
> the `<transformer-spec>` is an instance of `syntax-rules`.
> Like variable definitions, syntax definitions can appear at the outermost level or
> nested within a `body`.
> 
> If the `define-syntax` occurs at the outermost level, then the global
> syntactic environment is extended by binding the
> `<keyword>` to the specified transformer, but previous expansions
> of any global binding for `<keyword>` remain unchanged.
> Otherwise, it is an __internal syntax definition__, and is local to the
> `<body>` in which it is defined.
> Any use of a syntax keyword before its corresponding definition is an error.
> In particular, a use that precedes an inner definition will not apply an outer
> definition.
> 
> ````
> (let ((x 1) (y 2))
>   (define-syntax swap!
>     (syntax-rules ()
>       ((swap! a b)
>        (let ((tmp a))
>          (set! a b)
>          (set! b tmp)))))
>   (swap! x y)
>   (list x y))                ===> (2 1)
> ````
> 
> Macros can expand into definitions in any context that permits
> them. However, it is an error for a definition to define an
> identifier whose binding has to be known in order to determine the meaning of the
> definition itself, or of any preceding definition that belongs to the
> same group of internal definitions. Similarly, it is an error for an
> internal definition to define an identifier whose binding has to be known
> in order
> to determine the boundary between the internal definitions and the
> expressions of the body it belongs to. For example, the following are
> errors:
> 
> ````
> (define define 3)
> 
> (begin (define begin list))
> 
> (let-syntax
>     ((foo (syntax-rules ()
>             ((foo (proc args ...) body ...)
>              (define proc
>                (lambda (args ...)
>                  body ...))))))
>   (let ((x 3))
>     (foo (plus x y) (+ x y))
>     (define foo x)
>     (plus foo x)))
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__define-syntax__categories'></a>

#### Categories

 * [`vs:syntaxes`](../../r7rs/categories/vs_3a_syntaxes.md#category__r7rs__vs_3a_syntaxes);
 * [`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);


<a id='definition__r7rs__define-syntax__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

