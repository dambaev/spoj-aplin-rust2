//extern crate libc;

use std::vec;
use std::io;
use std::io::prelude::*;
//use libc::*;
use std::mem;

const BUF_SIZE: usize = 500 * 1000001;

fn main()
{
    let mut ret_buf: String = String::with_capacity(BUF_SIZE);
    let mut inp = io::stdin();
    let mut outp = io::stdout();
//    let fp = unsafe { fdopen(0, "r") };

    let mut buf_len;

    while let Ok(n) = inp.read_line(&mut ret_buf)
    {
        if n == 0 {break;}
        buf_len = n;
        if buf_len > 0 { buf_len -= 1; }
        ret_buf.truncate(buf_len);
        calc_next_palindrome( &mut ret_buf);
        outp.write_all(ret_buf.as_bytes());
        outp.write_all("\n".as_bytes());
        ret_buf.clear();
    }
}

fn calc_next_palindrome( mut read_buf: &mut String)
{
    let half = read_buf.len() >> 1;
    let center_len = read_buf.len() - half * 2;
    if isRLeftMoreThanRight(&read_buf)
    {
        reverseInPlace(&mut read_buf, half, center_len);
    }
}


fn reverseInPlace(read_buf: &mut String, half: usize, center_len: usize)
{
    println!("reverse");
    let newlen = (half + center_len + half) ;
    let cap = read_buf.capacity();
    let mut chr: char ;

    if (cap < newlen) { return; }

    let mut n = half;
//    let mut p_buf = unsafe { read_buf.as_mut_vec()};
    while (n > 0)
    {
        chr = read_buf.char_at(n-1);
        read_buf.push( chr);
        n-=1;
    }
}

fn isRLeftMoreThanRight( read_buf: &String) -> bool
{
    true
}


