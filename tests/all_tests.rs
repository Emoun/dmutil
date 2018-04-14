#![feature(trace_macros)] //trace_macros!(true);
#![recursion_limit="128"]
#[macro_use]
extern crate dmutil;

mod macros;
/*
macro_rules! mac1{
	(	@expand[
			[$($prefix:tt)*] $($expand_rest:tt)*
		]
		3 4 mac2Exp 3 4
	) => {
		check_expand!{
			[$($expand_rest)*]
			mac1Exp $($prefix)*
		}
	};
}

macro_rules! mac2{
	(	@expand[
			[$($prefix:tt)*] $($expand_rest:tt)*
		]
		5 6
	) => {
		check_expand!{
			[$($expand_rest)*]
			mac2Exp $($prefix)*
		}
	};
}

#[test]
fn test(){
	trace_macros!(true);
	expand!{
		1 2 mac1!{ 3 4 mac2!{ 5 6 } 3 4 } 1 2
	}
	trace_macros!(false);
}
*/



