

<a id='definition__r7rs__open-input-file'></a>

# `open-input-file` -- `r7rs` Definitions


<a id='definition__r7rs__open-input-file__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__open-input-file__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((path-string) -> (textual-input-port-open))`
   * input: a value of type [`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
   * output: a value of type [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);


<a id='definition__r7rs__open-input-file__exports'></a>

#### Exports

 * [`scheme:file`](../../r7rs/exports/scheme_3a_file.md#export__r7rs__scheme_3a_file) -- `(scheme file)`;


<a id='definition__r7rs__open-input-file__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__open-input-file__description'></a>

#### Description

> ````
> (open-input-file string)
> (open-binary-input-file string)
> ````
> 
> 
> Takes a `string` for an existing file and returns a textual
> input port or binary input port that is capable of delivering data from the
> file.  If the file does not exist or cannot be opened, an error that satisfies `file-error?` is signaled.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__open-input-file__referenced-types'></a>

#### Referenced-types

 * [`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
 * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);


<a id='definition__r7rs__open-input-file__categories'></a>

#### Categories

 * [`vs:ports:input`](../../r7rs/categories/vs_3a_ports_3a_input.md#category__r7rs__vs_3a_ports_3a_input);
 * [`vs:ports:open`](../../r7rs/categories/vs_3a_ports_3a_open.md#category__r7rs__vs_3a_ports_3a_open);


<a id='definition__r7rs__open-input-file__categories-recursive'></a>

#### Categories recursive

 * [`vs:ports`](../../r7rs/categories/vs_3a_ports.md#category__r7rs__vs_3a_ports);
 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

