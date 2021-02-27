(module cfg.completion
  {require {a aniseed.core
            s aniseed.string
            lsp vim.lsp
            nvim aniseed.nvim}})

(def packages ["neoclide/coc.nvim"])

(global check_back_space 
  (fn []
    (let [col (- (nvim.fn.col ".") 1)]
      (do
        (or
          (= col 0)
          (= (string.match (string.sub (nvim.fn.getline ".") col col) "%s") nil))))))

(global show_documentation
  (fn []
    (if
      (>= (nvim.fn.index ["nvim" "help"] nvim.bo.filetype) 0)
      (nvim.command (.. "h " (nvim.fn.expand "<cword>")))
      (if (= (nvim.call_function "coc#rpc#ready" {}) 1)
        (nvim.fn.CocActionAsync "doHover")
        (nvim.command (.. "!" nvim.bo.keywordprg " " (nvim.fn.expand "<cword>")))))))


; check_back_spaceの関数定義は正しいと思うけどうまく動かない
(defn configure []
  (do
    (nvim.ex.inoremap
      "<silent><expr>" "<TAB>"
      "pumvisible() ? \"\\<C-n>\"" ":" "\"\\<TAB>\"")
    (nvim.ex.inoremap
      "<silent><expr>" "<S-TAB>"
      "pumvisible() ? \"\\<C-n>\"" ":" "\"\\<TAB>\"")
    (nvim.ex.inoremap
      "<silent><expr> <cr> pumvisible() ?"
      "coc#_select_confirm() :"
      "\"\\<C-g>u\\<CR>\\<c-r>=coc#on_enter()\\<CR>\"")
    (nvim.ex.nmap "<silent> [g" "<Plug>(coc-diagonostics-prev)")
    (nvim.ex.nmap "<silent> ]g" "<Plug>(coc-diagonostics-next)")
    (nvim.ex.nmap "<silent> gd" "<Plug>(coc-definition)")
    (nvim.ex.nmap "<silent> gy" "<Plug>(coc-type-definition)")
    (nvim.ex.nmap "<silent> gi" "<Plug>(coc-implementation)")
    (nvim.ex.nmap "<silent> gr" "<Plug>(coc-references)")
    (nvim.ex.nnoremap "<silent> K" ":lua show_documentation()<CR>")))
