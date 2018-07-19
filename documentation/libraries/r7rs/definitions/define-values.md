

<a id='definition__r7rs__define-values'></a>

# `define-values` -- `r7rs` Definition


<a id='definition__r7rs__define-values__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__define-values__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `variable`: identifier;
 * `expression`: expression;

Syntax variants:
 * `(_ (variable ...) expression)`


<a id='definition__r7rs__define-values__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__define-values__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__define-values__description'></a>

#### Description

> ````
> (define-values <formals> <expression>)
> ````
> 
> 
> Another kind of definition is provided by `define-values`,
> which creates multiple definitions from a single
> expression returning multiple values.
> It is allowed wherever `define` is allowed.
> 
> It is an error if a variable appears more than once in the set of `<formals>`.
> 
> **Semantics**:
> `<Expression>` is evaluated, and the `<formals>` are bound
> to the return values in the same way that the `<formals>` in a
> `lambda` expression are matched to the arguments in a procedure
> call.
> 
> ````
> (define-values (x y) (integer-sqrt 17))
> (list x y) ===> (4 1)
> 
> (let ()
>   (define-values (x y) (values 1 2))
>   (+ x y))     ===> 3
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__define-values__categories'></a>

#### Categories

 * [`vs:contexts`](../../vonuvoli/categories/vs_3a_contexts.md#category__vonuvoli__vs_3a_contexts);
 * [`vs:values`](../../vonuvoli/categories/vs_3a_values.md#category__vonuvoli__vs_3a_values);


<a id='definition__r7rs__define-values__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

