

<a id='definition__r7rs__vector-_3e_list'></a>

# `vector->list` -- `r7rs` Definition


<a id='definition__r7rs__vector-_3e_list__kind'></a>

#### Kind

`converter`;


<a id='definition__r7rs__vector-_3e_list__implemented-by'></a>

#### Implemented by

 * [`vector->list`](../../vonuvoli/definitions/vector-_3e_list.md#definition__vonuvoli__vector-_3e_list) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__vector-_3e_list__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((vector-empty) -> (null))`
   * input: a value of type [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
   * output: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * `((vector-not-empty) -> (list-proper-not-null))`
   * input: a value of type [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);
   * output: a value of type [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);
 * `((vector range-start) -> (list-proper))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
 * `((vector range-start range-end) -> (list-proper))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);


<a id='definition__r7rs__vector-_3e_list__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__vector-_3e_list__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__vector-_3e_list__description'></a>

#### Description

> ````
> (vector->list vector)
> (vector->list vector start)
> (vector->list vector start end)
> (list->vector list)
> ````
> 
> 
> The `vector->list` procedure returns a newly allocated list of the objects contained
> in the elements of `vector` between `start` and `end`.
> The `list->vector` procedure returns a newly
> created vector initialized to the elements of the list `list`.
> 
> In both procedures, order is preserved.
> 
> ````
> (vector->list '#(dah dah didah))      ===>  (dah dah didah)
> (vector->list '#(dah dah didah) 1 2)  ===>  (dah)
> (list->vector '(dididit dah))         ===>  #(dididit dah)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__vector-_3e_list__referenced-types'></a>

#### Referenced-types

 * [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
 * [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);
 * [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);
 * [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
 * [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
 * [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
 * [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

