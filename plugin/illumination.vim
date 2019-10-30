let s:bin = 'illumination'

if !exists('s:nvimMdJobId')
    let s:nvimMdJobId = 0
endif

function! s:configureCommands()
    command! -nargs=0 Illuminate :call s:render()
    command! -nargs=0 IlluminateClose :call s:render_stop()
    command! -nargs=0 IlluminateLock :call s:lock()
    command! -nargs=1 IlluminateRustDoc :call s:docopen()
    command! -nargs=0 IlluminateDebug :call s:debug()
endfunction

call s:configureCommands() 

function! s:initRpc()
    let id = jobstart([s:bin], { 'rpc': v:true })
    return id
endfunction

function! s:render_stop() 
    echom s:nvimMdJobId
    call jobstop(s:nvimMdJobId)
endfunction

function! s:render()
    let id = s:initRpc()
    if 0 == id
        echoerr "nvimMd: cannot start rpc process"
    elseif -1 == id
        echoerr "nvimMd: rpc process is not executable"
    else
        let s:nvimMdJobId = id
    endif
endfunction


let s:RustDocOpen = 'rust_doc_open'
let s:Lock = 'lock'

function! s:lock()
    call rpcnotify(s:nvimMdJobId, s:Lock)
endfunction

function! s:docopen()
  call rpcnotify(s:nvimMdJobId, s:RustDocOpen)
endfunction

function! s:debug()
    let s:nvimMdJobId = g:nvimMdJobId
    echo s:nvimMdJobId
endfunction
