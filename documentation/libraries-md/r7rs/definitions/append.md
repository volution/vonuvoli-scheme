

<a id='definition__r7rs__append'></a>

# `append` -- `r7rs` Definition


<a id='definition__r7rs__append__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__append__implemented-by'></a>

#### Implemented by

 * [`append`](../../vonuvoli/definitions/append.md#definition__vonuvoli__append) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__append__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (null))`
   * inputs: none;
   * output: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * `(((a any)) -> ((a any)))`
   * input: `a` of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: `a` of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * `((list-proper |2...|) -> (list-proper))`
   * inputs:
     * a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
     * `...` -- at least 2 times;
   * output: a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
 * `((&variadic-min +1 list-proper &trailing any) -> (list-dotted))`
   * inputs:
     * (variadic) a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
     * (variadic -- at least one time;)
     * (trailing) a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`list-dotted`](../../r7rs/types/list-dotted.md#type__r7rs__list-dotted);


<a id='definition__r7rs__append__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__append__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__append__description'></a>

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


<a id='definition__r7rs__append__referenced-types'></a>

#### Referenced-types

 * [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
 * [`list-dotted`](../../r7rs/types/list-dotted.md#type__r7rs__list-dotted);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

