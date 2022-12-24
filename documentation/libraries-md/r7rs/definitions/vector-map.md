

<a id='definition__r7rs__vector-map'></a>

# `vector-map` -- `r7rs` Definition


<a id='definition__r7rs__vector-map__kind'></a>

#### Kind

`functor`;


<a id='definition__r7rs__vector-map__implemented-by'></a>

#### Implemented by

 * [`vector-map`](../../vonuvoli/definitions/vector-map.md#definition__vonuvoli__vector-map) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__vector-map__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((map-procedure vector |1...|) -> (vector))`
   * inputs:
     * a value of type [`map-procedure`](../../r7rs/types/map-procedure.md#type__r7rs__map-procedure);
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `...` -- at least one time;
   * output: a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);


<a id='definition__r7rs__vector-map__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__vector-map__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__vector-map__description'></a>

#### Description

> ````
> (vector-map proc vector_1 vector_2 ...)
> ````
> 
> 
> **Domain**:  It is an error if `proc` does not
> accept as many arguments as there are `vector`s
> and return a single value.
> 
> The `vector-map` procedure applies `proc` element-wise to the elements of the
> `vector`s and returns a vector of the results, in order.
> If more than one `vector` is given and not all vectors have the same length,
> `vector-map` terminates when the shortest vector runs out.
> The dynamic order in which `proc` is applied to the elements of the
> `vector`s is unspecified.
> If multiple returns occur from `vector-map`,
> the values returned by earlier returns are not mutated.
> 
> ````
> (vector-map cadr '#((a b) (d e) (g h)))   ===>  #(b e h)
> 
> (vector-map (lambda (n) (expt n n))
>             '#(1 2 3 4 5))                ===>  #(1 4 27 256 3125)
> 
> (vector-map + '#(1 2 3) '#(4 5 6 7))       ===>  #(5 7 9)
> 
> (let ((count 0))
>   (vector-map
>    (lambda (ignored)
>      (set! count (+ count 1))
>      count)
>    '#(a b)))                     ===>  #(1 2) or #(2 1)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__vector-map__referenced-types'></a>

#### Referenced-types

 * [`map-procedure`](../../r7rs/types/map-procedure.md#type__r7rs__map-procedure);
 * [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

