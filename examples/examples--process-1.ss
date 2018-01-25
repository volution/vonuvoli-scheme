

(define p
	(process-spawn*
		"base64"
		'("--" "/etc/services")
		'(
			(stdout . piped)
		)
	))

(define p-out (process-stdout p))
(define s-out (current-output-port))


(do
		((line (read-line p-out) (read-line p-out)))
		((eof-object? line))
	(write-string line s-out)
	(newline))


(process-wait p)

