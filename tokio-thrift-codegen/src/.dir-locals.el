((nil
  (eval
   (lambda ()
     (when (string= (file-name-extension buffer-file-name)
                    "hbs")
       (rust-mode))))))
