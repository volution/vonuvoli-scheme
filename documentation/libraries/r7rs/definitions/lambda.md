

<a id='definition__r7rs__lambda'></a>

# `lambda` -- `r7rs` Definitions


<a id='definition__r7rs__lambda__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__lambda__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `argument`: identifier;
 * `argument-rest`: identifier;
 * `arguments`: pattern with variants:
   * `()`;
   * `(argument ...)`;
   * `(argument ... . argument-rest)`;
   * `argument-rest`;
 * `expression`: expression;

Syntax variants:
 * `(_ arguments expression ...)`


<a id='definition__r7rs__lambda__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base);


<a id='definition__r7rs__lambda__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__lambda__description'></a>

#### Description

> ````
> (lambda <formals> <body>)
> ````
> 
> **Syntax**:
> `<Formals>` is a formal arguments list as described below,
> and `<body>` is a sequence of zero or more definitions
> followed by one or more expressions.
> 
> **Semantics**:
> A `lambda` expression evaluates to a procedure.  The environment in
> effect when the `lambda` expression was evaluated is remembered as part of the
> procedure.  When the procedure is later called with some actual
> arguments, the environment in which the `lambda` expression was evaluated will
> be extended by binding the variables in the formal argument list to
> fresh locations, and the corresponding actual argument values will be stored
> in those locations.
> (A __fresh__ location is one that is distinct from every previously
> existing location.)
> Next, the expressions in the
> body of the lambda expression (which, if it contains definitions,
> represents a `letrec*` form, see section on `letrec*`)
> will be evaluated sequentially in the extended environment.
> The results of the last expression in the body will be returned as
> the results of the procedure call.
> 
> ````
> (lambda (x) (+ x x))      ===>  #procedure
> ((lambda (x) (+ x x)) 4)  ===>  8
> 
> (define reverse-subtract
>   (lambda (x y) (- y x)))
> (reverse-subtract 7 10)         ===>  3
> 
> (define add4
>   (let ((x 4))
>     (lambda (y) (+ x y))))
> (add4 6)                        ===>  10
> ````
> 
> `<Formals>` have one of the following forms:
> 
>   * `(variable_1 ...)`:
> The procedure takes a fixed number of arguments; when the procedure is
> called, the arguments will be stored in fresh locations
> that are bound to the corresponding variables.
> 
>   * `<variable>`:
> The procedure takes any number of arguments; when the procedure is
> called, the sequence of actual arguments is converted into a newly
> allocated list, and the list is stored in a fresh location
> that is bound to
> `<variable>`.
> 
>   * `(variable_1 ... <variable_n> . <variable_n+1>)`:
> If a space-delimited period precedes the last variable, then
> the procedure takes `n` or more arguments, where `n` is the
> number of formal arguments before the period (it is an error if there is not
> at least one).
> The value stored in the binding of the last variable will be a
> newly allocated
> list of the actual arguments left over after all the other actual
> arguments have been matched up against the other formal arguments.
> 
> It is an error for a `<variable>` to appear more than once in
> `<formals>`.
> 
> ````
> ((lambda x x) 3 4 5 6)          ===>  (3 4 5 6)
> ((lambda (x y . z) z)
>  3 4 5 6)                       ===>  (5 6)
> ````
> 
> Each procedure created as the result of evaluating a `lambda` expression is
> (conceptually) tagged
> with a storage location, in order to make `eqv?` and
> `eq?` work on procedures (see section on equivalence predicates).
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__lambda__categories'></a>

#### Categories

 * [`vs:lambda`](../../r7rs/categories/vs_3a_lambda.md#category__r7rs__vs_3a_lambda);


<a id='definition__r7rs__lambda__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

