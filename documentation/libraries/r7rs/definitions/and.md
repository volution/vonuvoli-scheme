

<a id='definition__r7rs__and'></a>

# `and` -- `r7rs` Definition


<a id='definition__r7rs__and__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__and__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `expression`: expression;

Syntax variants:
 * `(_)`
 * `(_ expression ...)`


<a id='definition__r7rs__and__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__and__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__and__description'></a>

#### Description

> ````
> (and <test_1> ...)
> ````
> 
> 
> **Semantics**:
> The `<test>` expressions are evaluated from left to right, and if
> any expression evaluates to `#f` (see
> section on booleans), then `#f` is returned.  Any remaining
> expressions are not evaluated.  If all the expressions evaluate to
> true values, the values of the last expression are returned.  If there
> are no expressions, then `#t` is returned.
> 
> ````
> (and (= 2 2) (> 2 1))           ===>  #t
> (and (= 2 2) (< 2 1))           ===>  #f
> (and 1 2 'c '(f g))             ===>  (f g)
> (and)                           ===>  #t
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__and__categories'></a>

#### Categories

 * [`vs:control`](../../vonuvoli/categories/vs_3a_control.md#category__vonuvoli__vs_3a_control);


<a id='definition__r7rs__and__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

