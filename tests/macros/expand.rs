
mod test_prefix{
	macro_rules ! test_macro{
		( ! ! ) =>{
			expand!{
				struct
				test_macro!{??}
			}
		};
		( 	@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			??
		) =>{
			check_expand!{
				[$($expand_rest)*]
				SomeName test_macro!{| |}
				$($postfix)*
			}
		};
		( 	@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			| |
		) =>{
			check_expand!{
				[$($expand_rest)*]
				{field: u32}
				$($postfix)*
			}
		};
	}
	test_macro!(!!);
}
mod test_postfix{
	macro_rules! test_macro{
		(!!)=>{
			expand!{
				struct
				test_macro!{??}
			}
		};
		(
			@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			??
		)=>{
			check_expand!{
				[$($expand_rest)*]
				test_macro!{||}
				{field: u32} $($postfix)*
			}
		};
		(
			@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			||
		)=>{
			check_expand!{
				[$($expand_rest)*]
				SomeStruct
				$($postfix)*
			}
		};
	}
	test_macro!(!!);
}
mod test_multiple_calls{
	use std::marker::PhantomData;
	macro_rules! mac1{
		{
			@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			$typ:tt
		}=>{
			check_expand!{
				[$($expand_rest)*]
				$typ
				$($postfix)*
			}
		};
	}
	macro_rules! mac2{
		($V:ident,$eq:tt)=>{
			expand!{
				struct $V<V,W> where W:
				mac1!{$eq},
				V:
				mac1!{$eq}
				{ph1: PhantomData<W>,ph2: PhantomData<V>}
			}
		}
	}
	trace_macros!(true);
	mac2!(SomeStruct, PartialEq);
}
/*
mod test_nested_calls{
	use std::marker::PhantomData;
	macro_rules! mac1{
		{}=>{
			{ph1: PhantomData<W>,ph2: PhantomData<V>}
		};
	}
	macro_rules! mac2{
		{
			@expand [[$($prefix:tt)*] [$($postfix:tt)*]]
			$($all:tt)*
		} =>{
			expand!{$($prefix)* {$($all)*} $($postfix)*}
		};
		{
			$($to_expand)*
		}=>{
			expand!{mac2!{$($to_expand)*}}
		}
	}
	macro_rules! mac3{
		($some:ident)=>{
			expand!{
				struct some<V,W>
				mac2!{
					mac1!{!!}
				}
			}
		};
	}
	trace_macros!(true);
	mac3!{SomeStruct}
}
*/
//--------------------------------------------------------------------------------
/*
	@check_expansion[
		[[][]]
	]
	1 2 mac1!{ 3 4 mac2!{ 5 6 } 3 4 } 1 2
*/
/*
	@check_expansion[
		[[1][]]
	]
	2 mac1!{ 3 4 mac2!{ 5 6 } 3 4 } 1 2
*/
/*
	@check_expansion[
		[[2 1][]]
	]
	mac1!{ 3 4 mac2!{ 5 6 } 3 4 } 1 2
*/
/*
	@check_expansion[
		[[mac1 2 1][]]
	]
	!{ 3 4 mac2!{ 5 6 } 3 4 } 1 2
*/
/*
	@check_expansion[
		[[! mac1 2 1][]]
	]
	{ 3 4 mac2!{ 5 6 } 3 4 } 1 2
*/
/*
	@check_expansion[
		[[][]]
		[[! mac1 2 1][1 2]{}]
	]
	3 4 mac2!{ 5 6 } 3 4
*/
/*
	@check_expansion[
		[[3][]]
		[[! mac1 2 1][1 2]{}]
	]
	4 mac2!{ 5 6 } 3 4
*/
/*
	@check_expansion[
		[[4 3][]]
		[[! mac1 2 1][1 2]{}]
	]
	mac2!{ 5 6 } 3 4
*/
/*
	@check_expansion[
		[[mac2 4 3][]]
		[[! mac1 2 1][1 2]{}]
	]
	!{ 5 6 } 3 4
*/
/*
	@check_expansion[
		[[! mac2 4 3][]]
		[[! mac1 2 1][1 2]{}]
	]
	{ 5 6 } 3 4
*/
/*
	@check_expansion[
		[[][]]
		[[! mac2 4 3][3 4]{}]
		[[! mac1 2 1][1 2]{}]
	]
	5 6
*/
/*
	@check_expansion[
		[[5][]]
		[[! mac2 4 3][3 4]{}]
		[[! mac1 2 1][1 2]{}]
	]
	6
*/
/*
	@check_expansion[
		[[6 5][]]
		[[! mac2 4 3][3 4]{}]
		[[! mac1 2 1][1 2]{}]
	]
*/
/*
	@check_expansion[
		[[5][]]
		[[! mac2 4 3][3 4]{6}]
		[[! mac1 2 1][1 2]{}]
	]
*/
/*
	@check_expansion[
		[[! mac2 4 3][3 4]{5 6}]
		[[! mac1 2 1][1 2]{}]
	]
*/
/*
	mac2!{
		@expand[
			[3 4] [4 3]
			[[! mac1 2 1][1 2]{}]
		]
		5 6
	}
*/
/*
	check_expand!{
		[
			[4 3]
			[[! mac1 2 1][1 2]{}]
		]
		mac2Exp 3 4
	}
*/
/*
	@check_expansion[
		[[4 3][]]
		[[! mac1 2 1][1 2]{}]
	]
	mac2Exp 3 4
*/
/*
	@check_expansion[
		[[mac2Exp 4 3][]]
		[[! mac1 2 1][1 2]{}]
	]
	3 4
*/
/*
	@check_expansion[
		[[4 3 mac2Exp 4 3][]]
		[[! mac1 2 1][1 2]{}]
	]
*/
/*
	@check_expansion[
		[[3 mac2Exp 4 3][]]
		[[! mac1 2 1][1 2]{4}]
	]
*/
/*
	@check_expansion[
		[[mac2Exp 4 3][]]
		[[! mac1 2 1][1 2]{3 4}]
	]
*/
/*
	@check_expansion[
		[[4 3][]]
		[[! mac1 2 1][1 2]{mac2Exp 3 4}]
	]
*/
/*
	@check_expansion[
		[[3][]]
		[[! mac1 2 1][1 2]{4 mac2Exp 3 4}]
	]
*/
/*
	@check_expansion[
		[[][]]
		[[! mac1 2 1][1 2]{3 4 mac2Exp 3 4}]
	]
*/
/*
	@check_expansion[
		[[! mac1 2 1][1 2]{3 4 mac2Exp 3 4}]
	]
*/
/*
	mac1!{
		@expand[
			[1 2] [2 1]
		]
		3 4 mac2Exp 3 4
	}
*/
/*
	check_expand!{
		[
			[2 1]
		]
		mac1Exp 1 2
	}
*/
/*
	@check_expansion[
		[[2 1][]]
	]
	mac1Exp 1 2
*/
/*
	@check_expansion[
		[[mac1Exp 2 1][]]
	]
	1 2
*/
/*
	@check_expansion[
		[[1 mac1Exp 2 1][]]
	]
	2
*/
/*
	@check_expansion[
		[[2 1 mac1Exp 2 1][]]
	]
*/
/*
	reverse!{ [2 1 mac1Exp 2 1] }
*/
/*
	1 2 mac1Exp 1 2
*/