let s:bin = '/home/okno/WORKSHOP/RUST_sandbox/nvim-md/target/debug/nvim-md'
let s:MdRender = 'md_render'
let s:BuffChanged = "buffer_changed"

if !exists('s:nvimMdJobId')
    let g:nvimMdJobId = 0
endif

function! s:configureCommands()
    command! -nargs=0 MdRender :call s:md_render()
endfunction

call s:configureCommands() 

" Initialize RPC
function! s:initRpc()
    let id = jobstart([s:bin], { 'rpc': v:true })
    return id
endfunction

function! s:md_render()
    call Connect()
endfunction

function! Connect()
    let id = s:initRpc()
    let g:nvimMdJobId = id

    if 0 == id
        echoerr "nvimMd: cannot start rpc process"
    elseif -1 == id
        echoerr "nvimMd: rpc process is not executable"
    else
        echom g:nvimMdJobId
        echom s:BuffChanged
        autocmd TextChanged,TextChangedP,TextChangedI * :call rpcnotify(g:nvimMdJobId, "buffer_changed")
    endif

endfunction
