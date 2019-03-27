" Constants for RPC messages.
let s:MdRender = 'md_render'

" Initialize the channel
if !exists('s:nvimMdJobId')
	let s:nvimMdJobId = 0
endif

" The path to the binary that was created out of 'cargo build' or 'cargo build --release". This will generally be 'target/release/name'
let s:bin = '/home/okno/WORKSHOP/RUST_sandbox/nvim-md/target/debug/nvim-md'

" Entry point. Initialize RPC. If it succeeds, then attach commands to the `rpcnotify` invocations.
function! s:connect()
  let id = s:initRpc()
  
  if 0 == id
    echoerr "nvimMd: cannot start rpc process"
  elseif -1 == id
    echoerr "nvimMd: rpc process is not executable"
  else
    " Mutate our jobId variable to hold the channel ID
    let s:nvimMdJobId = id 
 
     call s:configureCommands() 
  endif
endfunction

" Initialize RPC
function! s:initRpc()
  if s:nvimMdJobId == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true })
    return jobid
  else
    return s:nvimMdJobId
  endif
endfunction

  command! -nargs=0 MdRender :call s:md_render()

function! s:md_render()
  echom "ok sended"
  call s:initRpc()
  call rpcnotify(s:nvimMdJobId, s:MdRender)
endfunction
