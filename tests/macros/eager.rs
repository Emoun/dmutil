#![allow(dead_code)]

mod test_prefix{
	/*
	Tests that input can be followed by a macro call
	*/
	eager_macro_rules! {
		test_macro $eager_1 $eager_2
		{ ! ! } =>{
			eager!{
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
	eager_macro_rules! {
		test_macro $eager_1 $eager_2
		(!!)=>{
			eager!{
				struct
				test_macro!{??}
			}
		};
		(??)=>{
			test_macro!{||}
			{field: u32}
		};
		(||)=>{
			SomeStruct
		};
	}
	test_macro!(!!);
}
mod test_multiple_calls{
	/*
	Test that multiple macro call can be done after each other
	*/
	use std::marker::PhantomData;
	eager_macro_rules! {
		mac1 $eager_1 $eager_2
		{
			$typ:tt
		}=>{
			$typ
		};
	}
	macro_rules! mac2{
		($V:ident,$eq:tt)=>{
			eager!{
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
	eager_macro_rules! {
		mac1 $eager_1 $eager_2
		(!!)=>{
			ph1: PhantomData<W>,ph2: PhantomData<V>
		};
	}
	eager_macro_rules! {
		mac2 $eager_1 $eager_2
		{
			$($to_encapsulate:tt)*
		}=>{
			{$($to_encapsulate)*}
		}
	}
	eager_macro_rules! {
		mac3 $eager_1 $eager_2
		($some:ident)=>{
			eager!{
				struct $some<V,W>
				mac2!{
					mac1!{mac3!{{SomeThing}}}
				}
			}
		};
		(
			{SomeThing}
		)=>{
			!!
		};
	}
	mac3!{SomeStruct}
}
mod test_non_call_block_ignored{
	
	eager_macro_rules! {
		test_macro $eager_1 $eager_2
		() => {
			eager!{
				test_macro!{1}
				{field: i32}
				struct SomeSecondStruct{}
			}
		};
		( 1 ) => {
			struct SomeStruct
		};
	}
	test_macro!{}
}

// Same tests as above, but with the '()' block type
mod paren_test_prefix{
	/*
	Tests that input can be followed by a macro call
	*/
	eager_macro_rules! {
		test_macro $eager_1 $eager_2
		(!! ) =>{
			eager!{
				const N: i32 = test_macro!(1);
			}
		};
		( 1 ) =>{
			(5+5) test_macro!(2)
		};
		( 2	) =>{
			+ 1 test_macro!(3)
		};
		( 3 ) =>{
			+ (5)
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
	eager_macro_rules! {
		test_macro $eager_1 $eager_2
		(!! ) =>{
			eager!{
				const N: i32 = test_macro!(1);
			}
		};
		( 1 ) =>{
			test_macro!(2) (5+5)
		};
		( 2 ) =>{
			test_macro!(3) 1 +
		};
		( 3 ) =>{
			(5) +
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
	eager_macro_rules! {
		test_macro $eager_1 $eager_2
		(!! ) =>{
			eager!{
				const N: i32 = test_macro!(1) + test_macro!(1) + test_macro!(1);
			}
		};
		( 1 ) =>{
			(5+5)
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
			eager!{
				const N: i32 = test_macro_2!(test_macro_3!(test_macro_4!()));
			}
		};
	}
	eager_macro_rules! {
		test_macro_2 $eager_1 $eager_2
		( $($all:tt)* ) =>{
			($($all)*) + 2
		};
	}
	eager_macro_rules! {
		test_macro_3 $eager_1 $eager_2
		( $($all:tt)* ) =>{
			1 + ($($all)*) + 2
		};
	}
	eager_macro_rules! {
		test_macro_4 $eager_1 $eager_2
		( ) =>{
			4
		};
	}
	test_macro_1!(!!);
	#[test]
	fn test(){
		assert_eq!(9, N);
	}
}




