#![allow(dead_code)]

mod test_prefix{
	/*
	Tests that input can be followed by a macro call
	*/
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
	/*
	Tests that a macro call can be precede some non-macro tokens.
	*/
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
	/*
	Test that multiple macro call can be done after each other
	*/
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
	mac2!(SomeStruct, PartialEq);
}
mod test_nested_calls{
	/*
	Tests that macro call can be nested, i.e. one macro's expansion is the input
	to another macro.
	*/
	use std::marker::PhantomData;
	macro_rules! mac1{
		(
			@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			!!
		)=>{
			check_expand!{
				[$($expand_rest)*]
				ph1: PhantomData<W>,ph2: PhantomData<V>
				$($postfix)*
			}
		};
	}
	macro_rules! mac2{
		{
			@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			$($to_encapsulate:tt)*
		}=>{
			check_expand!{
				[$($expand_rest)*]
				{$($to_encapsulate)*}
				$($postfix)*
			}
		}
	}
	macro_rules! mac3{
		($some:ident)=>{
			expand!{
				struct $some<V,W>
				mac2!{
					mac1!{!!}
				}
			}
		};
	}
	mac3!{SomeStruct}
}