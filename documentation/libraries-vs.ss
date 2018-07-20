
(library
	
	
	
	
	(identifier vonuvoli)
	
	(title "Vonuvoli-Scheme builtin functionality")
	
	(description
		#<<<
					
					**FIXME!**
					
		>>>#)
	
	
	
	
	(categories
		
		(vs
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(vs:arithmetic
			(parent vs))
		
		(vs:associations
			(parent vs))
		
		(vs:bytes
			(parent vs))
		
		(vs:booleans
			(parent vs))
		
		(vs:conversions
			(parent vs))
		
		(vs:globals
			(parent vs))
		
		(vs:file-system
			(parent vs))
		
		(vs:characters
			(parent vs))
		
		(vs:comparisons
			(parent vs))
		
		(vs:compiler
			(parent vs))
		
		(vs:contexts
			(parent vs))
		
		(vs:continuations
			(parent vs))
		
		(vs:control
			(parent vs))
		
		(vs:equivalence
			(parent vs)
			(description
				#<<<
					
					A **predicate** is a procedure that always returns a boolean
					value (`#t` or `#f`).  An **equivalence predicate** is
					the computational analogue of a mathematical equivalence relation; it is
					symmetric, reflexive, and transitive.
					
					Of the equivalence predicates
					described in this section, `eq?` is the finest or most
					discriminating, `equal?` is the coarsest, and `eqv?` is
					slightly less discriminating than `eq?`.
					
					
					----
					> *The text herein was sourced and adapted as described in the [[attribution]](#appendices) appendix.*
					
				>>>#))
		
		(vs:errors
			(parent vs))
		
		(vs:evaluator
			(parent vs))
		
		(vs:functions
			(parent vs))
		
		(vs:lambda
			(parent vs))
		
		(vs:lists
			(parent vs))
		
		(vs:loops
			(parent vs))
		
		(vs:modules
			(parent vs))
		
		(vs:pairs
			(parent vs))
		
		(vs:parameters
			(parent vs))
		
		(vs:ports
			(parent vs))
		
		(vs:ports:input
			(parent vs:ports))
		
		(vs:ports:output
			(parent vs:ports))
		
		(vs:ports:open
			(parent vs:ports))
		
		(vs:ports:values
			(parent vs:ports))
		
		(vs:promises
			(parent vs))
		
		(vs:quotation
			(parent vs))
		
		(vs:records
			(parent vs))
		
		(vs:strings
			(parent vs))
		
		(vs:symbols
			(parent vs))
		
		(vs:syntaxes
			(parent vs))
		
		(vs:system
			(parent vs))
		
		(vs:types
			(parent vs))
		
		(vs:unimplemented
			(parent vs))
		
		(vs:unsupported
			(parent vs))
		
		(vs:values
			(parent vs))
		
		(vs:vectors
			(parent vs))
		
	)
	
	
	
	
	(definitions
		
		
		
		
		;; ---- syntaxes
		
		
		;~ quote
		;~ quasiquote
		;~ unquote
		;~ unquote-splicing
		
		;~ ...
		;~ =>
		;~ _
		;~ else
		
		
		;~ begin
		;~ and
		;~ or
		
		;~ if
		;~ when
		;~ unless
		;~ cond
		;~ case
		
		
		;~ do
		
		(while
			(type syntax))
		
		(until
			(type syntax))
		
		
		(do-cond
			(type syntax))
		
		(while-cond
			(type syntax))
		
		(until-cond
			(type syntax))
		
		
		(loop
			(type syntax))
		
		
		;~ guard
		
		(guard*
			(type syntax))
		
		
		(locals
			(type syntax))
		
		;~ let
		;~ let*
		;~ letrec
		;~ letrec*
		;~ let-values
		;~ let*-values
		
		
		;~ define
		
		(redefine
			(type syntax))
		
		;~ define-values
		
		(redefine-values
			(type syntax))
		
		;~ set!
		
		(set!-values
			(type syntax))
		
		
		;~ define-record
		
		
		;~ lambda
		
		
		;~ parameterize
		
		
		
		
		;; ---- types
		
		
		;~ not
		
		
		;~ null?
		
		(void?
			(type type-predicate))
		
		(undefined?
			(type type-predicate))
		
		
		;~ boolean?
		
		(true?
			(type predicate))
		
		(false?
			(type predicate))
		
		(true-or-equivalent?
			(type predicate))
		
		(false-or-equivalent?
			(type predicate))
		
		
		;~ number?
		;~ complex?
		;~ real?
		;~ rational?
		;~ integer?
		;~ exact-integer?
		
		;~ exact?
		;~ inexact?
		
		
		;~ char?
		;~ symbol?
		
		(keyword?
			(type type-predicate))
		
		(unique?
			(type type-predicate))
		
		
		;~ string?
		
		(string-immutable?
			(type type-predicate))
		
		(string-mutable?
			(type type-predicate))
		
		(string-empty?
			(type predicate))
		
		(string-immutable-empty?
			(type predicate))
		
		(string-mutable-empty?
			(type predicate))
		
		
		;~ bytevector?
		
		(bytevector-immutable?
			(type type-predicate))
		
		(bytevector-mutable?
			(type type-predicate))
		
		(bytevector-empty?
			(type predicate))
		
		(bytevector-immutable-empty?
			(type predicate))
		
		(bytevector-mutable-empty?
			(type predicate))
		
		
		(string-regex?
			(type type-predicate))
		
		(bytevector-regex?
			(type type-predicate))
		
		
		;~ pair?
		
		(pair-immutable?
			(type type-predicate))
		
		(pair-mutable?
			(type type-predicate))
		
		
		;~ vector?
		
		(vector-immutable?
			(type type-predicate))
		
		(vector-mutable?
			(type type-predicate))
		
		(vector-empty?
			(type predicate))
		
		(vector-immutable-empty?
			(type predicate))
		
		(vector-mutable-empty?
			(type predicate))
		
		
		(values?
			(type type-predicate))
		
		(values-empty?
			(type predicate))
		
		
		(record-type?
			(type type-predicate))
		
		(record?
			(type type-predicate))
		
		(record-immutable?
			(type type-predicate))
		
		(record-mutable?
			(type type-predicate))
		
		
		;~ error-object?
		
		(syntax-error?
			(type predicate))
		
		;~ file-error?
		
		(port-error?
			(type type-predicate))
		
		;~ read-error?
		
		(write-error?
			(type type-predicate))
		
		
		
		
		(any-list?
			(type predicate))
		
		(empty-list?
			(type predicate)
			(alias null-list?))
		
		(any-or-empty-list?
			(type predicate))
		
		
		(proper-list?
			(type predicate))
		
		(proper-or-empty-list?
			(type predicate)
			(alias list?))
		
		
		(dotted-list?
			(type predicate))
		
		(dotted-or-empty-list?
			(type predicate))
		
		
		(circular-list?
			(type predicate))
		
		(circular-or-empty-list?
			(type predicate))
		
		
		;~ procedure?
		
		(syntax?
			(type predicate))
		
		
		;~ port?
		;~ input-port?
		;~ output-port?
		;~ binary-port?
		;~ textual-port?
		
		(binary-input-port?
			(type predicate))
		
		(textual-input-port?
			(type predicate))
		
		(binary-output-port?
			(type predicate))
		
		(textual-output-port?
			(type predicate))
		
		
		;~ eof-object?
		
		
		(path?
			(type type-predicate))
		
		(path-absolute?
			(type predicate))
		
		(path-relative?
			(type predicate))
		
		
		(process?
			(type type-predicate))
		
		
		(context?
			(type type-predicate))
		
		(binding?
			(type type-predicate))
		
		(parameters?
			(type type-predicate))
		
		(parameter?
			(type type-predicate))
		
		;~ promise?
		
		
		(resource?
			(type type-predicate))
		
		(internal?
			(type type-predicate))
		
		(opaque?
			(type predicate))
		
		
		;~ zero?
		;~ positive?
		;~ negative?
		;~ finite?
		;~ infinite?
		;~ nan?
		;~ even?
		;~ odd?
		
		
		;~ char-numeric?
		;~ char-alphabetic?
		;~ char-upper-case?
		;~ char-lower-case?
		
		(char-alphabetic-or-numeric?
			(type predicate))
		
		;~ char-whitespace?
		
		(char-control?
			(type predicate))
		
		(char-ascii?
			(type predicate))
		
		(char-ascii-numeric?
			(type predicate))
		
		(char-ascii-numeric-8?
			(type predicate))
		
		(char-ascii-numeric-16?
			(type predicate))
		
		(char-ascii-alphabetic?
			(type predicate))
		
		(char-ascii-upper-case?
			(type predicate))
		
		(char-ascii-lower-case?
			(type predicate))
		
		(char-ascii-alphabetic-or-numeric?
			(type predicate))
		
		(char-ascii-whitespace?
			(type predicate))
		
		(char-ascii-control?
			(type predicate))
		
		(char-ascii-punctuation?
			(type predicate))
		
		(char-ascii-graphic?
			(type predicate))
		
		
		(fs-metadata?
			(type type-predicate))
		
		(cache?
			(type type-predicate))
		
		
		
		
		;; ---- types (negated)
		
		
		(not-null?
			(type predicate))
		
		(not-void?
			(type predicate))
		
		(not-undefined?
			(type predicate))
		
		(not-boolean?
			(type predicate))
		
		(not-true?
			(type predicate))
		
		(not-false?
			(type predicate))
		
		(not-true-or-equivalent?
			(type predicate))
		
		(not-false-or-equivalent?
			(type predicate))
		
		(not-number?
			(type predicate))
		
		(not-complex?
			(type predicate))
		
		(not-real?
			(type predicate))
		
		(not-rational?
			(type predicate))
		
		(not-integer?
			(type predicate))
		
		(not-exact-integer?
			(type predicate))
		
		(not-exact?
			(type predicate))
		
		(not-inexact?
			(type predicate))
		
		(not-char?
			(type predicate))
		
		(not-symbol?
			(type predicate))
		
		(not-keyword?
			(type predicate))
		
		(not-unique?
			(type predicate))
		
		(not-string?
			(type predicate))
		
		(not-string-immutable?
			(type predicate))
		
		(not-string-mutable?
			(type predicate))
		
		(not-string-empty?
			(type predicate))
		
		(not-string-immutable-empty?
			(type predicate))
		
		(not-string-mutable-empty?
			(type predicate))
		
		(not-bytevector?
			(type predicate))
		
		(not-bytevector-immutable?
			(type predicate))
		
		(not-bytevector-mutable?
			(type predicate))
		
		(not-bytevector-empty?
			(type predicate))
		
		(not-bytevector-immutable-empty?
			(type predicate))
		
		(not-bytevector-mutable-empty?
			(type predicate))
		
		(not-string-regex?
			(type predicate))
		
		(not-bytevector-regex?
			(type predicate))
		
		(not-pair?
			(type predicate))
		
		(not-pair-mutable?
			(type predicate))
		
		(not-pair-immutable?
			(type predicate))
		
		(not-vector?
			(type predicate))
		
		(not-vector-mutable?
			(type predicate))
		
		(not-vector-immutable?
			(type predicate))
		
		(not-vector-empty?
			(type predicate))
		
		(not-vector-mutable-empty?
			(type predicate))
		
		(not-vector-immutable-empty?
			(type predicate))
		
		(not-values?
			(type predicate))
		
		(not-values-empty?
			(type predicate))
		
		(not-record-type?
			(type predicate))
		
		(not-record?
			(type predicate))
		
		(not-record-immutable?
			(type predicate))
		
		(not-record-mutable?
			(type predicate))
		
		(not-error-object?
			(type predicate))
		
		(not-syntax-error?
			(type predicate))
		
		(not-file-error?
			(type predicate))
		
		(not-port-error?
			(type predicate))
		
		(not-read-error?
			(type predicate))
		
		(not-write-error?
			(type predicate))
		
		(not-any-list?
			(type predicate))
		
		(not-empty-list?
			(type predicate))
		
		(not-any-or-empty-list?
			(type predicate))
		
		(not-proper-list?
			(type predicate))
		
		(not-proper-or-empty-list?
			(type predicate)
			(alias not-list?))
		
		(not-dotted-list?
			(type predicate))
		
		(not-dotted-list-or-emtpy?
			(type predicate))
		
		(not-circular-list?
			(type predicate))
		
		(not-circular-or-empty-list?
			(type predicate))
		
		(not-procedure?
			(type predicate))
		
		(not-syntax?
			(type predicate))
		
		(not-port?
			(type predicate))
		
		(not-input-port?
			(type predicate))
		
		(not-output-port?
			(type predicate))
		
		(not-binary-port?
			(type predicate))
		
		(not-textual-port?
			(type predicate))
		
		(not-binary-input-port?
			(type predicate))
		
		(not-textual-input-port?
			(type predicate))
		
		(not-binary-output-port?
			(type predicate))
		
		(not-textual-output-port?
			(type predicate))
		
		(not-eof-object?
			(type predicate))
		
		(not-path?
			(type predicate))
		
		(not-path-absolute?
			(type predicate))
		
		(not-path-relative?
			(type predicate))
		
		(not-process?
			(type predicate))
		
		(not-context?
			(type predicate))
		
		(not-binding?
			(type predicate))
		
		(not-parameters?
			(type predicate))
		
		(not-parameter?
			(type predicate))
		
		(not-promise?
			(type predicate))
		
		(not-resource?
			(type predicate))
		
		(not-internal?
			(type predicate))
		
		(not-opaque?
			(type predicate))
		
		(not-zero?
			(type predicate))
		
		(not-positive?
			(type predicate))
		
		(not-negative?
			(type predicate))
		
		(not-finite?
			(type predicate))
		
		(not-infinite?
			(type predicate))
		
		(not-nan?
			(type predicate))
		
		(not-even?
			(type predicate))
		
		(not-odd?
			(type predicate))
		
		(not-char-numeric?
			(type predicate))
		
		(not-char-alphabetic?
			(type predicate))
		
		(not-char-upper-case?
			(type predicate))
		
		(not-char-lower-case?
			(type predicate))
		
		(not-char-alphabetic-or-numeric?
			(type predicate))
		
		(not-char-whitespace?
			(type predicate))
		
		(not-char-control?
			(type predicate))
		
		(not-char-ascii?
			(type predicate))
		
		(not-char-ascii-numeric?
			(type predicate))
		
		(not-char-ascii-numeric-8?
			(type predicate))
		
		(not-char-ascii-numeric-16?
			(type predicate))
		
		(not-char-ascii-alphabetic?
			(type predicate))
		
		(not-char-ascii-upper-case?
			(type predicate))
		
		(not-char-ascii-lower-case?
			(type predicate))
		
		(not-char-ascii-alphabetic-or-numeric?
			(type predicate))
		
		(not-char-ascii-whitespace?
			(type predicate))
		
		(not-char-ascii-control?
			(type predicate))
		
		(not-char-ascii-punctuation?
			(type predicate))
		
		(not-char-ascii-graphic?
			(type predicate))
		
		(not-fs-metadata?
			(type predicate))
		
		(not-cache?
			(type predicate))
		
		
		
		
		;; ---- booleans
		
		
		(not*
			(type procedure))
		
		
		(and*
			(type procedure))
		
		(or*
			(type procedure)
			(alias ior*))
		
		(xor*
			(type procedure))
		
		
		(nand*
			(type procedure))
		
		(nor*
			(type procedure)
			(alias nior*))
		
		(nxor*
			(type procedure))
		
		
		
		
		;; ---- arithmetic
		
		
		(negative
			(type procedure))
		
		;~ abs
		
		(signum
			(type procedure))
		
		;~ floor
		;~ ceiling
		;~ round
		;~ truncate
		
		(fractional
			(type procedure))
		
		;~ exact
		;~ inexact
		;~ square
		;~ sqrt
		;~ exact-integer-sqrt
		;~ exp
		;~ log
		;~ sin
		;~ cos
		;~ tan
		;~ asin
		;~ acos
		;~ atan
		;~ floor/
		;~ floor-quotient
		;~ floor-remainder
		;~ modulo (alias)
		;~ truncate/
		;~ quotient (alias)
		;~ truncate-quotient
		;~ remainder (alias)
		;~ truncate-remainder
		;~ expt
		;~ +
		;~ -
		;~ *
		;~ /
		;~ gcd
		;~ lcm
		;~ min
		;~ max
		
		
		
		
		;; ---- bitwise
		
		
		(bitwise-not
			(type predicate))
		
		
		(bitwise-and
			(type predicate))
		
		(bitwise-or
			(type predicate)
			(alias bitwise-ior))
		
		(bitwise-xor
			(type predicate))
		
		
		(bitwise-nand
			(type predicate))
		
		(bitwise-nor
			(type predicate)
			(alias bitwise-nior))
		
		(bitwise-nxor
			(type predicate))
		
		
		(bitwise-shift-left
			(type predicate))
		
		(bitwise-shift-right
			(type predicate))
		
		(bitwise-rotate-left
			(type predicate))
		
		(bitwise-rotate-right
			(type predicate))
		
		
		
		
		;; ---- comparisons
		
		
		;~ eq?
		;~ equivalent-by-identity? (alias)
		;~ eqv?
		;~ equivalent-by-value-strict? (alias)
		;~ equal?
		;~ equivalent-by-value-strict-recursive? (alias)
		
		
		(equivalent-by-value-coerced?
			(type procedure))
		
		(equivalent-by-value-coerced-recursive?
			(type procedure))
		
		
		(generic<?
			(type procedure))
		
		(generic<=?
			(type procedure))
		
		(generic=?
			(type procedure))
		
		(generic>=?
			(type procedure))
		
		(generic>?
			(type procedure))
		
		
		(boolean<?
			(type procedure))
		
		(boolean<=?
			(type procedure))
		
		;~ boolean=?
		
		(boolean>=?
			(type procedure))
		
		(boolean>?
			(type procedure))
		
		
		;~ <
		;~ <=
		;~ =
		;~ >=
		;~ >
		
		;~ char<?
		;~ char<=?
		;~ char=?
		;~ char>=?
		;~ char>?
		
		;~ char-ci<?
		;~ char-ci<=?
		;~ char-ci=?
		;~ char-ci>=?
		;~ char-ci>?
		
		;~ string<?
		;~ string<=?
		;~ string=?
		;~ string>=?
		;~ string>?
		
		;~ string-ci<?
		;~ string-ci<=?
		;~ string-ci=?
		;~ string-ci>=?
		;~ string-ci>?
		
		
		(symbol<?
			(type procedure))
		
		(symbol<=?
			(type procedure))
		
		;~ symbol=?
		
		(symbol>=?
			(type procedure))
		
		(symbol>?
			(type procedure))
		
		
		(symbol-ci<?
			(type procedure))
		
		(symbol-ci<=?
			(type procedure))
		
		(symbol-ci=?
			(type procedure))
		
		(symbol-ci>=?
			(type procedure))
		
		(symbol-ci>?
			(type procedure))
		
		
		(keyword<?
			(type procedure))
		
		(keyword<=?
			(type procedure))
		
		(keyword=?
			(type procedure))
		
		(keyword>=?
			(type procedure))
		
		(keyword>?
			(type procedure))
		
		
		(keyword-ci<?
			(type procedure))
		
		(keyword-ci<=?
			(type procedure))
		
		(keyword-ci=?
			(type procedure))
		
		(keyword-ci>=?
			(type procedure))
		
		(keyword-ci>?
			(type procedure))
		
		
		(unique<?
			(type procedure))
		
		(unique<=?
			(type procedure))
		
		(unique=?
			(type procedure))
		
		(unique>=?
			(type procedure))
		
		(unique>?
			(type procedure))
		
		
		(bytevector<?
			(type procedure))
		
		(bytevector<=?
			(type procedure))
		
		(bytevector=?
			(type procedure))
		
		(bytevector>=?
			(type procedure))
		
		(bytevector>?
			(type procedure))
		
		
		(pair<?
			(type procedure))
		
		(pair<=?
			(type procedure))
		
		(pair=?
			(type procedure))
		
		(pair>=?
			(type procedure))
		
		(pair>?
			(type procedure))
		
		
		(vector<?
			(type procedure))
		
		(vector<=?
			(type procedure))
		
		(vector=?
			(type procedure))
		
		(vector>=?
			(type procedure))
		
		(vector>?
			(type procedure))
		
		
		(values<?
			(type procedure))
		
		(values<=?
			(type procedure))
		
		(values=?
			(type procedure))
		
		(values>=?
			(type procedure))
		
		(values>?
			(type procedure))
		
		
		(record<?
			(type procedure))
		
		(record<=?
			(type procedure))
		
		(record=?
			(type procedure))
		
		(record>=?
			(type procedure))
		
		(record>?
			(type procedure))
		
		
		(path<?
			(type procedure))
		
		(path<=?
			(type procedure))
		
		(path=?
			(type procedure))
		
		(path>=?
			(type procedure))
		
		(path>?
			(type procedure))
		
		
		
		
		;; ---- comparisons
		
		
		(not-equivalent-by-identity?
			(type procedure)
			(alias not-eq?))
		
		(not-equivalent-by-value-strict?
			(type procedure)
			(alias not-eqv?))
		
		(not-equivalent-by-value-strict-recursive?
			(type procedure)
			(alias not-equal?))
		
		
		(not-equivalent-by-value-coerced?
			(type procedure))
		
		(not-equivalent-by-value-coerced-recursive?
			(type procedure))
		
		
		(not-generic<?
			(type procedure))
		
		(not-generic<=?
			(type procedure))
		
		(not-generic=?
			(type procedure))
		
		(not-generic>=?
			(type procedure))
		
		(not-generic>?
			(type procedure))
		
		
		(not-boolean<?
			(type procedure))
		
		(not-boolean<=?
			(type procedure))
		
		(not-boolean=?
			(type procedure))
		
		(not-boolean>=?
			(type procedure))
		
		(not-boolean>?
			(type procedure))
		
		
		(not-<?
			(type procedure))
		
		(not-<=?
			(type procedure))
		
		(not-=?
			(type procedure))
		
		(not->=?
			(type procedure))
		
		(not->?
			(type procedure))
		
		
		(not-char<?
			(type procedure))
		
		(not-char<=?
			(type procedure))
		
		(not-char=?
			(type procedure))
		
		(not-char>=?
			(type procedure))
		
		(not-char>?
			(type procedure))
		
		
		(not-char-ci<?
			(type procedure))
		
		(not-char-ci<=?
			(type procedure))
		
		(not-char-ci=?
			(type procedure))
		
		(not-char-ci>=?
			(type procedure))
		
		(not-char-ci>?
			(type procedure))
		
		
		(not-string<?
			(type procedure))
		
		(not-string<=?
			(type procedure))
		
		(not-string=?
			(type procedure))
		
		(not-string>=?
			(type procedure))
		
		(not-string>?
			(type procedure))
		
		
		(not-string-ci<?
			(type procedure))
		
		(not-string-ci<=?
			(type procedure))
		
		(not-string-ci=?
			(type procedure))
		
		(not-string-ci>=?
			(type procedure))
		
		(not-string-ci>?
			(type procedure))
		
		
		(not-symbol<?
			(type procedure))
		
		(not-symbol<=?
			(type procedure))
		
		(not-symbol=?
			(type procedure))
		
		(not-symbol>=?
			(type procedure))
		
		(not-symbol>?
			(type procedure))
		
		
		(not-symbol-ci<?
			(type procedure))
		
		(not-symbol-ci<=?
			(type procedure))
		
		(not-symbol-ci=?
			(type procedure))
		
		(not-symbol-ci>=?
			(type procedure))
		
		(not-symbol-ci>?
			(type procedure))
		
		
		(not-keyword<?
			(type procedure))
		
		(not-keyword<=?
			(type procedure))
		
		(not-keyword=?
			(type procedure))
		
		(not-keyword>=?
			(type procedure))
		
		(not-keyword>?
			(type procedure))
		
		
		(not-keyword-ci<?
			(type procedure))
		
		(not-keyword-ci<=?
			(type procedure))
		
		(not-keyword-ci=?
			(type procedure))
		
		(not-keyword-ci>=?
			(type procedure))
		
		(not-keyword-ci>?
			(type procedure))
		
		
		(not-unique<?
			(type procedure))
		
		(not-unique<=?
			(type procedure))
		
		(not-unique=?
			(type procedure))
		
		(not-unique>=?
			(type procedure))
		
		(not-unique>?
			(type procedure))
		
		
		(not-bytevector<?
			(type procedure))
		
		(not-bytevector<=?
			(type procedure))
		
		(not-bytevector=?
			(type procedure))
		
		(not-bytevector>=?
			(type procedure))
		
		(not-bytevector>?
			(type procedure))
		
		
		(not-pair<?
			(type procedure))
		
		(not-pair<=?
			(type procedure))
		
		(not-pair=?
			(type procedure))
		
		(not-pair>=?
			(type procedure))
		
		(not-pair>?
			(type procedure))
		
		
		(not-vector<?
			(type procedure))
		
		(not-vector<=?
			(type procedure))
		
		(not-vector=?
			(type procedure))
		
		(not-vector>=?
			(type procedure))
		
		(not-vector>?
			(type procedure))
		
		
		(not-values<?
			(type procedure))
		
		(not-values<=?
			(type procedure))
		
		(not-values=?
			(type procedure))
		
		(not-values>=?
			(type procedure))
		
		(not-values>?
			(type procedure))
		
		
		(not-record<?
			(type procedure))
		
		(not-record<=?
			(type procedure))
		
		(not-record=?
			(type procedure))
		
		(not-record>=?
			(type procedure))
		
		(not-record>?
			(type procedure))
		
		
		(not-path<?
			(type procedure))
		
		(not-path<=?
			(type procedure))
		
		(not-path=?
			(type procedure))
		
		(not-path>=?
			(type procedure))
		
		(not-path>?
			(type procedure))
		
		
		
		
		;; ---- lists
		
		
		;~ car
		;~ cdr
		;~ caar
		;~ cdar
		
		
		(first-pair
			(type procedure))
		
		(second-pair
			(type procedure))
		
		(third-pair
			(type procedure))
		
		(fourth-pair
			(type procedure))
		
		(fifth-pair
			(type procedure))
		
		(sixth-pair
			(type procedure))
		
		(seventh-pair
			(type procedure))
		
		(eighth-pair
			(type procedure))
		
		(ninth-pair
			(type procedure))
		
		(tenth-pair
			(type procedure))
		
		
		(first
			(type procedure))
		
		(second
			(type procedure)
			(alias cadr))
		
		(third
			(type procedure))
		
		(fourth
			(type procedure))
		
		(fifth
			(type procedure))
		
		(sixth
			(type procedure))
		
		(seventh
			(type procedure))
		
		(eighth
			(type procedure))
		
		(ninth
			(type procedure))
		
		(tenth
			(type procedure))
		
		
		(first-tail
			(type procedure))
		
		(second-tail
			(type procedure)
			(alias cddr))
		
		(third-tail
			(type procedure))
		
		(fourth-tail
			(type procedure))
		
		(fifth-tail
			(type procedure))
		
		(sixth-tail
			(type procedure))
		
		(seventh-tail
			(type procedure))
		
		(eighth-tail
			(type procedure))
		
		(ninth-tail
			(type procedure))
		
		(tenth-tail
			(type procedure))
		
		
		;~ length
		;~ reverse
		
		(pair->immutable
			(type procedure))
		
		(pair->mutable
			(type procedure))
		
		(list->immutable
			(type procedure))
		
		(list->mutable
			(type procedure))
		
		
		;~ cons
		
		(xcons
			(type procedure))
		
		;~ set-car!
		;~ set-cdr!
		
		
		(list-ref-cons
			(type procedure)
			(alias list-tail))
		
		(list-ref-car
			(type procedure)
			(alias list-ref))
		
		(list-ref-cdr
			(type procedure))
		
		
		;~ memq
		;~ memv
		;~ assq
		;~ assv
		
		(find
			(type procedure))
		
		(list-set-car!
			(type procedure)
			(alias list-set!))
		
		(list-set-cdr!
			(type procedure))
		
		
		(make-pair
			(type procedure))
		
		;~ make-list
		;~ list
		
		(list*
			(type procedure))
		
		;~ append
		
		(list-fill!
			(type procedure))
		
		(list-copy!
			(type procedure))
		
		;~ list-copy
		
		(list-reverse!
			(type procedure))
		
		;~ member
		;~ assoc
		
		
		
		
		;; ---- arrays
		
		
		;~ vector-length
		
		(vector-reverse
			(type procedure))
		
		(vector->immutable
			(type procedure))
		
		(vector->mutable
			(type procedure))
		
		(vector-clear!
			(type procedure))
		
		;~ vector-ref
		
		(vector-push-from!
			(type procedure))
		
		(vector-find
			(type procedure))
		
		;~ vector-set!
		
		(vector-insert-from!
			(type procedure))
		
		(vector-swap!
			(type procedure))
		
		(vector-clear-at!
			(type procedure))
		
		;~ make-vector
		;~ vector
		;~ vector-append
		;~ vector-fill!
		;~ vector-copy!
		
		(vector-append!
			(type procedure))
		
		;~ vector-copy
		
		(vector-reverse!
			(type procedure))
		
		;~ vector->list
		;~ list->vector
		
		(vector-push!
			(type procedure))
		
		(vector-pop!
			(type procedure))
		
		(vector-insert!
			(type procedure))
		
		(vector-remove!
			(type procedure))
		
		(vector-resize!
			(type procedure))
		
		(vector-resize-at!
			(type procedure))
		
		
		
		
		;; ---- bytes
		
		
		;~ bytevector-length
		
		(bytevector-reverse
			(type procedure))
		
		(bytevector->immutable
			(type procedure))
		
		(bytevector->mutable
			(type procedure))
		
		;~ bytevector-u8-ref
		
		;~ bytevector-u8-set!
		;~ make-bytevector
		;~ bytevector
		;~ bytevector-append
		
		(bytevector-u8-fill!
			(type procedure))
		
		;~ bytevector-copy!
		;~ bytevector-copy
		
		(bytevector-reverse!
			(type procedure))
		
		(bytevector->list
			(type procedure))
		
		(list->bytevector
			(type procedure))
		
		(bytevector->vector
			(type procedure))
		
		(vector->bytevector
			(type procedure))
		
		
		
		
		;; ---- bytes regular-expressions
		
		
		(make-bytevector-regex
			(type procedure))
		
		(bytevector-regex-match?
			(type procedure))
		
		(bytevector-regex-match
			(type procedure))
		
		(bytevector-regex-match-all
			(type procedure))
		
		(bytevector-regex-match-all->vector
			(type procedure))
		
		(bytevector-regex-match-position
			(type procedure))
		
		(bytevector-regex-match-position-all
			(type procedure))
		
		(bytevector-regex-match-position-all->vector
			(type procedure))
		
		(bytevector-regex-match-captures
			(type procedure))
		
		(bytevector-regex-match-captures->assoc
			(type procedure))
		
		(bytevector-regex-match-captures->vector
			(type procedure))
		
		(bytevector-regex-match-captures-all
			(type procedure))
		
		(bytevector-regex-match-captures-all->assoc
			(type procedure))
		
		(bytevector-regex-match-captures-all->vector
			(type procedure))
		
		(bytevector-regex-match-captures-position
			(type procedure))
		
		(bytevector-regex-match-captures-position->assoc
			(type procedure))
		
		(bytevector-regex-match-captures-position->vector
			(type procedure))
		
		(bytevector-regex-match-captures-position-all
			(type procedure))
		
		(bytevector-regex-match-captures-position-all->assoc
			(type procedure))
		
		(bytevector-regex-match-captures-position-all->vector
			(type procedure))
		
		
		
		
		;; ---- string
		
		
		;~ string-length
		
		(string-reverse
			(type procedure))
		
		;~ string->symbol
		;~ symbol->string
		;~ char->integer
		;~ integer->char
		
		(string->keyword
			(type procedure))
		
		(keyword->string
			(type procedure))
		
		(symbol->keyword
			(type procedure))
		
		(keyword->symbol
			(type procedure))
		
		;~ string-upcase
		;~ string-downcase
		;~ string-foldcase
		
		(symbol-upcase
			(type procedure))
		
		(symbol-downcase
			(type procedure))
		
		(symbol-foldcase
			(type procedure))
		
		;~ char-upcase
		;~ char-downcase
		;~ char-foldcase
		
		(keyword-upcase
			(type procedure))
		
		(keyword-downcase
			(type procedure))
		
		(keyword-foldcase
			(type procedure))
		
		(string->immutable
			(type procedure))
		
		(string->mutable
			(type procedure))
		
		;~ string-ref
		
		(string-set!
			(type procedure))
		
		(make-string
			(type procedure))
		
		(string
			(type procedure))
		
		(string-append
			(type procedure))
		
		(string-fill!
			(type procedure))
		
		(string-copy!
			(type procedure))
		
		;~ string-copy
		;~ substring (alias)
		
		(string-reverse!
			(type procedure))
		
		;~ string->list
		;~ list->string
		;~ string->vector
		;~ vector->string
		;~ string->utf8
		;~ utf8->string
		;~ string->number
		;~ number->string
		;~ digit-value
		
		
		
		
		;; ---- string regular-expressions
		
		
		(make-string-regex
			(type procedure))
		
		(string-regex-match?
			(type procedure))
		
		(string-regex-match
			(type procedure))
		
		(string-regex-match-all
			(type procedure))
		
		(string-regex-match-all->vector
			(type procedure))
		
		(string-regex-match-position
			(type procedure))
		
		(string-regex-match-position-all
			(type procedure))
		
		(string-regex-match-position-all->vector
			(type procedure))
		
		(string-regex-match-captures
			(type procedure))
		
		(string-regex-match-captures->assoc
			(type procedure))
		
		(string-regex-match-captures->vector
			(type procedure))
		
		(string-regex-match-captures-all
			(type procedure))
		
		(string-regex-match-captures-all->assoc
			(type procedure))
		
		(string-regex-match-captures-all->vector
			(type procedure))
		
		(string-regex-match-captures-position
			(type procedure))
		
		(string-regex-match-captures-position->assoc
			(type procedure))
		
		(string-regex-match-captures-position->vector
			(type procedure))
		
		(string-regex-match-captures-position-all
			(type procedure))
		
		(string-regex-match-captures-position-all->assoc
			(type procedure))
		
		(string-regex-match-captures-position-all->vector
			(type procedure))
		
		
		
		
		;; ---- functional
		
		
		(identity
			(type procedure))
		
		(constant-fn
			(type procedure))
		
		(constant-fn*
			(type procedure))
		
		(not-fn
			(type procedure))
		
		(call-with-list*
			(type procedure))
		
		(call-with-list
			(type procedure))
		
		(call-with-vector*
			(type procedure))
		
		(call-with-vector
			(type procedure))
		
		(call-with-values*
			(type procedure))
		
		;~ call-with-values
		
		(map-in-order
			(type procedure))
		
		;~ call
		;~ apply
		;~ map
		;~ for-each
		;~ vector-map
		;~ vector-for-each
		;~ bytevector-u8-map
		;~ bytevector-u8-for-each
		;~ string-map
		;~ string-for-each
		;~ values
		
		(curry
			(type procedure))
		
		(curry-last
			(type procedure))
		
		(compose
			(type procedure))
		
		(compose*
			(type procedure))
		
		
		
		
		;; ---- records
		
		
		(record-type-identifier
			(type procedure))
		
		(record-type-size
			(type procedure))
		
		(record-type
			(type procedure))
		
		(record->immutable
			(type procedure))
		
		(record->mutable
			(type procedure))
		
		(make-record-type
			(type procedure))
		
		(record-type-predicate
			(type procedure))
		
		(record-type-accessor
			(type procedure))
		
		(record-type-mutator
			(type procedure))
		
		(record-type-constructor
			(type procedure))
		
		(record-type-constructor*
			(type procedure))
		
		(record-of?
			(type procedure))
		
		(record-set!
			(type procedure))
		
		(record-ref
			(type procedure))
		
		(make-record
			(type procedure))
		
		(make-record*
			(type procedure))
		
		(record->vector
			(type procedure))
		
		(vector->record
			(type procedure))
		
		(record->values
			(type procedure))
		
		(values->record
			(type procedure))
		
		(record->list
			(type procedure))
		
		(list->record
			(type procedure))
		
		
		
		
		;; ---- runtime
		
		
		;~ command-line
		
		(command-line->vector
			(type procedure))
		
		(command-line-length
			(type procedure))
		
		;~ get-environment-variables
		
		(get-environment-variables->vector
			(type procedure))
		
		(get-environment-fingerprint
			(type procedure))
		
		;~ current-second
		;~ current-jiffy
		;~ jiffies-per-second
		;~ raise
		;~ error-object-message
		;~ error-object-irritants
		
		(error-object-irritants->vector
			(type procedure))
		
		(error-object-irritants->values
			(type procedure))
		
		(command-line-ref
			(type procedure))
		
		;~ get-environment-variable
		
		(process-wait-poll
			(type procedure))
		
		(process-wait-try
			(type procedure))
		
		(process-wait
			(type procedure))
		
		(process-stdin
			(type procedure))
		
		(process-stdout
			(type procedure))
		
		(process-stderr
			(type procedure))
		
		(cache-close
			(type procedure))
		
		(serialize-bytevector
			(type procedure))
		
		(deserialize-bytevector
			(type procedure))
		
		(hash
			(type procedure))
		
		(hash-sip-unseeded
			(type procedure))
		
		(hash-sea-unseeded
			(type procedure))
		
		(hash-blake2b-unseeded
			(type procedure))
		
		(hash-blake2s-unseeded
			(type procedure))
		
		(process-spawn
			(type procedure))
		
		(process-exec
			(type procedure))
		
		(process-run-try
			(type procedure))
		
		(process-run
			(type procedure))
		
		;~ error
		
		(make-error
			(type procedure))
		
		;~ make-parameter
		
		(parameter-ref
			(type procedure))
		
		(parameter-set!
			(type procedure))
		
		(trace-critical
			(type procedure))
		
		(trace-error
			(type procedure))
		
		(trace-warning
			(type procedure))
		
		(trace-notice
			(type procedure))
		
		(trace-information
			(type procedure))
		
		(trace-internal
			(type procedure))
		
		(trace-debugging
			(type procedure))
		
		(abort
			(type procedure))
		
		(pause
			(type procedure))
		
		;~ exit
		;~ emergency-exit
		
		(process-spawn*
			(type procedure))
		
		(process-exec*
			(type procedure))
		
		(cache-open
			(type procedure))
		
		(cache-select-bytevector
			(type procedure))
		
		(cache-include-bytevector
			(type procedure))
		
		(cache-exclude-bytevector
			(type procedure))
		
		(cache-resolve-bytevector
			(type procedure))
		
		(cache-select
			(type procedure))
		
		(cache-include
			(type procedure))
		
		(cache-exclude
			(type procedure))
		
		(cache-resolve
			(type procedure))
		
		(cache-exclude-all
			(type procedure))
		
		(cache-prune-all
			(type procedure))
		
		(hash-sip-seeded
			(type procedure))
		
		(hash-sea-seeded
			(type procedure))
		
		(hash-blake2b-seeded
			(type procedure))
		
		(hash-blake2s-seeded
			(type procedure))
		
		
		
		
		;; ---- ports
		
		
		;~ current-input-port
		;~ current-output-port
		;~ current-error-port
		;~ eof-object
		;~ open-input-bytevector
		;~ open-input-string
		;~ get-output-bytevector
		;~ get-output-string
		;~ open-binary-input-file
		;~ open-binary-output-file
		;~ open-input-file
		;~ open-output-file
		
		(port-descriptor
			(type procedure))
		
		(port-descriptor-clone
			(type procedure))
		
		(port-descriptor-ref
			(type procedure))
		
		(port-descriptor-path
			(type procedure))
		
		(port-temporary-release
			(type procedure))
		
		(port-temporary-path
			(type procedure))
		
		;~ call-with-port
		
		(call-with-binary-input-file
			(type procedure))
		
		(call-with-binary-output-file
			(type procedure))
		
		;~ call-with-input-file
		;~ call-with-output-file
		
		(with-binary-input-file
			(type procedure))
		
		(with-binary-output-file
			(type procedure))
		
		(with-input-from-file
			(type procedure))
		
		(with-output-to-file
			(type procedure))
		
		(port-descriptor-flag-ref
			(type procedure))
		
		(port-descriptor-flag-set!
			(type procedure))
		
		;~ open-output-bytevector
		;~ open-output-string
		;~ input-port-open?
		;~ output-port-open?
		;~ close-port
		;~ close-input-port
		;~ close-output-port
		;~ u8-ready?
		;~ peek-u8
		;~ read-u8
		;~ char-ready?
		;~ peek-char
		;~ read-char
		;~ read-bytevector!
		
		(read-bytevector-append!
			(type procedure))
		
		;~ read-bytevector
		
		(read-bytevector-chunk
			(type procedure))
		
		(read-bytevector-line
			(type procedure))
		
		(read-bytevector-zero
			(type procedure))
		
		(read-string-append!
			(type procedure))
		
		;~ read-string
		
		(read-string-chunk
			(type procedure))
		
		(read-string-line
			(type procedure)
			(alias read-line))
		
		(read-string-zero
			(type procedure))
		
		;~ read
		
		(read-bytevector-fold
			(type procedure))
		
		(read-bytevector-chunk-fold
			(type procedure))
		
		(read-bytevector-line-fold
			(type procedure))
		
		(read-bytevector-zero-fold
			(type procedure))
		
		(read-string-fold
			(type procedure))
		
		(read-string-chunk-fold
			(type procedure))
		
		(read-string-line-fold
			(type procedure))
		
		(read-string-zero-fold
			(type procedure))
		
		(read-fold
			(type procedure))
		
		;~ write-u8
		;~ write-bytevector
		
		(write-bytevector-line
			(type procedure))
		
		(write-bytevector-zero
			(type procedure))
		
		;~ write-char
		;~ write-string
		
		(write-string-line
			(type procedure))
		
		(write-string-zero
			(type procedure))
		
		;~ write
		;~ write-shared
		;~ write-simple
		;~ display
		
		(write-line
			(type procedure))
		
		(display-line
			(type procedure))
		
		;~ newline
		;~ flush-output-port
		
		(open-binary-temporary
			(type procedure))
		
		(open-temporary
			(type procedure))
		
		
		
		
		;; ---- crypto
		
		
		(crypto-bytevector
			(type procedure))
		
		(crypto-bytevector-append!
			(type procedure))
		
		(crypto-bytevector-fill!
			(type procedure))
		
		(crypto-blake2b-128
			(type procedure))
		
		(crypto-blake2b-192
			(type procedure))
		
		(crypto-blake2b-224
			(type procedure))
		
		(crypto-blake2b-256
			(type procedure))
		
		(crypto-blake2b-320
			(type procedure))
		
		(crypto-blake2b-384
			(type procedure))
		
		(crypto-blake2b-448
			(type procedure))
		
		(crypto-blake2b-512
			(type procedure))
		
		(crypto-blake2b-64
			(type procedure))
		
		(crypto-blake2s-128
			(type procedure))
		
		(crypto-blake2s-192
			(type procedure))
		
		(crypto-blake2s-224
			(type procedure))
		
		(crypto-blake2s-256
			(type procedure))
		
		(crypto-blake2s-64
			(type procedure))
		
		(crypto-md5
			(type procedure))
		
		(crypto-sha1
			(type procedure))
		
		(crypto-sha2-256
			(type procedure))
		
		(crypto-sha2-256-224
			(type procedure)
			(alias crypto-sha2-224))
		
		(crypto-sha2-512
			(type procedure))
		
		(crypto-sha2-512-224
			(type procedure))
		
		(crypto-sha2-512-256
			(type procedure))
		
		(crypto-sha2-512-384
			(type procedure)
			(alias crypto-sha2-384))
		
		(crypto-sha3-224
			(type procedure))
		
		(crypto-sha3-256
			(type procedure))
		
		(crypto-sha3-384
			(type procedure))
		
		(crypto-sha3-512
			(type procedure))
		
		
		
		
		;; ---- encoding
		
		
		(base32-decode
			(type procedure))
		
		(base32-decode-append!
			(type procedure))
		
		(base32-decode-fill!
			(type procedure))
		
		(base32-hex-decode
			(type procedure))
		
		(base32-hex-decode-append!
			(type procedure))
		
		(base32-hex-decode-fill!
			(type procedure))
		
		(base32-hex-nopad-decode
			(type procedure))
		
		(base32-hex-nopad-decode-append!
			(type procedure))
		
		(base32-hex-nopad-decode-fill!
			(type procedure))
		
		(base32-nopad-decode
			(type procedure))
		
		(base32-nopad-decode-append!
			(type procedure))
		
		(base32-nopad-decode-fill!
			(type procedure))
		
		(base64-decode
			(type procedure))
		
		(base64-decode-append!
			(type procedure))
		
		(base64-decode-fill!
			(type procedure))
		
		(base64-mime-decode
			(type procedure))
		
		(base64-mime-decode-append!
			(type procedure))
		
		(base64-mime-decode-fill!
			(type procedure))
		
		(base64-nopad-decode
			(type procedure))
		
		(base64-nopad-decode-append!
			(type procedure))
		
		(base64-nopad-decode-fill!
			(type procedure))
		
		(base64-url-decode
			(type procedure))
		
		(base64-url-decode-append!
			(type procedure))
		
		(base64-url-decode-fill!
			(type procedure))
		
		(base64-url-nopad-decode
			(type procedure))
		
		(base64-url-nopad-decode-append!
			(type procedure))
		
		(base64-url-nopad-decode-fill!
			(type procedure))
		
		(hex-decode
			(type procedure))
		
		(hex-decode-append!
			(type procedure))
		
		(hex-decode-fill!
			(type procedure))
		
		(base32-encode
			(type procedure))
		
		(base32-encode-append!
			(type procedure))
		
		(base32-encode-fill!
			(type procedure))
		
		(base32-hex-encode
			(type procedure))
		
		(base32-hex-encode-append!
			(type procedure))
		
		(base32-hex-encode-fill!
			(type procedure))
		
		(base32-hex-nopad-encode
			(type procedure))
		
		(base32-hex-nopad-encode-append!
			(type procedure))
		
		(base32-hex-nopad-encode-fill!
			(type procedure))
		
		(base32-nopad-encode
			(type procedure))
		
		(base32-nopad-encode-append!
			(type procedure))
		
		(base32-nopad-encode-fill!
			(type procedure))
		
		(base64-encode
			(type procedure))
		
		(base64-encode-append!
			(type procedure))
		
		(base64-encode-fill!
			(type procedure))
		
		(base64-mime-encode
			(type procedure))
		
		(base64-mime-encode-append!
			(type procedure))
		
		(base64-mime-encode-fill!
			(type procedure))
		
		(base64-nopad-encode
			(type procedure))
		
		(base64-nopad-encode-append!
			(type procedure))
		
		(base64-nopad-encode-fill!
			(type procedure))
		
		(base64-url-encode
			(type procedure))
		
		(base64-url-encode-append!
			(type procedure))
		
		(base64-url-encode-fill!
			(type procedure))
		
		(base64-url-nopad-encode
			(type procedure))
		
		(base64-url-nopad-encode-append!
			(type procedure))
		
		(base64-url-nopad-encode-fill!
			(type procedure))
		
		(hex-lower-encode
			(type procedure)
			(alias hex-encode))
		
		(hex-lower-encode-append!
			(type procedure)
			(alias hex-encode-append!))
		
		(hex-lower-encode-fill!
			(type procedure)
			(alias hex-encode-fill!))
		
		(hex-upper-encode
			(type procedure))
		
		(hex-upper-encode-append!
			(type procedure))
		
		(hex-upper-encode-fill!
			(type procedure))
		
		
		
		
		;; ---- random
		
		
		(random-boolean
			(type procedure))
		
		(random-boolean-weighted
			(type procedure))
		
		(random-bytevector
			(type procedure))
		
		(random-bytevector-append!
			(type procedure))
		
		(random-bytevector-fill!
			(type procedure))
		
		(random-bytevector-permutation
			(type procedure))
		
		(random-bytevector-shuffle!
			(type procedure))
		
		(random-char
			(type procedure))
		
		(random-char*
			(type procedure))
		
		(random-char-ascii
			(type procedure))
		
		(random-char-ascii-alphabetic
			(type procedure))
		
		(random-char-ascii-lower-case
			(type procedure))
		
		(random-char-ascii-upper-case
			(type procedure))
		
		(random-char-ascii-numeric
			(type procedure))
		
		(random-char-ascii-numeric-8
			(type procedure))
		
		(random-char-ascii-numeric-16
			(type procedure))
		
		(random-char-ascii-alphabetic-or-numeric
			(type procedure))
		
		(random-char-ascii-control
			(type procedure))
		
		(random-char-ascii-graphic
			(type procedure))
		
		(random-char-ascii-punctuation
			(type procedure))
		
		(random-char-ascii-whitespace
			(type procedure))
		
		(random-string-ascii
			(type procedure))
		
		(random-string-ascii-alphabetic
			(type procedure))
		
		(random-string-ascii-lower-case
			(type procedure))
		
		(random-string-ascii-upper-case
			(type procedure))
		
		(random-string-ascii-numeric
			(type procedure))
		
		(random-string-ascii-numeric-8
			(type procedure))
		
		(random-string-ascii-numeric-16
			(type procedure))
		
		(random-string-ascii-alphabetic-or-numeric
			(type procedure))
		
		(random-string-ascii-control
			(type procedure))
		
		(random-string-ascii-graphic
			(type procedure))
		
		(random-string-ascii-punctuation
			(type procedure))
		
		(random-string-ascii-whitespace
			(type procedure))
		
		(random-string-ascii-append!
			(type procedure))
		
		(random-string-ascii-alphabetic-append!
			(type procedure))
		
		(random-string-ascii-lower-case-append!
			(type procedure))
		
		(random-string-ascii-upper-case-append!
			(type procedure))
		
		(random-string-ascii-numeric-append!
			(type procedure))
		
		(random-string-ascii-numeric-8-append!
			(type procedure))
		
		(random-string-ascii-numeric-16-append!
			(type procedure))
		
		(random-string-ascii-alphabetic-or-numeric-append!
			(type procedure))
		
		(random-string-ascii-control-append!
			(type procedure))
		
		(random-string-ascii-graphic-append!
			(type procedure))
		
		(random-string-ascii-punctuation-append!
			(type procedure))
		
		(random-string-ascii-whitespace-append!
			(type procedure))
		
		(random-string-ascii-permutation
			(type procedure))
		
		(random-string-ascii-alphabetic-permutation
			(type procedure))
		
		(random-string-ascii-lower-case-permutation
			(type procedure))
		
		(random-string-ascii-upper-case-permutation
			(type procedure))
		
		(random-string-ascii-numeric-permutation
			(type procedure))
		
		(random-string-ascii-numeric-8-permutation
			(type procedure))
		
		(random-string-ascii-numeric-16-permutation
			(type procedure))
		
		(random-string-ascii-alphabetic-or-numeric-permutation
			(type procedure))
		
		(random-string-ascii-control-permutation
			(type procedure))
		
		(random-string-ascii-graphic-permutation
			(type procedure))
		
		(random-string-ascii-punctuation-permutation
			(type procedure))
		
		(random-string-ascii-whitespace-permutation
			(type procedure))
		
		(random-f64
			(type procedure))
		
		(random-f64*
			(type procedure))
		
		(random-i8
			(type procedure))
		
		(random-i16
			(type procedure))
		
		(random-i32
			(type procedure))
		
		(random-i64
			(type procedure))
		
		(random-i64*
			(type procedure))
		
		(random-u1
			(type procedure))
		
		(random-u2
			(type procedure))
		
		(random-u3
			(type procedure))
		
		(random-u4
			(type procedure))
		
		(random-u5
			(type procedure))
		
		(random-u6
			(type procedure))
		
		(random-u63
			(type procedure))
		
		(random-u7
			(type procedure))
		
		(random-u8
			(type procedure))
		
		(random-u15
			(type procedure))
		
		(random-u16
			(type procedure))
		
		(random-u31
			(type procedure))
		
		(random-u32
			(type procedure))
		
		
		
		
		;; ---- extended
		
		
		(caaar
			(type procedure))
		
		(caaaar
			(type procedure))
		
		(caaadr
			(type procedure))
		
		(caadr
			(type procedure))
		
		(caadar
			(type procedure))
		
		(caaddr
			(type procedure))
		
		(cadar
			(type procedure))
		
		(cadaar
			(type procedure))
		
		(cadadr
			(type procedure))
		
		(caddr
			(type procedure))
		
		(caddar
			(type procedure))
		
		(cadddr
			(type procedure))
		
		(cdaar
			(type procedure))
		
		(cdaaar
			(type procedure))
		
		(cdaadr
			(type procedure))
		
		(cdadr
			(type procedure))
		
		(cdadar
			(type procedure))
		
		(cdaddr
			(type procedure))
		
		(cddar
			(type procedure))
		
		(cddaar
			(type procedure))
		
		(cddadr
			(type procedure))
		
		(cdddr
			(type procedure))
		
		(cdddar
			(type procedure))
		
		(cddddr
			(type procedure))
		
		
		
		
		;; ---- parameters
		
		
		(process-spawn:stdin
			(type parameter))
		
		(process-spawn:stdout
			(type parameter))
		
		(process-spawn:stderr
			(type parameter))
		
		(process-spawn:directory
			(type parameter))
		
		(process-spawn:env-empty
			(type parameter))
		
		(process-spawn:env-include
			(type parameter))
		
		(process-spawn:env-exclude
			(type parameter))
		
		
		
		
		;; ---- unimplemented syntaxes
		
		
		;~ delay
		;~ delay-force
		
		
		
		
		;; ---- unsupported syntaxes
		
		
		;~ case-lambda
		;~ cond-expand
		;~ define-syntax
		;~ import
		;~ include
		;~ include-ci
		;~ let-syntax
		;~ letrec-syntax
		;~ syntax-error
		;~ syntax-rules
		
		
		
		
		;; ---- unimplemented procedures
		
		
		;~ make-promise
		;~ force
		
		;~ eval
		;~ environment
		;~ null-environment
		;~ interaction-environment
		;~ scheme-report-environment
		
		;~ load
		
		
		;! +fn
		;! -fn
		;! -fn-last
		;! *fn
		;! /fn
		;! /fn-last
		;! <=fn
		;! <fn
		;! =fn
		;! >=fn
		;! >fn
		
		;! begin-fn
		
		;! or-fn
		;! and-fn
		;! xor-fn
		
		;! nand-fn
		;! nor-fn
		;! nxor-fn
		
		;! if-fn
		;! when-fn
		;! unless-fn
		;! cond-fn
		;! case-fn
		
		;! do-fn
		;! while-fn
		;! until-fn
		
		;! eq-fn
		;! equal-fn
		;! eqv-fn
		
		;! string-for-each-fn
		;! string-map-fn
		
		;! bytevector-u8-for-each-fn
		;! bytevector-u8-map-fn
		;! bytevector-u8-ref-fn
		;! bytevector-u8-set-fn
		
		;! vector-for-each-fn
		;! vector-map-fn
		;! vector-ref-fn
		;! vector-set-fn
		
		;! cons-car-fn
		;! cons-cdr-fn
		
		;! map-fn
		;! for-each-fn
		
		;! member-fn
		;! assoc-fn
		
		
		;! alist-cons
		;! alist-copy
		;! alist-delete
		;! any
		;! append-map
		;! append-reverse
		;! break
		;! circular-list
		;! concatenate
		;! cons*
		;! count
		;! delete
		;! delete-duplicates
		;! drop
		;! drop-right
		;! drop-while
		;! every
		;! filter-map
		;! find-pair
		;! find-tail
		;! fold
		;! fold-right
		;! iota
		;! last
		;! last-pair
		;! list-index
		;! list-tabulate
		;! list<
		;! list<=
		;! list=
		;! list>
		;! list>=
		;! pair-fold
		;! pair-fold-right
		;! pair-for-each
		;! partition
		;! reduce
		;! reduce-right
		;! remove
		;! span
		;! split-at
		;! take
		;! take-right
		;! take-while
		;! unfold
		;! unfold-right
		;! unzip*
		;! unzip1
		;! unzip2
		;! unzip3
		;! unzip4
		;! unzip5
		;! zip
		
		
		
		
		;; ---- unsupported procedures
		
		
		;~ features
		
		;~ rationalize
		;~ numerator
		;~ denominator
		
		;~ make-rectangular
		;~ real-part
		;~ imag-part
		
		;~ make-polar
		;~ angle
		;~ magnitude
		
		;~ call-with-current-continuation
		;~ call/cc (alias)
		;~ dynamic-wind
		;~ with-exception-handler
		;~ raise-continuable
		
		
		;! car+cdr
		;! length+
		
		;! alist-delete!
		;! append!
		;! append-map!
		;! append-reverse!
		;! break!
		;! concatenate!
		;! delete!
		;! delete-duplicates!
		;! drop-right!
		;! drop-while!
		;! filter!
		;! map!
		;! partition!
		;! remove!
		;! reverse!
		;! span!
		;! split-at!
		;! take!
		;! take-right!
		;! take-while!
		
		;! lset-adjoinlset-diff+intersection
		;! lset-diff+intersection!
		;! lset-difference
		;! lset-difference!
		;! lset-intersection
		;! lset-intersection!
		;! lset-union
		;! lset-union!
		;! lset-xor
		;! lset-xor!
		;! lset<
		;! lset<=
		;! lset=
		;! lset>
		;! lset>=
		
		
		
		
	)
	
	
	
	
)

