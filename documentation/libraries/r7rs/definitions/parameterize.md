

<a id='definition__r7rs__parameterize'></a>

# `parameterize` -- `r7rs` Definition


<a id='definition__r7rs__parameterize__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__parameterize__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `parameter`: expression;
 * `initializer`: expression;
 * `parameters`: pattern with variants:
   * `()`;
   * `((parameter initializer) ...)`;
 * `expression`: expression;

Syntax variants:
 * `(_ parameters)`
 * `(_ parameters expression ...)`


<a id='definition__r7rs__parameterize__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__parameterize__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__parameterize__description'></a>

#### Description

> ````
> (parameterize ((<param_1> <value_1>) ...)
>         <body>)
> ````
> 
> **Syntax**:
> Both `<param_1>` and `<value_1>` are expressions.
> 
> **Domain**:
> It is an error if the value of any `<param>` expression is not a parameter object.
> 
> **Semantics**:
> A `parameterize` expression is used to change the values returned by
> specified parameter objects during the evaluation of the body.
> 
> The `<param>` and `<value>` expressions
> are evaluated in an unspecified order.  The `<body>` is
> evaluated in a dynamic environment in which calls to the
> parameters return the results of passing the corresponding values
> to the conversion procedure specified when the parameters were created.
> Then the previous values of the parameters are restored without passing
> them to the conversion procedure.
> The results of the last
> expression in the `<body>` are returned as the results of the entire
> `parameterize` expression.
> 
> **Note**:
> If the conversion procedure is not idempotent, the results of
> `(parameterize ((x (x))) ...)`,
> which appears to bind the parameter `x` to its current value,
> might not be what the user expects.
> 
> If an implementation supports multiple threads of execution, then
> `parameterize` must not change the associated values of any parameters
> in any thread other than the current thread and threads created
> inside `<body>`.
> 
> Parameter objects can be used to specify configurable settings for a
> computation without the need to pass the value to every
> procedure in the call chain explicitly.
> 
> ````
> (define radix
>   (make-parameter
>    10
>    (lambda (x)
>      (if (and (exact-integer? x) (<= 2 x 16))
>          x
>          (error "invalid radix")))))
> 
> (define (f n) (number->string n (radix)))
> 
> (f 12)                                       ===> "12"
> (parameterize ((radix 2))
>   (f 12))                                    ===> "1100"
> (f 12)                                       ===> "12"
> 
> (radix 16)                                   ===> #unspecified
> 
> (parameterize ((radix 0))
>   (f 12))                                    ===> #error
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__parameterize__categories'></a>

#### Categories

 * [`vs:parameters`](../../r7rs/categories/vs_3a_parameters.md#category__r7rs__vs_3a_parameters);


<a id='definition__r7rs__parameterize__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

