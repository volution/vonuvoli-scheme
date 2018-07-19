

<a id='definition__r7rs__delay-force'></a>

# `delay-force` -- `r7rs` Definition


<a id='definition__r7rs__delay-force__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__delay-force__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `expression`: expression;

Syntax variants:
 * `(_ expression)`


<a id='definition__r7rs__delay-force__exports'></a>

#### Exports

 * [`scheme:lazy`](../../r7rs/exports/scheme_3a_lazy.md#export__r7rs__scheme_3a_lazy) -- `(scheme lazy)`;


<a id='definition__r7rs__delay-force__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__delay-force__description'></a>

#### Description

> ````
> (delay-force <expression>)
> ````
> 
> 
> **Semantics**:
> The expression `(delay-force expression)` is conceptually similar to
> `(delay (force expression))`,
> with the difference that forcing the result
> of `delay-force` will in effect result in a tail call to
> `(force expression)`,
> while forcing the result of
> `(delay (force expression))`
> might not.  Thus
> iterative lazy algorithms that might result in a long series of chains of
> `delay` and `force`
> can be rewritten using `delay-force` to prevent consuming
> unbounded space during evaluation.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__delay-force__categories'></a>

#### Categories

 * [`vs:promises`](../../vonuvoli/categories/vs_3a_promises.md#category__vonuvoli__vs_3a_promises);
 * [`vs:evaluator`](../../vonuvoli/categories/vs_3a_evaluator.md#category__vonuvoli__vs_3a_evaluator);


<a id='definition__r7rs__delay-force__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

