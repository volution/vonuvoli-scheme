

<a id='definition__r7rs__map'></a>

# `map` -- `r7rs` Definition


<a id='definition__r7rs__map__kind'></a>

#### Kind

`functor`;


<a id='definition__r7rs__map__implemented-by'></a>

#### Implemented by

 * [`map`](../../vonuvoli/definitions/map.md#definition__vonuvoli__map) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__map__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((map-procedure list |1...|) -> (list-proper))`
   * inputs:
     * a value of type [`map-procedure`](../../r7rs/types/map-procedure.md#type__r7rs__map-procedure);
     * a value of type [`list`](../../r7rs/types/list.md#type__r7rs__list);
     * `...` -- at least one time;
   * output: a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);


<a id='definition__r7rs__map__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__map__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__map__description'></a>

#### Description

> ````
> (map proc list_1 list_2 ...)
> ````
> 
> 
> **Domain**:  It is an error if `proc` does not
> accept as many arguments as there are `list`s
> and return a single value.
> 
> The `map` procedure applies `proc` element-wise to the elements of the
> `list`s and returns a list of the results, in order.
> If more than one `list` is given and not all lists have the same length,
> `map` terminates when the shortest list runs out.
> The `list`s can be circular, but it is an error if all of them are circular.
> It is an error for `proc` to mutate any of the lists.
> The dynamic order in which `proc` is applied to the elements of the
> `list`s is unspecified.  If multiple returns occur from `map`,
> the values returned by earlier returns are not mutated.
> 
> ````
> (map cadr '((a b) (d e) (g h)))   ===>  (b e h)
> 
> (map (lambda (n) (expt n n))
>      '(1 2 3 4 5))                ===>  (1 4 27 256 3125)
> 
> (map + '(1 2 3) '(4 5 6 7))         ===>  (5 7 9)
> 
> (let ((count 0))
>   (map (lambda (ignored)
>          (set! count (+ count 1))
>          count)
>        '(a b)))                 ===>  (1 2) or (2 1)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__map__referenced-types'></a>

#### Referenced-types

 * [`map-procedure`](../../r7rs/types/map-procedure.md#type__r7rs__map-procedure);
 * [`list`](../../r7rs/types/list.md#type__r7rs__list);
 * [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

