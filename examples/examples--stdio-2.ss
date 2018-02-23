
(define s-in (current-input-port))
(define s-out (current-output-port))


(do
		((chunk (read-string-chunk s-in) (read-string-chunk s-in)))
		((eof-object? chunk))
	; (newline) (display (string-length chunk)) (newline)
	(write-string chunk s-out))

