#![feature(trace_macros)] //trace_macros!(true);
#[macro_use]
extern crate dmutil;

#[test]
fn test(){
	assert!(reverse_tt!{[< 1]{2 -}|[6 -]{5 -}});
}



