extern crate libc;

// use std::io;
//use std::io::prelude::*;
//use std::io::BufReader;
//use std::io::BufWriter;
use libc::*;
use std::mem;

const LINE_SIZE: usize = 1000002;

fn main()
{
    let mut ret_buf: String = String::with_capacity(LINE_SIZE + 1);
    let mut buf_len;

    let fout = unsafe { fdopen(STDOUT_FILENO, ("w".as_ptr()) as *const i8)};
    let fin = unsafe { fdopen(STDIN_FILENO, ("r".as_ptr()) as *const i8)};

    buf_len = gets( &mut ret_buf, fin);
    let mut ptr = ret_buf.as_ptr() as *mut u8;
    mem::forget(ret_buf);
    ret_buf = unsafe { String::from_raw_parts(ptr, buf_len, LINE_SIZE + 1) } ;
    let mut count = u32::from_str_radix(&ret_buf.as_str(), 10).unwrap();


    loop
    { 
        if count == 0 { break; }
        buf_len = gets( &mut ret_buf, fin);
//        buf_len = unsafe { fread( ptr as *mut c_void, 1, LINE_SIZE as size_t, fin) };
        if buf_len == 0 { break; }
        
        ptr = ret_buf.as_ptr() as *mut u8;
        mem::forget(ret_buf);
        ret_buf = unsafe { String::from_raw_parts(ptr, buf_len, LINE_SIZE + 1) } ;
        if buf_len == 1 || (buf_len == 2 && ret_buf == "10")
        {
            unsafe { fwrite(b"11\n".as_ptr() as *const c_void, 3, 1, fout); }
        }else
        {
            calc_next_palindrome( &mut ret_buf);
            buf_len = ret_buf.len();
            unsafe { fwrite( ret_buf.as_ptr() as *const c_void, buf_len, 1, fout);
                fwrite(b"\n".as_ptr() as *const c_void, 1, 1, fout);
            };
        }
        count -= 1;
    }

}



fn gets(ret_buf: &mut String, fin: *mut FILE) -> usize
{
        let ptr = ret_buf.as_ptr() as *mut u8;
        let readedptr = unsafe { fgets( ptr as *mut c_char, LINE_SIZE as c_int, fin )} ;
        if readedptr.is_null() { 
            return 0; 
        }
        let mut buf_len = unsafe { strlen( ptr as *const c_char)} as usize;
        if buf_len == 0 
        { 
            return 0; 
        }
        let lastchar = unsafe { *ptr.offset((buf_len-1) as isize)};
        if lastchar == '\n' as u8 
        { 
            buf_len -= 1;
        }
        buf_len
}

fn calc_next_palindrome( mut read_buf: &mut String)
{
    let half = read_buf.len() >> 1;
    let center_len = read_buf.len() - half * 2;
    let (new_half, new_center_len) = if !is_rleft_more_than_right(&read_buf)
        {
            let new_half_center_len = inc_string( &mut read_buf, half+center_len);
            if new_half_center_len == (half+center_len)
            {
                (half, center_len)
            }else
            {
                if center_len == 0 { 
                    (half, 1)
                } else
                {
                    (half+1, 0)
                }
            }
        }else
        {
            (half, center_len)
        };
    reverse_in_place( &mut read_buf, new_half, new_center_len);
}


fn reverse_in_place(read_buf: &mut String, half: usize, center_len: usize)
{
    let newlen = half + center_len + half ;
    let cap = read_buf.capacity();
    let mut chr: u8 ;

    if cap < newlen { return; }

    let mut n = half;
    read_buf.truncate(half+center_len);
    while n > 0
    {
        chr = read_buf.as_bytes()[n-1];
        read_buf.push( chr as char);
        n-=1;
    }
}

fn is_rleft_more_than_right( read_buf: &String) -> bool
{
    let buf_len = read_buf.len();
    let mut n = buf_len >> 1;
    let buf = read_buf.as_bytes();
    while n > 0 
    {
        if buf[ buf_len - n ] > buf[ n-1]
        {
            return false;
        }
        if buf[buf_len - n ] < buf [n-1]
        {
            return true;
        }
        n -= 1;
    }
    false
}

fn inc_string(read_buf: &mut String, buf_len: usize) -> usize
{
    let mut n = buf_len as isize;
    let mut p_buf = read_buf.as_ptr() as *mut u8;
    let mut chr: u8;
    let mut stillinc = true;
    let mut ret = buf_len;
    while n > 0
    {
        chr = unsafe { *p_buf.offset(n-1) };
        if !stillinc { break;}
        if chr == '9' as u8
        {
            unsafe { *p_buf.offset(n-1) = '0' as u8; };
        }else
        {
            unsafe { *p_buf.offset(n-1) = (chr + 1) as u8; }; 
            stillinc = false;
        }
        n -= 1;
    }
    if stillinc
    {
        ret += 1;
        n = buf_len as isize;
        while n > 0
        {
            chr = unsafe { *p_buf.offset(n-1)}; 
            unsafe { *p_buf.offset(n) = chr};
            n -= 1;
        }
        unsafe { *p_buf = '1' as u8};
    }
    ret
}
