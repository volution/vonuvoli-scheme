

<a id='definition__r7rs__error'></a>

# `error` -- `r7rs` Definition


<a id='definition__r7rs__error__kind'></a>

#### Kind

`constructor`;


<a id='definition__r7rs__error__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(((message . string)) -> (error-object))`
   * input: `message` of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
   * output: a value of type [`error-object`](../../r7rs/types/error-object.md#type__r7rs__error-object);
 * `(((message . string) (irritant . any) ...) -> (error-object))`
   * inputs:
     * `message` of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `irritant` of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`error-object`](../../r7rs/types/error-object.md#type__r7rs__error-object);


<a id='definition__r7rs__error__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__error__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__error__description'></a>

#### Description

> ````
> (error message obj ...)
> ````
> 
> 
> **Domain**:  `Message` should be a string.
> 
> Raises an exception as if by calling
> `raise` on a newly allocated implementation-defined object which encapsulates
> the information provided by `message`,
> as well as any `obj`s, known as the __irritants__.
> The procedure `error-object?` must return `#t` on such objects.
> 
> ````
> (define (null-list? l)
>   (cond ((pair? l) #f)
>         ((null? l) #t)
>         (else
>           (error
>             "null-list?: argument out of domain"
>             l))))
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__error__referenced-types'></a>

#### Referenced-types

 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`error-object`](../../r7rs/types/error-object.md#type__r7rs__error-object);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__error__categories'></a>

#### Categories

 * [`vs:errors`](../../r7rs/categories/vs_3a_errors.md#category__r7rs__vs_3a_errors);


<a id='definition__r7rs__error__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

