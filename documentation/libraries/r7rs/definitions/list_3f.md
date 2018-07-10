

<a id='definition__r7rs__list_3f'></a>

# `list?` -- `r7rs` Definitions


#### Kind

`type-predicate`;


#### Procedure signature

Procedure variants:
 * `((|null|) |->| (|true|))`
   * input: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|list-proper|) |->| (|true|))`
   * input: a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|list-dotted|) |->| (|false|))`
   * input: a value of type [`list-dotted`](../../r7rs/types/list-dotted.md#type__r7rs__list-dotted);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|list-circular|) |->| (|false|))`
   * input: a value of type [`list-circular`](../../r7rs/types/list-circular.md#type__r7rs__list-circular);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|any|) |->| (|false|))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|any| |...|) |->| (|boolean|))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `|vonuvoli|`


#### Referenced types

[`null`](../../r7rs/types/null.md#type__r7rs__null);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
[`list-dotted`](../../r7rs/types/list-dotted.md#type__r7rs__list-dotted);
[`false`](../../r7rs/types/false.md#type__r7rs__false);
[`list-circular`](../../r7rs/types/list-circular.md#type__r7rs__list-circular);
[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Description

> ````
> (list? obj)
> ````
> 
> 
> Returns `#t` if `obj` is a list.  Otherwise, it returns `#f`.
> By definition, all lists have finite length and are terminated by
> the empty list.
> 
> ````
> (list? '(a b c))     ===>  #t
> (list? '())          ===>  #t
> (list? '(a . b))     ===>  #f
> (let ((x (list 'a)))
>   (set-cdr! x x)
>   (list? x))         ===>  #f
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:lists`](../../r7rs/categories/vs_3a_lists.md#category__r7rs__vs_3a_lists);
[`vs:types`](../../r7rs/categories/vs_3a_types.md#category__r7rs__vs_3a_types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

