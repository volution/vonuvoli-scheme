

<a id='definition__r7rs__string-for-each'></a>

# `string-for-each` -- `r7rs` Definition


<a id='definition__r7rs__string-for-each__kind'></a>

#### Kind

`functor`;


<a id='definition__r7rs__string-for-each__extended-by'></a>

#### Extended by

 * [`string-for-each`](../../vonuvoli/definitions/string-for-each.md#definition__vonuvoli__string-for-each) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__string-for-each__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((for-each-procedure string |1...|) -> (undefined))`
   * inputs:
     * a value of type [`for-each-procedure`](../../r7rs/types/for-each-procedure.md#type__r7rs__for-each-procedure);
     * a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
     * `...` -- at least one time;
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


<a id='definition__r7rs__string-for-each__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__string-for-each__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__string-for-each__description'></a>

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


<a id='definition__r7rs__string-for-each__referenced-types'></a>

#### Referenced-types

 * [`for-each-procedure`](../../r7rs/types/for-each-procedure.md#type__r7rs__for-each-procedure);
 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

