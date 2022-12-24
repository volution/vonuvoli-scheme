

<a id='definition__r7rs__pair_3f'></a>

# `pair?` -- `r7rs` Definition


<a id='definition__r7rs__pair_3f__kind'></a>

#### Kind

`type-predicate`;


<a id='definition__r7rs__pair_3f__extended-by'></a>

#### Extended by

 * [`pair?`](../../vonuvoli/definitions/pair_3f.md#definition__vonuvoli__pair_3f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__pair_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((pair) -> (true))`
   * input: a value of type [`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((null) -> (false))`
   * input: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((any) -> (false))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__pair_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__pair_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__pair_3f__description'></a>

#### Description

> ````
> (pair? obj)
> ````
> 
> 
> The `pair?` predicate returns `#t` if `obj` is a pair, and otherwise
> returns `#f`.
> 
> ````
> (pair? '(a . b))        ===>  #t
> (pair? '(a b c))        ===>  #t
> (pair? '())             ===>  #f
> (pair? '#(a b))         ===>  #f
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__pair_3f__referenced-types'></a>

#### Referenced-types

 * [`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

