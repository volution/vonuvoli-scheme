

<a id='definition__r7rs__error'></a>

# `error` -- `r7rs` Definitions


#### Kind

`constructor`;


#### Procedure signature

Procedure variants:
 * `(((|message| . |string|)) |->| (|error-object|))`
   * input: `message` of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
   * output: a value of type [`error-object`](../../r7rs/types/error-object.md#type__r7rs__error-object);
 * `(((|message| . |string|) (|irritant| . |any|) |...|) |->| (|error-object|))`
   * inputs:
     * `message` of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `irritant` of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`error-object`](../../r7rs/types/error-object.md#type__r7rs__error-object);


#### Referenced types

[`string`](../../r7rs/types/string.md#type__r7rs__string);
[`error-object`](../../r7rs/types/error-object.md#type__r7rs__error-object);
[`any`](../../r7rs/types/any.md#type__r7rs__any);


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


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:errors`](../../r7rs/categories/vs_3a_errors.md#category__r7rs__vs_3a_errors);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

