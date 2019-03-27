syntax on
filetype off                  " required
set clipboard+=unnamedplus
set nocompatible              " be iMproved, required
set foldmethod=syntax
set foldnestmax=1
set number                      " Show line numbers

set rtp+=~/.vim/bundle/Vundle.vim

call vundle#begin()


Plugin 'VundleVim/Vundle.vim'
Plugin 'https://github.com/scrooloose/nerdtree.git'
Plugin 'https://github.com/tpope/vim-fugitive.git'
Plugin 'racer-rust/vim-racer'
Plugin 'rust-lang/rust.vim'
Plugin 'fatih/vim-go'
Plugin 'wakatime/vim-wakatime'
Plugin 'Valloric/YouCompleteMe'
Plugin 'vim-syntastic/syntastic'
"Plugin 'majutsushi/tagbar'
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
let g:go_addtags_transform = "camelcase"

set statusline+=%#warningmsg#
set statusline+=%{SyntasticStatuslineFlag()}
set statusline+=%*

let g:syntastic_always_populate_loc_list = 1
let g:syntastic_auto_loc_list = 1
let g:syntastic_check_on_open = 1
let g:syntastic_check_on_wq = 0
let g:syntastic_go_checkers = ['govet','golint']
let g:syntastic_ocaml_checkers = ['merlin']

let g:rustfmt_autosave = 1
let g:pymode_python = 'python3'

nmap <F8> :TagbarToggle<CR>
nmap <F7> :NERDTreeToggle<CR>

au FileType rust nmap gd <Plug>(rust-def)
au FileType rust nmap gs <Plug>(rust-def-split)
au FileType rust nmap gx <Plug>(rust-def-vertical)
au FileType rust nmap <leader>gd <Plug>(rust-doc)

