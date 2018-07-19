

<a id='definition__r7rs__open-output-file'></a>

# `open-output-file` -- `r7rs` Definition


<a id='definition__r7rs__open-output-file__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__open-output-file__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((path-string) -> (textual-output-port-open))`
   * input: a value of type [`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
   * output: a value of type [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);


<a id='definition__r7rs__open-output-file__exports'></a>

#### Exports

 * [`scheme:file`](../../r7rs/exports/scheme_3a_file.md#export__r7rs__scheme_3a_file) -- `(scheme file)`;


<a id='definition__r7rs__open-output-file__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__open-output-file__description'></a>

#### Description

> ````
> (open-output-file string)
> (open-binary-output-file string)
> ````
> 
> 
> Takes a `string` naming an output file to be created and returns a
> textual output port or binary output port that is capable of writing
> data to a new file by that name.
> 
> If a file with the given name already exists,
> the effect is unspecified.
> If the file cannot be opened,
> an error that satisfies `file-error?` is signaled.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__open-output-file__referenced-types'></a>

#### Referenced-types

 * [`path-string`](../../r7rs/types/path-string.md#type__r7rs__path-string);
 * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);


<a id='definition__r7rs__open-output-file__categories'></a>

#### Categories

 * [`vs:ports:output`](../../vonuvoli/categories/vs_3a_ports_3a_output.md#category__vonuvoli__vs_3a_ports_3a_output);
 * [`vs:ports:open`](../../vonuvoli/categories/vs_3a_ports_3a_open.md#category__vonuvoli__vs_3a_ports_3a_open);


<a id='definition__r7rs__open-output-file__categories-recursive'></a>

#### Categories recursive

 * [`vs:ports`](../../vonuvoli/categories/vs_3a_ports.md#category__vonuvoli__vs_3a_ports);
 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

