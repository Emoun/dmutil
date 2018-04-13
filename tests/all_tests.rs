//#![feature(trace_macros)] //trace_macros!(true);
#[macro_use]
extern crate dmutil;

mod macros;

#[test]
fn test(){
	assert!(reverse_tt!{[< 1]{2 -}|[6 -]{5 -}});
}

macro_rules! dec_struct{
	{
		[$name:ident]
		[$($generics:tt)*]
		[$($constraints:tt)*]
		[$($body:tt)*]
	}=>{
		struct $name<$($generics)*>
		where $($constraints)*
		{$($body)*}
	};
}


/* We want it to expand to:

	struct SomeStruct < V , W >
		where W : PartialEq , V : PartialEq
	{
		ph1 : PhantomData < W > ,
		ph2 : PhantomData < V >
	}
*/



