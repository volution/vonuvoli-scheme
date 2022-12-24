

<a id='definition__r7rs__vector-fill_21'></a>

# `vector-fill!` -- `r7rs` Definition


<a id='definition__r7rs__vector-fill_21__kind'></a>

#### Kind

`mutator!`;


<a id='definition__r7rs__vector-fill_21__implemented-by'></a>

#### Implemented by

 * [`vector-fill!`](../../vonuvoli/definitions/vector-fill_21.md#definition__vonuvoli__vector-fill_21) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__vector-fill_21__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((vector any) -> (undefined))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * `((vector any range-start) -> (undefined))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * `((vector any range-start range-end) -> (undefined))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


<a id='definition__r7rs__vector-fill_21__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__vector-fill_21__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__vector-fill_21__description'></a>

#### Description

> ````
> (vector-fill! vector fill)
> (vector-fill! vector fill start)
> (vector-fill! vector fill start end)
> ````
> 
> 
> The `vector-fill!` procedure stores `fill`
> in the elements of `vector`
> between `start` and `end`.
> 
> ````
> (define a (vector 1 2 3 4 5))
> (vector-fill! a 'smash 2 4)
> a  ===>  #(1 2 smash smash 5)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__vector-fill_21__referenced-types'></a>

#### Referenced-types

 * [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
 * [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

