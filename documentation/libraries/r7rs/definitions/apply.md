

<a id='definition__r7rs__apply'></a>

# `apply` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|procedure|) |->| (|any|))`
   * input: a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * `((|procedure| |any| |...|) |->| (|any|))`
   * inputs:
     * a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Referenced types

[`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
[`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Description

> ````
> (apply proc arg_1 ... args)
> ````
> 
> 
> The `apply` procedure calls `proc` with the elements of the list
> `(append (list arg_1 ...) args)` as the actual
> arguments.
> 
> ````
> (apply + (list 3 4))              ===>  7
> 
> (define compose
>   (lambda (f g)
>     (lambda args
>       (f (apply g args)))))
> 
> ((compose sqrt *) 12 75)              ===>  30
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:functions`](../../r7rs/categories/vs_3a_functions.md#category__r7rs__vs_3a_functions);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

