

<a id='type__r7rs__exception-handler'></a>

# `exception-handler` -- `r7rs` Type


<a id='type__r7rs__exception-handler__super-types'></a>

#### Super-types

 * [`procedure-1`](../../r7rs/types/procedure-1.md#type__r7rs__procedure-1);


<a id='type__r7rs__exception-handler__super-types-recursive'></a>

##### Super-types recursive

 * [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);


<a id='type__r7rs__exception-handler__referent-definitions-input'></a>

#### Referent definitions as input

 * [`with-exception-handler`](../../r7rs/definitions/with-exception-handler.md#definition__r7rs__with-exception-handler);


<a id='type__r7rs__exception-handler__referent-definitions-input-recursive'></a>

#### Referent definitions as input (recursive)

 * [`procedure?`](../../r7rs/definitions/procedure_3f.md#definition__r7rs__procedure_3f);
 * [`apply`](../../r7rs/definitions/apply.md#definition__r7rs__apply);
 * [`call-with-values`](../../r7rs/definitions/call-with-values.md#definition__r7rs__call-with-values);
 * [`call-with-port`](../../r7rs/definitions/call-with-port.md#definition__r7rs__call-with-port);
 * [`call-with-input-file`](../../r7rs/definitions/call-with-input-file.md#definition__r7rs__call-with-input-file);
 * [`call-with-output-file`](../../r7rs/definitions/call-with-output-file.md#definition__r7rs__call-with-output-file);
 * [`make-parameter`](../../r7rs/definitions/make-parameter.md#definition__r7rs__make-parameter);
 * [`call-with-current-continuation`](../../r7rs/definitions/call-with-current-continuation.md#definition__r7rs__call-with-current-continuation);

Note:  These definitions consume an input that is a super-type.


<a id='type__r7rs__exception-handler__description'></a>

#### Description

> __Exception handler__'s are one-argument procedures that determine the
> action the program takes when an exceptional situation is signaled.
> The system implicitly maintains a current exception handler
> in the dynamic environment.
> 
> The program raises an exception by
> invoking the __current exception handler__, passing it an object
> encapsulating information about the exception.  Any procedure
> accepting one argument can serve as an exception handler and any
> object can be used to represent an exception.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='type__r7rs__exception-handler__categories'></a>

#### Categories

 * [`types-miscellaneous`](../../r7rs/categories/types-miscellaneous.md#category__r7rs__types-miscellaneous);


<a id='type__r7rs__exception-handler__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

