

<a id='definition__r7rs__call-with-port'></a>

# `call-with-port` -- `r7rs` Definition


<a id='definition__r7rs__call-with-port__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__call-with-port__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((port procedure) -> (any))`
   * inputs:
     * a value of type [`port`](../../r7rs/types/port.md#type__r7rs__port);
     * a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__call-with-port__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__call-with-port__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__call-with-port__description'></a>

#### Description

> ````
> (call-with-port port proc)
> ````
> 
> 
> **Domain**:  It is an error if `proc` does not accept one argument.
> 
> The `call-with-port`
> procedure calls `proc` with `port` as an argument.
> If `proc` returns,
> then the port is closed automatically and the values yielded by the
> `proc` are returned.  If `proc` does not return, then
> the port must not be closed automatically unless it is possible to
> prove that the port will never again be used for a read or write
> operation.
> 
> **Rationale**:  Because Scheme's escape procedures have unlimited extent, it  is
> possible to escape from the current continuation but later to resume it.
> If implementations were permitted to close the port on any escape from the
> current continuation, then it would be impossible to write portable code using
> both `call-with-current-continuation` and `call-with-port`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__call-with-port__referenced-types'></a>

#### Referenced-types

 * [`port`](../../r7rs/types/port.md#type__r7rs__port);
 * [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__call-with-port__categories'></a>

#### Categories

 * [`vs:ports`](../../vonuvoli/categories/vs_3a_ports.md#category__vonuvoli__vs_3a_ports);
 * [`vs:functions`](../../vonuvoli/categories/vs_3a_functions.md#category__vonuvoli__vs_3a_functions);


<a id='definition__r7rs__call-with-port__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

