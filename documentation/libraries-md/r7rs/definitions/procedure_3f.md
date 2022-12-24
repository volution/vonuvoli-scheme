

<a id='definition__r7rs__procedure_3f'></a>

# `procedure?` -- `r7rs` Definition


<a id='definition__r7rs__procedure_3f__kind'></a>

#### Kind

`type-predicate`;


<a id='definition__r7rs__procedure_3f__extended-by'></a>

#### Extended by

 * [`procedure?`](../../vonuvoli/definitions/procedure_3f.md#definition__vonuvoli__procedure_3f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__procedure_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((procedure) -> (true))`
   * input: a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((any) -> (false))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__procedure_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__procedure_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__procedure_3f__description'></a>

#### Description

> ````
> (procedure? obj)
> ````
> 
> 
> Returns `#t` if `obj` is a procedure, otherwise returns `#f`.
> 
> ````
> (procedure? car)            ===>  #t
> (procedure? 'car)           ===>  #f
> (procedure? (lambda (x) (* x x)))
>                             ===>  #t
> (procedure? '(lambda (x) (* x x)))
>                             ===>  #f
> (call-with-current-continuation procedure?)
>                             ===>  #t
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__procedure_3f__referenced-types'></a>

#### Referenced-types

 * [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

