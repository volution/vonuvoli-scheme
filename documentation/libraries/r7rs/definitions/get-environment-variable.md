

<a id='definition__r7rs__get-environment-variable'></a>

# `get-environment-variable` -- `r7rs` Definitions


<a id='definition__r7rs__get-environment-variable__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__get-environment-variable__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((string) -> (string-or-false))`
   * input: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);
   * output: a value of type [`string-or-false`](../../r7rs/types/string-or-false.md#type__r7rs__string-or-false);


<a id='definition__r7rs__get-environment-variable__exports'></a>

#### Exports

 * [`scheme:process-context`](../../r7rs/exports/scheme_3a_process-context.md#export__r7rs__scheme_3a_process-context) -- `(scheme process-context)`;


<a id='definition__r7rs__get-environment-variable__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__get-environment-variable__description'></a>

#### Description

> ````
> (get-environment-variable name)
> ````
> 
> 
> Many operating systems provide each running process with an
> __environment__ consisting of __environment variables__.
> (This environment is not to be confused with the Scheme environments that
> can be passed to `eval`: see section on environments and evaluation.)
> Both the name and value of an environment variable are strings.
> The procedure `get-environment-variable` returns the value
> of the environment variable `name`,
> or `#f` if the named
> environment variable is not found.  It may
> use locale information to encode the name and decode the value
> of the environment variable.  It is an error if
> `get-environment-variable` can't decode the value.
> It is also an error to mutate the resulting string.
> 
> ````
> (get-environment-variable "PATH") ===> "/usr/local/bin:/usr/bin:/bin"
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__get-environment-variable__referenced-types'></a>

#### Referenced-types

 * [`string`](../../r7rs/types/string.md#type__r7rs__string);
 * [`string-or-false`](../../r7rs/types/string-or-false.md#type__r7rs__string-or-false);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

