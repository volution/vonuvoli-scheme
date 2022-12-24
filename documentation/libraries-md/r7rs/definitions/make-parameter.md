

<a id='definition__r7rs__make-parameter'></a>

# `make-parameter` -- `r7rs` Definition


<a id='definition__r7rs__make-parameter__kind'></a>

#### Kind

`constructor`;


<a id='definition__r7rs__make-parameter__implemented-by'></a>

#### Implemented by

 * [`make-parameter`](../../vonuvoli/definitions/make-parameter.md#definition__vonuvoli__make-parameter) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__make-parameter__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(((initial any)) -> (parameter))`
   * input: `initial` of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`parameter`](../../r7rs/types/parameter.md#type__r7rs__parameter);
 * `(((initial any) (converter procedure-1)) -> (parameter))`
   * inputs:
     * `initial` of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `converter` of type [`procedure-1`](../../r7rs/types/procedure-1.md#type__r7rs__procedure-1);
   * output: a value of type [`parameter`](../../r7rs/types/parameter.md#type__r7rs__parameter);


<a id='definition__r7rs__make-parameter__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__make-parameter__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__make-parameter__description'></a>

#### Description

> ````
> (make-parameter init)
> (make-parameter init converter)
> ````
> 
> 
> Returns a newly allocated parameter object,
> which is a procedure that accepts zero arguments and
> returns the value associated with the parameter object.
> Initially, this value is the value of
> `(converter init)`, or of `init`
> if the conversion procedure `converter` is not specified.
> The associated value can be temporarily changed
> using `parameterize`, which is described below.
> 
> The effect of passing arguments to a parameter object is
> implementation-dependent.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__make-parameter__referenced-types'></a>

#### Referenced-types

 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`parameter`](../../r7rs/types/parameter.md#type__r7rs__parameter);
 * [`procedure-1`](../../r7rs/types/procedure-1.md#type__r7rs__procedure-1);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

