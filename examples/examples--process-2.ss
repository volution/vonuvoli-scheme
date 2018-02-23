

(define (exec _executable _arguments _options)
	(define _options_1 (append _options '((stdout . piped))))
	(define _process (process-spawn* _executable _arguments _options_1))
	(define _stream (process-stdout _process))
	(do
			((_line (read-line _stream) (read-line _stream)))
			((eof-object? _line))
		(write-string "[>>] ")
		(write-string _line)
		(newline))
	(process-wait _process))


(exec "pwd" '() '())
(exec "pwd" '() '((directory . "/tmp")))

(exec "env" '() '((env-empty . #t)))
(exec "env" '()
	'(
		(env-empty . #t)
		(env-include
			("a" . "a")
			("b" . "b"))
	))

(exec "ls" '("-l" "--" "/proc/self/fd") '())

