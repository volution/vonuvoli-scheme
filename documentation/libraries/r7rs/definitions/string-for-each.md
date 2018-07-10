

<a id='definition__r7rs__string-for-each'></a>

# `string-for-each` -- `r7rs` Definitions


#### Kind

`functor`;


#### Procedure signature

Procedure variants:
 * `((|procedure| |string| |...|) |->| (|undefined|))`
   * inputs:
     * a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `...` (i.e. variadic);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
   * requires: `(|not| |vonuvoli|)`
 * `((|procedure| |string| |...|) |->| (|void|))`
   * inputs:
     * a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `...` (i.e. variadic);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
   * requires: `|vonuvoli|`


#### Referenced types

[`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
[`string`](../../r7rs/types/string.md#type__r7rs__string);
[`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
[`void`](../../r7rs/types/void.md#type__r7rs__void);


#### Description

> ````
> (string-for-each proc string_1 string_2 ...)
> ````
> 
> 
> **Domain**:  It is an error if `proc` does not
> accept as many arguments as there are `string`s.
> 
> The arguments to `string-for-each` are like the arguments to
> `string-map`, but `string-for-each` calls `proc` for its side
> effects rather than for its values.  Unlike `string-map`,
> `string-for-each` is guaranteed to call `proc` on the elements of
> the `list`s in order from the first element(s) to the last, and the
> value returned by `string-for-each` is unspecified.
> If more than one `string` is given and not all strings have the same length,
> `string-for-each` terminates when the shortest string runs out.
> It is an error for `proc` to mutate any of the strings.
> 
> ````
> (let ((v '()))
>   (string-for-each
>    (lambda (c) (set! v (cons (char->integer c) v)))
>    "abcde")
>   v)                         ===>  (101 100 99 98 97)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);
[`vs:functions`](../../r7rs/categories/vs_3a_functions.md#category__r7rs__vs_3a_functions);
[`vs:loops`](../../r7rs/categories/vs_3a_loops.md#category__r7rs__vs_3a_loops);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

