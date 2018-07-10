

<a id='definition__r7rs__for-each'></a>

# `for-each` -- `r7rs` Definitions


#### Kind

`functor`;


#### Procedure signature

Procedure variants:
 * `((|procedure| |list| |...|) |->| (|undefined|))`
   * inputs:
     * a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
     * a value of type [`list`](../../r7rs/types/list.md#type__r7rs__list);
     * `...` (i.e. variadic);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
   * requires: `(|not| |vonuvoli|)`
 * `((|procedure| |list| |...|) |->| (|void|))`
   * inputs:
     * a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
     * a value of type [`list`](../../r7rs/types/list.md#type__r7rs__list);
     * `...` (i.e. variadic);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
   * requires: `|vonuvoli|`


#### Referenced types

[`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
[`list`](../../r7rs/types/list.md#type__r7rs__list);
[`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
[`void`](../../r7rs/types/void.md#type__r7rs__void);


#### Description

> ````
> (for-each proc list_1 list_2 ...)
> ````
> 
> 
> **Domain**:  It is an error if `proc` does not
> accept as many arguments as there are `list`s.
> 
> The arguments to `for-each` are like the arguments to `map`, but
> `for-each` calls `proc` for its side effects rather than for its
> values.  Unlike `map`, `for-each` is guaranteed to call `proc` on
> the elements of the `list`s in order from the first element(s) to the
> last, and the value returned by `for-each` is unspecified.
> If more than one `list` is given and not all lists have the same length,
> `for-each` terminates when the shortest list runs out.
> The `list`s can be circular, but it is an error if all of them are circular.
> 
> It is an error for `proc` to mutate any of the lists.
> 
> ````
> (let ((v (make-vector 5)))
>   (for-each (lambda (i)
>               (vector-set! v i (* i i)))
>             '(0 1 2 3 4))
>   v)                                ===>  #(0 1 4 9 16)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:lists`](../../r7rs/categories/vs_3a_lists.md#category__r7rs__vs_3a_lists);
[`vs:functions`](../../r7rs/categories/vs_3a_functions.md#category__r7rs__vs_3a_functions);
[`vs:loops`](../../r7rs/categories/vs_3a_loops.md#category__r7rs__vs_3a_loops);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

