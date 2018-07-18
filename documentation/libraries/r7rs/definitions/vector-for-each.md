

<a id='definition__r7rs__vector-for-each'></a>

# `vector-for-each` -- `r7rs` Definition


<a id='definition__r7rs__vector-for-each__kind'></a>

#### Kind

`functor`;


<a id='definition__r7rs__vector-for-each__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((procedure vector ...) -> (undefined))`
   * inputs:
     * a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `...` (i.e. variadic);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
   * requires: `(not vonuvoli)`
 * `((procedure vector ...) -> (void))`
   * inputs:
     * a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `...` (i.e. variadic);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
   * requires: `vonuvoli`


<a id='definition__r7rs__vector-for-each__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__vector-for-each__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__vector-for-each__description'></a>

#### Description

> ````
> (vector-for-each proc vector_1 vector_2 ...)
> ````
> 
> 
> **Domain**:  It is an error if `proc` does not
> accept as many arguments as there are `vector`s.
> 
> The arguments to `vector-for-each` are like the arguments to
> `vector-map`, but `vector-for-each` calls `proc` for its side
> effects rather than for its values.  Unlike `vector-map`,
> `vector-for-each` is guaranteed to call `proc` on the elements of
> the `vector`s in order from the first element(s) to the last, and
> the value returned by `vector-for-each` is unspecified.
> If more than one `vector` is given and not all vectors have the same length,
> `vector-for-each` terminates when the shortest vector runs out.
> It is an error for `proc` to mutate any of the vectors.
> 
> ````
> (let ((v (make-list 5)))
>   (vector-for-each
>    (lambda (i) (list-set! v i (* i i)))
>    '#(0 1 2 3 4))
>   v)                                ===>  (0 1 4 9 16)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__vector-for-each__referenced-types'></a>

#### Referenced-types

 * [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
 * [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
 * [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * [`void`](../../r7rs/types/void.md#type__r7rs__void);


<a id='definition__r7rs__vector-for-each__categories'></a>

#### Categories

 * [`vs:vectors`](../../r7rs/categories/vs_3a_vectors.md#category__r7rs__vs_3a_vectors);
 * [`vs:functions`](../../r7rs/categories/vs_3a_functions.md#category__r7rs__vs_3a_functions);
 * [`vs:loops`](../../r7rs/categories/vs_3a_loops.md#category__r7rs__vs_3a_loops);


<a id='definition__r7rs__vector-for-each__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

