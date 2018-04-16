#![allow(dead_code)]

mod test_prefix{
	/*
	Tests that input can be followed by a macro call
	*/
	eager_macro_rules! {
		test_macro $eager_1 $eager_2
		{ ! ! } =>{
			expand!{
				struct
				test_macro!{??}
			}
		};
		{ ?? } =>{
			SomeName test_macro!{| |}
		};
		{ | | } =>{
			{field: u32}
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
					mac1!{mac3!{{SomeThing}}}
				}
			}
		};
		(	@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			{SomeThing}
		)=>{
			check_expand!{
				[$($expand_rest)*]
				!!
				$($postfix)*
			}
		};
	}
	mac3!{SomeStruct}
}
mod test_non_call_block_ignored{
	
	macro_rules! test_macro{
		() => {
			expand!{
				test_macro!{1}
				{field: i32}
				struct SomeSecondStruct{}
			}
		};
		(	@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			1
		) => {
			check_expand!{
				[$($expand_rest)*]
				struct SomeStruct
				$($postfix)*
			}
		};
	}
	test_macro!{}
}

// Same tests as above, but with the '()' block type
mod paren_test_prefix{
	/*
	Tests that input can be followed by a macro call
	*/
	macro_rules ! test_macro{
		(!! ) =>{
			expand!{
				const N: i32 = test_macro!(1);
				
			}
		};
		( 	@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			1
		) =>{
			check_expand!{
				[$($expand_rest)*]
				(5+5) test_macro!(2)
				$($postfix)*
			}
		};
		( 	@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			2
		) =>{
			check_expand!{
				[$($expand_rest)*]
				+ 1 test_macro!(3)
				$($postfix)*
			}
		};
		( 	@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			3
		) =>{
			check_expand!{
				[$($expand_rest)*]
				+ (5)
				$($postfix)*
			}
		};
	}
	test_macro!(!!);
	#[test]
	fn test(){
		assert_eq!(16, N);
	}
}
mod paren_test_postfix{
	/*
	Tests that a macro call can be followed by a macro call
	*/
	macro_rules ! test_macro{
		(!! ) =>{
			expand!{
				const N: i32 = test_macro!(1);
				
			}
		};
		( 	@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			1
		) =>{
			check_expand!{
				[$($expand_rest)*]
				test_macro!(2) (5+5)
				$($postfix)*
			}
		};
		( 	@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			2
		) =>{
			check_expand!{
				[$($expand_rest)*]
				test_macro!(3) 1 +
				$($postfix)*
			}
		};
		( 	@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			3
		) =>{
			check_expand!{
				[$($expand_rest)*]
				(5) +
				$($postfix)*
			}
		};
	}
	test_macro!(!!);
	#[test]
	fn test(){
		assert_eq!(16, N);
	}
}
mod paren_test_multiple_calls{
	/*
	Tests that multliple macro calls can be done in serial
	*/
	macro_rules ! test_macro{
		(!! ) =>{
			expand!{
				const N: i32 = test_macro!(1) + test_macro!(1) + test_macro!(1);
				
			}
		};
		( 	@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			1
		) =>{
			check_expand!{
				[$($expand_rest)*]
				(5+5)
				$($postfix)*
			}
		};
	}
	test_macro!(!!);
	#[test]
	fn test(){
		assert_eq!(30, N);
	}
}
mod paren_test_nested_calls{
	/*
	Tests that a macro call can be nested, where the input to one macro is the expansion of another.
	*/
	macro_rules ! test_macro_1 {
		(!!) =>{
			expand!{
				const N: i32 = test_macro_2!(test_macro_3!(test_macro_4!()));
			}
		};
	}
	macro_rules! test_macro_2 {
		( 	@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			$($all:tt)*
		) =>{
			check_expand!{
				[$($expand_rest)*]
				($($all)*) + 2
				$($postfix)*
			}
		};
	}
	macro_rules! test_macro_3{
		( 	@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
			$($all:tt)*
		) =>{
			check_expand!{
				[$($expand_rest)*]
				1 + ($($all)*) + 2
				$($postfix)*
			}
		};
	}
	macro_rules! test_macro_4{
		( 	@expand[
				[$($postfix:tt)*] $($expand_rest:tt)*
			]
		) =>{
			check_expand!{
				[$($expand_rest)*]
				4
				$($postfix)*
			}
		};
	}
	test_macro_1!(!!);
	#[test]
	fn test(){
		assert_eq!(9, N);
	}
}




