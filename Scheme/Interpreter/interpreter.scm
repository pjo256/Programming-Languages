                            ;;;Mini-Scheme Interpreter


;;; Your first task is to understand this. 

(define (repl)     ;;; the read-eval-print loop.
  (display "--> ") 
  (let ((exp (read)))
    (cond ((equal? exp '(exit))      ; (exit) only allowed at top level
	   'done)
	  (else  (display (top-eval exp))
            (newline)
		 (repl))
	  )) 
)


(define (my-load filename)       ;; don't want to redefine the Scheme LOAD
  (load-repl (open-input-file filename)))

(define (my-apply func args)
  (let ((decl (car func)))
    (cond 
          ((eq? decl 'primitive-function) (apply (cadr func) args))
          ((eq? decl 'closure) (handle-call (cons func args)))
          (else (func args))
     )))


(define (load-repl port)
  (let ((exp (read port)))
    (cond ((eof-object? exp) 'done)
	  (else (display (top-eval exp))
		(load-repl port)))
	  ))



;; insert!, below, is a destructive update of a list L, inserting the
;; parameter val onto the front of L (so that L is actually modified).
;; insert! must only be used where absolutely necessary, e.g. when an
;; environment must be destructively updated to allow for recursion
;; (see the implementation of (define ...) below).

;; As their names imply, set-car! and set-cdr! destructively modify 
;; the car field and cdr field of a cons cell, respectively. They are
;; built-in functions (see *global-env* below).

(define (insert! val L)
  (set-cdr! L (cons (car L) (cdr L)))
  (set-car! L val)
  )


;; (define ....) is only allowed at the top level and affects only the 
;; global environment. Only the basic form of define is supported here.

(define (top-eval exp)
  (cond ((not (pair? exp)) (my-eval exp *global-env*))
	((eq? (car exp) 'define)   
	 (handle-define (cadr exp) (cddr exp)))
	(else (my-eval exp *global-env*))
	))


(define (lookup var env)
  (let ((item (assoc var env)))  ;; assoc returns #f if var not found in env
    (cond ((not item) (display "Error: Undefined Symbol ")
		      (display var) (newline))
	  (else (cadr item))
	  )))

(define (handle-define key values)
  (cond ((and (not (pair? key)) (null? (cdr values))) (insert! (list key (my-eval (car values) *global-env*)) *global-env*)
              key)
        ((and (pair? key) (not (pair? (cdr values)))) (let* ((f (car key))
                            (body (cons 'lambda (cons (cdr key) (list (car values)))))
                           )
                           (insert! (list f (my-eval body *global-env*)) *global-env*)
                     ))
        ((and (pair? key) (pair? (cdr values))) 
         (let* ((f (car key))
                (body (cons 'lambda (cons (cdr key) values))))
               (insert! (list f (my-eval body *global-env*)) *global-env*))
        )       
        (else (display "Error: define syntax is incorrect."))
  )
)

(define (handle-if test then-exp else-exp env)
  (if (my-eval test env)
      (my-eval then-exp env)
      (my-eval else-exp env)))

(define (handle-cond clauses env)
  (cond 
        ((eq? 'else (caar clauses)) (handle-block (cdar clauses) env))
        ((my-eval (caar clauses) env) (handle-block (cdar clauses) env))
        (else (handle-cond (cdr clauses) env))
  )
)


;; still missing let, let*, letrec, the syntax for (define (f x) ...),
;; cond, begin (block).

(define (my-eval exp env)
  (cond
   ((symbol? exp) (lookup exp env))
   ((not (pair? exp)) exp)
   ((eq? (car exp) 'quote) (cadr exp))
   ((eq? (car exp) 'if)
    (handle-if (cadr exp) (caddr exp) (cadddr exp) env))
   ((eq? (car exp) 'cond)
    (handle-cond (cdr exp) env)
   )
   ((eq? (car exp) 'lambda)
    (list 'closure exp env))
   ((eq? (car exp) 'letrec)
    (handle-letrec (cadr exp) (cddr exp) env))  ;; see explanation below
   ((eq? (car exp) 'let)
    (handle-let (cadr exp) (cddr exp) env))
   ((eq? (car exp) 'let*)
    (handle-let* (cadr exp) (cddr exp) env))
   ((eq? (car exp) 'begin)
    (handle-begin (cdr exp) env))
   (else
    (handle-call (map (lambda (sub-exp) (my-eval sub-exp env)) exp)))
   ))


(define (bind formals actuals)
  (cond ((null? formals) '())
	(else (cons (list (car formals) (car actuals))
		    (bind (cdr formals) (cdr actuals))))
	))

(define (handle-begin exps env)
  (handle-block exps env)
)

(define (handle-block block env)
  (cond ((null? block) (display "Error: Can't have empty block or body"))
	((null? (cdr block)) (my-eval (car block) env))
	(else (my-eval (car block) env)
	      (handle-block (cdr block) env))
	))
    

; Here's how handle-letrec should implement LETREC
; 0) The parameters are the defs,(e.g. ((f exp1) (g exp2)), and the body,
;    which is a list of expressions, e.g. ((display x) (f (g 1)))
; 1) create an association list binding the new names introducted by
;    the letrec to uninitialized values (e.g. the symbol '*uninitialized*).
;    For example, if the new names are x and y, then create 
;    ((x *uninitialized*) (y *uninitialized*))
; 2) create a new-env by appending the above association list to env.
; 3) eval the right hand side of each def using new-env
; 4) destructively modify new-env to replace the unitialized value for each
;    new name with its correspondinng value.
; 5) evaluate the body of the letrec using new-env


(define (handle-letrec defs body env)
  (let* ((association-list (get-and-init defs))
         (new-env (append association-list env))
         (values (get-values defs))
         (new-values (map (lambda (value) (my-eval value new-env)) values))
        )
         
        (set-values new-env new-values)   
        (handle-block body new-env)
  )
)

(define (handle-let defs body env)
  (let* ((eval-defs (map (lambda (x) (list (car x) (my-eval (cadr x) env))) defs))
         (new-env (append eval-defs env))
        )
        (handle-block body new-env)
  )
)

(define (handle-let* defs body env)
  (cond ((null? defs) (handle-block body env))
        (else (let ((eval-env (cons (list (caar defs) (my-eval (cadar defs) env)) env))) 
                (handle-let* (cdr defs) body eval-env)
              )
        )
  )                                   
)

;Binds new names introduced by letrec to default value.

;Input of the from ((a b) (c d) ...) -> ((a *null*) (c *null*) ... )
(define (get-and-init defs)
  (cond ((null? defs) '())            
        ((null? (cdr defs)) (list (list (caar defs) *null*)))    
        (else (append (list (list (caar defs) *null*))           
            (get-and-init (cdr defs))))
  )
)

;Gets values of each name
;((a b) (a (+ 1 c)) ... ) -> (b (+ 1 c))
(define (get-values defs)
  (cond ((null? (cdr defs)) (cdar defs))      
        (else (append (cdar defs)         
            (get-values (cdr defs))))
  )
)

;Destructively modify uninit'd values in a given environment with new values. (
;In letrec, the association list is passed in with null values that must be replaced.
(define (set-values env values)
  (cond ((null? values) #f)
        ((eq? *null* (cadar env)) 
             (set-car! (cdar env) (car values))
             (set-values (cdr env) (cdr values)))           
        (else (set-values (cdr env) (cdr values)))
  )
)


(define (handle-call evald-exps)
  (let ((fn (car evald-exps))
	(args (cdr evald-exps)))
   (cond
    ((eq? (car fn) 'closure)
     (let ((formals (cadr (cadr fn)))
	   (body (cddr (cadr fn)))
	   (env (caddr fn)))
       (handle-block body (append (bind formals args) env))))
    ((eq? (car fn) 'primitive-function)
     (apply (cadr fn) args))
    (else (display "Error: Calling non-function"))
    )))

;;-------------------- Here is the initial global environment --------

(define *null* '(null))
(define *global-env*
   (list (list 'car (list 'primitive-function car))
	(list 'cdr (list 'primitive-function cdr))
	(list 'set-car! (list 'primitive-function set-car!))
	(list 'set-cdr! (list 'primitive-function set-cdr!))
	(list 'cons (list 'primitive-function cons))
	(list 'list (list 'primitive-function list))
	(list '+ (list 'primitive-function +))
	(list '- (list 'primitive-function -))
	(list '* (list 'primitive-function *))
	(list '= (list 'primitive-function =))
	(list '< (list 'primitive-function <))
	(list '> (list 'primitive-function >))
	(list '<= (list 'primitive-function  <=))
	(list '>= (list 'primitive-function >=))
	(list 'eq? (list 'primitive-function eq?))
   (list 'pair? (list 'primitive-function pair?))
   (list 'number? (list 'primitive-function number?))
	(list 'symbol? (list 'primitive-function symbol?))
	(list 'null? (list 'primitive-function null?))
	(list 'read (list 'primitive-function read))
   (list 'display (list 'primitive-function  display))
	(list 'open-input-file (list 'primitive-function open-input-file))
	(list 'close-input-port (list 'primitive-function close-input-port))
   (list 'eof-object? (list 'primitive-function eof-object?))
	(list 'load (list 'primitive-function my-load)) ;;defined above
   (list 'apply (list 'primitive-function my-apply))
   (list 'newline (list 'primitive-function newline))
	))
