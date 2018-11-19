set number
set noswapfile
syntax on
filetype plugin on
set tabstop=4
set shiftwidth=4
set noexpandtab
set guicursor=
set nohlsearch

" terminalからの脱出
tnoremap <C-j> <C-\><C-n>

augroup fileTypeIndent
	autocmd!
	autocmd BufNewFile,BufRead *.elm setlocal tabstop=4 softtabstop=4 shiftwidth=4 expandtab
	autocmd BufNewFile,BufRead *.yml setlocal tabstop=2 softtabstop=2 shiftwidth=2 expandtab
	autocmd BufNewFile,BufRead *.js  setlocal tabstop=2 softtabstop=2 shiftwidth=2 expandtab
	autocmd BufNewFile,BufRead *.vue  setlocal tabstop=2 softtabstop=2 shiftwidth=2 expandtab
	autocmd BufNewFile,BufRead *.satyh setlocal tabstop=2 softtabstop=2 shiftwidth=2 expandtab
	autocmd BufNewFile,BufRead *.saty setlocal tabstop=2 softtabstop=2 shiftwidth=2 expandtab
	autocmd BufNewFile,BufRead *.elm setfiletype elm
augroup END

" NERDTreeの設定
augroup NERDTreeSetting
	autocmd!
	autocmd StdinReadPre * let s:std_in = 1
	if argc() == 0 || argc() == 1 && isdirectory(argv()[0]) && !exists("s:std_in")
		" ディレクトリ又は指定なしではツリーにフォーカス
		autocmd vimenter * NERDTreeToggle
	else
		" ファイル指定して開いた場合はバッファにフォーカス
		autocmd vimenter * NERDTreeToggle | wincmd p
	endif
	" NERDTree以外のバッファが閉じられたらNERDTreeも閉じる
	autocmd bufenter * if (winnr("$") == 1 && exists("b:NERDTree") && b:NERDTree.isTabTree()) | q | endif
augroup END

" 不可視文字の可視化
set list
set listchars=tab:»-,trail:-,nbsp:%

" 編集位置保存設定
if has("autocmd")
	au BufReadPost * if line("'\"") > 1 && line("'\"") <= line("$") | exe "normal! g`\"" | endif
endif

" undo永続化
let s:undoDir = expand("~/.nvimundo")
call system('mkdir ' . s:undoDir)
let &undodir = s:undoDir
set undofile

" 以下カラースキーム設定
set background=dark
let g:artesanal_transp_bg = 0
colorscheme artesanal

" 透過関連
highlight Normal ctermbg=NONE guibg=NONE
highlight NonText ctermbg=NONE guibg=NONE
highlight SpecialKey ctermbg=NONE guibg=NONE
highlight EndOfBuffer ctermbg=NONE guibg=NONE
highlight LineNr ctermbg=NONE guibg=NONE
