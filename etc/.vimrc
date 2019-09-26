syntax on
filetype off           
set clipboard+=unnamedplus
set nocompatible
set foldmethod=syntax
set foldnestmax=2
set number
"set colorcolumn=81

call plug#begin('~/.vim/plugged')
Plug 'https://github.com/scrooloose/nerdtree.git'
Plug 'https://github.com/tpope/vim-fugitive.git'
Plug 'racer-rust/vim-racer'
Plug 'rust-lang/rust.vim'
Plug 'fatih/vim-go'
Plug 'wakatime/vim-wakatime'
Plug 'neoclide/coc.nvim', {'branch': 'release'}
Plug 'w0rp/ale'
Plug 'SirVer/ultisnips'
Plug 'honza/vim-snippets'
Plug 'vim-airline/vim-airline'
Plug 'python-mode/python-mode', { 'branch': 'develop' }
"Plug 'VundleVim/Vundle.vim'
"Plug 'vim-syntastic/syntastic'
"Plug 'majutsushi/tagbar'
"Plug 'elixir-editors/vim-elixir'
"Plug 'slashmili/alchemist.vim'
"Plug 'ryanoasis/vim-devicons'
"Plug 'Valloric/YouCompleteMe'
"Plug 'autozimu/LanguageClient-neovim', { 'branch': 'next'}
call plug#end()
filetype plugin indent on    " required

let g:airline_powerline_fonts = 1
let g:go_fmt_command = "goimports"
let g:go_metalinter_enabled = ['vet', 'golint', 'errcheck']
let g:go_addtags_transform = "camelcase"

" <tab> used by coc.nvim
"let g:UltiSnipsExpandTrigger="<tab>"
let g:UltiSnipsExpandTrigger="<c-x>"
"let g:UltiSnipsJumpForwardTrigger="<c-b>"
"let g:UltiSnipsJumpBackwardTrigger="<c-z>"
"let g:UltiSnipsEditSplit="vertical"

"set statusline+=%#warningmsg#
"set statusline+=%{SyntasticStatuslineFlag()}
"set statusline+=%*
"let g:syntastic_always_populate_loc_list = 1
"let g:syntastic_auto_loc_list = 1
"let g:syntastic_check_on_open = 1
"let g:syntastic_check_on_wq = 0
"let g:syntastic_go_checkers = ['govet','golint']
"let g:syntastic_go_checkers = []
"let g:syntastic_ocaml_checkers = ['merlin']
"let g:syntastic_java_checkers = ['checkstyle']
let g:ale_open_list = 1
let g:ale_linters={
\ 'go':['govet','golint'],
\ 'java':[],
\ 'python':[],
\}

let g:rustfmt_autosave = 1
let g:pymode_python = 'python3'
let g:pymode_indent = 1

"\ 'rust': ['~/.cargo/bin/rustup', 'run', 'stable', 'rls'],
let g:LanguageClient_serverCommands = {
    \ 'rust': ['~/.cargo/bin/rustup', 'run', 'stable', 'rls'],
    \ 'python': ['/usr/bin/pyls']
    \ }
inoremap <expr> <Tab> pumvisible() ? "\<C-n>" : "\<Tab>"
inoremap <expr> <S-Tab> pumvisible() ? "\<C-p>" : "\<S-Tab>"

nmap <silent> gd <Plug>(coc-definition)
nmap <silent> gy <Plug>(coc-type-definition)
nmap <silent> gi <Plug>(coc-implementation)
nmap <silent> gr <Plug>(coc-references)

nmap <F8> :TagbarToggle<CR>
nmap <F7> :NERDTreeToggle<CR>

"au FileType rust nmap gd <Plug>(rust-def)
au FileType rust nmap gs <Plug>(rust-def-split)
au FileType rust nmap gx <Plug>(rust-def-vertical)
au FileType rust nmap <leader>gd <Plug>(rust-doc)

augroup CloseLoclistWindowGroup
  autocmd!
  autocmd QuitPre * if empty(&buftype) | lclose | endif
augroup END

