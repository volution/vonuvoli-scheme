

<a id='category__r7rs__r7rs_3a_expressions-syntax-bindings'></a>

# `r7rs:expressions-syntax-bindings` -- `r7rs` Categories


<a id='category__r7rs__r7rs_3a_expressions-syntax-bindings__description'></a>

#### Description

> ##### Macros
> 
> Scheme programs can define and use new derived expression types,
>  called __macros__.
> Program-defined expression types have the syntax
> ````
> (<keyword> <datum> ...)
> ````
> where `<keyword>` is an identifier that uniquely determines the
> expression type.  This identifier is called the
> __syntactic keyword__, or simply
> __keyword__, of the macro.  The
> number of the `<datum>`s, and their syntax, depends on the
> expression type.
> 
> Each instance of a macro is called a __use__
> of the macro.
> The set of rules that specifies
> how a use of a macro is transcribed into a more primitive expression
> is called the __transformer__
> of the macro.
> 
> The macro definition facility consists of two parts:
> 
>   * A set of expressions used to establish that certain identifiers
> are macro keywords, associate them with macro transformers, and control
> the scope within which a macro is defined, and
> 
>   * a pattern language for specifying macro transformers.
> 
> The syntactic keyword of a macro can shadow variable bindings, and local
> variable bindings can shadow syntactic bindings.
> Two mechanisms are provided to prevent unintended conflicts:
> 
>   * If a macro transformer inserts a binding for an identifier
> (variable or keyword), the identifier will in effect be renamed
> throughout its scope to avoid conflicts with other identifiers.
> Note that a global variable definition may or may not introduce a binding;
> see section variable definitions.
> 
>   * If a macro transformer inserts a free reference to an
> identifier, the reference refers to the binding that was visible
> where the transformer was specified, regardless of any local
> bindings that surround the use of the macro.
> 
> In consequence, all macros
> defined using the pattern language  are "hygienic" and "referentially
> transparent" and thus preserve Scheme's lexical scoping.
> 
> Implementations may provide macro facilities of other types.
> 
> ----
> 
> ##### Binding constructs for syntactic keywords
> 
> The `let-syntax` and `letrec-syntax` binding constructs are
> analogous to `let` and `letrec`, but they bind
> syntactic keywords to macro transformers instead of binding variables
> to locations that contain values.  Syntactic keywords can also be
> bound globally or locally with `define-syntax`;
> see section on `define-syntax`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='category__r7rs__r7rs_3a_expressions-syntax-bindings__super-categories'></a>

#### Super-categories

 * [`r7rs:expressions`](../../r7rs/categories/r7rs_3a_expressions.md#category__r7rs__r7rs_3a_expressions);


<a id='category__r7rs__r7rs_3a_expressions-syntax-bindings__super-categories-recursive'></a>

##### Super-categories recursive

 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

