

<a id='definition__r7rs__syntax-error'></a>

# `syntax-error` -- `r7rs` Definition


<a id='definition__r7rs__syntax-error__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__syntax-error__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `message`: value of type [string](../../r7rs/types/string.md#type__r7rs__string);
 * `argument`: value of type [any](../../r7rs/types/any.md#type__r7rs__any);

Syntax variants:
 * `(_ message)`
 * `(_ message argument ...)`


<a id='definition__r7rs__syntax-error__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__syntax-error__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__syntax-error__description'></a>

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


<a id='definition__r7rs__syntax-error__referenced-types'></a>

#### Referenced-types

 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__syntax-error__categories'></a>

#### Categories

 * [`vs:syntaxes`](../../vonuvoli/categories/vs_3a_syntaxes.md#category__vonuvoli__vs_3a_syntaxes);
 * [`vs:unsupported`](../../vonuvoli/categories/vs_3a_unsupported.md#category__vonuvoli__vs_3a_unsupported);


<a id='definition__r7rs__syntax-error__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

