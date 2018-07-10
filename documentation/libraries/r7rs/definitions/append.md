

<a id='definition__r7rs__append'></a>

# `append` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|null|))`
   * inputs: none;
   * output: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * `((|any|) |->| (|any|))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * `((|list-proper| |...|) |->| (|list-proper|))`
   * inputs:
     * a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
     * `...` (i.e. variadic);
   * output: a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
 * `((|list| |...|) |->| (|list-dotted|))`
   * inputs:
     * a value of type [`list`](../../r7rs/types/list.md#type__r7rs__list);
     * `...` (i.e. variadic);
   * output: a value of type [`list-dotted`](../../r7rs/types/list-dotted.md#type__r7rs__list-dotted);


#### Referenced types

[`null`](../../r7rs/types/null.md#type__r7rs__null);
[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
[`list`](../../r7rs/types/list.md#type__r7rs__list);
[`list-dotted`](../../r7rs/types/list-dotted.md#type__r7rs__list-dotted);


#### Description

> ````
> (append list ...)
> ````
> 
> 
> **Domain**:  The last argument, if there is one, can be of any type.
> 
> Returns a list consisting of the elements of the first `list`
> followed by the elements of the other `list`s.
> If there are no arguments, the empty list is returned.
> If there is exactly one argument, it is returned.
> Otherwise the resulting list is always newly allocated, except that it shares
> structure with the last argument.
> An improper list results if the last argument is not a
> proper list.
> 
> ````
> (append '(x) '(y))              ===>  (x y)
> (append '(a) '(b c d))          ===>  (a b c d)
> (append '(a (b)) '((c)))        ===>  (a (b) (c))
> ````
> 
> 
> ````
> (append '(a b) '(c . d))        ===>  (a b c . d)
> (append '() 'a)                 ===>  a
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:lists`](../../r7rs/categories/vs_3a_lists.md#category__r7rs__vs_3a_lists);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

