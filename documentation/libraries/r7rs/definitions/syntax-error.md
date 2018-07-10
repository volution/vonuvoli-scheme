

<a id='definition__r7rs__syntax-error'></a>

# `syntax-error` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `message`: value of type [string](../../r7rs/types/string.md#type__r7rs__string);
 * `argument`: value of type [any](../../r7rs/types/any.md#type__r7rs__any);

Syntax variants:
 * `(|_| |message|)`
 * `(|_| |message| |argument| |...|)`


#### Referenced types

[`string`](../../r7rs/types/string.md#type__r7rs__string);
[`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Description

> ````
> (syntax-error <message> <args> ...)
> ````
> 
> 
> `syntax-error` behaves similarly to `error` except that implementations
> with an expansion pass separate from evaluation should signal an error
> as soon as `syntax-error` is expanded.  This can be used as
> a `syntax-rules` `<template>` for a `<pattern>` that is
> an invalid use of the macro, which can provide more descriptive error
> messages.  `<message>` is a string literal, and `<args>`
> arbitrary expressions providing additional information.
> Applications cannot count on being able to catch syntax errors with
> exception handlers or guards.
> 
> ````
> (define-syntax simple-let
>   (syntax-rules ()
>     ((_ (head ... ((x . y) val) . tail)
>         body1 body2 ...)
>      (syntax-error
>       "expected an identifier but got"
>       (x . y)))
>     ((_ ((name val) ...) body1 body2 ...)
>      ((lambda (name ...) body1 body2 ...)
>        val ...))))
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

