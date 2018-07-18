

<a id='definition__r7rs__delay'></a>

# `delay` -- `r7rs` Definition


<a id='definition__r7rs__delay__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__delay__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `expression`: expression;

Syntax variants:
 * `(_ expression)`


<a id='definition__r7rs__delay__exports'></a>

#### Exports

 * [`scheme:lazy`](../../r7rs/exports/scheme_3a_lazy.md#export__r7rs__scheme_3a_lazy) -- `(scheme lazy)`;


<a id='definition__r7rs__delay__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__delay__description'></a>

#### Description

> ````
> (delay <expression>)
> ````
> 
> 
> **Semantics**:
> The `delay` construct is used together with the procedure `force` to
> implement __lazy evaluation__ or __call by need__.
> `(delay <expression>)` returns an object called a
> __promise__ which at some point in the future can be asked (by
> the `force` procedure) to evaluate
> `<expression>`, and deliver the resulting value.
> The effect of `<expression>` returning multiple values
> is unspecified.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__delay__categories'></a>

#### Categories

 * [`vs:promises`](../../r7rs/categories/vs_3a_promises.md#category__r7rs__vs_3a_promises);
 * [`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);


<a id='definition__r7rs__delay__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

