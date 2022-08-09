syntax on
filetype off           
set maxmempattern=5000
set clipboard+=unnamedplus
set background=dark
set nocompatible
set foldmethod=syntax
set foldnestmax=4
set number
"set colorcolumn=81
"set cuc
"set cul

if empty(glob('~/.vim/autoload/plug.vim'))
  silent !curl -fLo ~/.vim/autoload/plug.vim --create-dirs
    \ https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
  autocmd VimEnter * PlugInstall --sync | source $MYVIMRC
endif

call plug#begin('~/.vim/plugged')
Plug 'https://github.com/scrooloose/nerdtree.git'
Plug 'https://github.com/tpope/vim-fugitive.git'
Plug 'fatih/vim-go'
Plug 'wakatime/vim-wakatime'
Plug 'neoclide/coc.nvim', {'branch': 'release'}
Plug 'w0rp/ale'
Plug 'SirVer/ultisnips'
Plug 'honza/vim-snippets'
Plug 'vim-airline/vim-airline'
Plug 'python-mode/python-mode', { 'branch': 'develop' }
Plug 'hotoo/pangu.vim'
Plug 'preservim/nerdcommenter' 
Plug 'wfxr/minimap.vim'
Plug 'Yggdroot/LeaderF', { 'do': './install.sh' }
Plug 'easymotion/vim-easymotion'

"Plug 'rust-lang/rust.vim'
"Plug 'racer-rust/vim-racer'
"Plug 'vim-syntastic/syntastic'
Plug 'majutsushi/tagbar'
"Plug 'elixir-editors/vim-elixir'
"Plug 'ryanoasis/vim-devicons'
"Plug 'Valloric/YouCompleteMe'
call plug#end()

filetype plugin indent on

let g:airline_powerline_fonts = 1
let g:go_gopls_enabled = 0
let g:go_gopls_options = ['-remote=auto']
let g:go_def_mapping_enabled = 0
let g:go_fmt_command = "goimports"
let g:go_fmt_autosave = 1
let g:go_metalinter_enabled = ['vet', 'golint', 'errcheck']
"let g:go_addtags_transform = "camelcase"
let g:NERDCustomDelimiters = { 'go': { 'left': '//' } }
let g:NERDSpaceDelims = 1

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
let g:ale_open_list = 0
let g:ale_linters={
\ 'go':['govet','revive'],
\ 'java':[],
\ 'rust':['cargo'],
\ 'python':[],
\}

let g:rustfmt_autosave = 0
let g:rustfmt_options = '--edition 2018'
let g:pymode_python = 'python3'
let g:pymode_indent = 1

let g:coc_global_extensions = ['coc-ultisnips', 'coc-python', 'coc-rust-analyzer', 'coc-phpls', 'coc-tsserver', 'coc-vetur']
let g:coc_channel_timeout = 3
let g:coc_user_config = {}
let g:coc_user_config['languageserver'] = {}

let g:coc_user_config['languageserver']['ccls'] = {
		\  'command': 'ccls',
		\  'filetypes': ['c', 'cpp'],
		\  'trace.server': 'verbose',
		\  'rootPatterns': ['.ccls-root', 'compile_commands.json'],
		\}

let g:coc_user_config['languageserver']['golang'] = {
		\   'command': 'gopls',
		\   'args': ['-remote=auto'],
		\   'rootPatterns': ['go.mod'],
		\   'filetypes': ['go']
		\}

let g:coc_user_config['coc.preferences.formatOnSaveFiletypes'] = ['rust', 'cpp']
let g:coc_user_config['coc.preferences.rootPatterns'] = ["Cargo.toml"]
"let g:coc_user_config['python.pythonPath'] = '/home/liyiheng/Downloads/work/miniconda3/bin/python'
let g:coc_user_config['rust.rustfmt_path'] = '/home/liyiheng/.cargo/bin/rustfmt'
let g:coc_user_config['rust-analyzer.serverPath'] = '/home/liyiheng/.cargo/bin/rust-analyzer'
let g:coc_user_config['diagnostic.displayByAle'] = v:true

"inoremap <expr> <Tab> pumvisible() ? "\<C-n>" : "\<Tab>"
"inoremap <expr> <S-Tab> pumvisible() ? "\<C-p>" : "\<S-Tab>"
inoremap <silent><expr> <CR> coc#pum#visible() ? coc#pum#confirm() : "\<C-g>u\<CR>\<c-r>=coc#on_enter()\<CR>"
inoremap <silent><expr> <C-x><C-z> coc#pum#visible() ? coc#pum#stop() : "\<C-x>\<C-z>"
" remap for complete to use tab and <cr>
inoremap <silent><expr> <TAB>
      \ coc#pum#visible() ? coc#pum#next(1):
      \ CheckBackspace() ? "\<Tab>" :
      \ coc#refresh()
inoremap <expr><S-TAB> coc#pum#visible() ? coc#pum#prev(1) : "\<C-h>"
inoremap <silent><expr> <c-space> coc#refresh()
function! CheckBackspace() abort
  let col = col('.') - 1
  return !col || getline('.')[col - 1]  =~# '\s'
endfunction

hi CocSearch ctermfg=12 guifg=#18A3FF
hi CocMenuSel ctermbg=109 guibg=#13354A

" neovim 0.6.0 后，Y 默认映射为 y$, 不符合本人习惯
" 习惯映射为 yy
silent! unmap Y

nmap <silent> gd <Plug>(coc-definition)
nmap <silent> gy <Plug>(coc-type-definition)
nmap <silent> gi <Plug>(coc-implementation)
nmap <silent> gr <Plug>(coc-references)
nmap <leader>rn <Plug>(coc-rename)

nmap <F8> :TagbarToggle<CR>
"nmap <F8> :Vista!!<CR>
nmap <F7> :NERDTreeToggle<CR>

"au FileType rust nmap gd <Plug>(rust-def)
au FileType rust nmap gs <Plug>(rust-def-split)
au FileType rust nmap gx <Plug>(rust-def-vertical)
au FileType rust nmap <leader>gd <Plug>(rust-doc)

augroup CloseLoclistWindowGroup
  autocmd!
  autocmd QuitPre * if empty(&buftype) | lclose | endif
augroup END

