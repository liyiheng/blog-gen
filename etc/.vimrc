
set clipboard+=unnamedplus
set nocompatible              " be iMproved, required
filetype off                  " required
syntax on
set number                      " Show line numbers
set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()

" ------------------ DOC ------------------------
" To ignore plugin indent changes, instead use:
" filetype plugin on
"
" Brief help
" :PluginList       - lists configured plugins
" :PluginInstall    - installs plugins; append `!` to update or just :PluginUpdate
" :PluginSearch foo - searches for foo; append `!` to refresh local cache
" :PluginClean      - confirms removal of unused plugins; append `!` to auto-approve removal
"
" see :h vundle for more details or wiki for FAQ
" Put your non-Plugin stuff after this line


Plugin 'VundleVim/Vundle.vim'
Plugin 'https://github.com/scrooloose/nerdtree.git'
Plugin 'https://github.com/tpope/vim-fugitive.git'
Plugin 'racer-rust/vim-racer'
Plugin 'rust-lang/rust.vim'
Plugin 'fatih/vim-go'
Plugin 'wakatime/vim-wakatime'
Plugin 'Valloric/YouCompleteMe'
Plugin 'vim-syntastic/syntastic'
Plugin 'majutsushi/tagbar'
Plugin 'elixir-editors/vim-elixir'
Plugin 'slashmili/alchemist.vim'
"Plugin 'ryanoasis/vim-devicons'
Plugin 'vim-airline/vim-airline'
Plugin 'python-mode/python-mode', { 'branch': 'develop' }

call vundle#end()            " required
filetype plugin indent on    " required

let g:airline_powerline_fonts = 1
let g:go_fmt_command = "goimports"
let g:go_metalinter_enabled = ['vet', 'golint', 'errcheck']

"let g:ycm_rust_src_path = '/home/liyiheng/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src'

set statusline+=%#warningmsg#
set statusline+=%{SyntasticStatuslineFlag()}
set statusline+=%*

let g:syntastic_always_populate_loc_list = 1
let g:syntastic_auto_loc_list = 1
let g:syntastic_check_on_open = 1
let g:syntastic_check_on_wq = 0
let g:syntastic_go_checkers = ['golint','govet']
"let g:syntastic_go_go_args = "vet"

let g:rustfmt_autosave = 1
let g:pymode_python = 'python3'

nmap <F8> :TagbarToggle<CR>
nmap <F7> :NERDTreeToggle<CR>

au FileType rust nmap gd <Plug>(rust-def)
au FileType rust nmap gs <Plug>(rust-def-split)
au FileType rust nmap gx <Plug>(rust-def-vertical)
au FileType rust nmap <leader>gd <Plug>(rust-doc)
