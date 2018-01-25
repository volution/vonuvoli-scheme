
(define s-in (current-input-port))
(define s-out (current-output-port))


(do
		((line (read-line s-in) (read-line s-in)))
		((eof-object? line))
	; (newline) (display (string-length chunk)) (newline)
	(write-string line s-out)
	(newline))

