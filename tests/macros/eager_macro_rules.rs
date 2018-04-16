#![allow(dead_code)]

mod test_produces_at_least_the_same{
	/*
	Test that a declared macro will work as if it was produced with 'macro_rules'
	when not called through'eager'
	*/
	eager_macro_rules!{
		test_macro $eager_var_1 $eager_var_2
		{1} =>{ 1 };
		(2) => ( 2 );
		[3] => [ 3 ];
		{4} => ( 4 );
		(5) => { 5 };
		[6] => [ 6 ];
		{7} => [ 7 ];
		(8) => { 8 };
		[9] => ( 9 );
	}
	#[test]
	fn test(){
		assert_eq!(1, test_macro!{1});
		assert_eq!(2, test_macro!{2});
		assert_eq!(3, test_macro!{3});
		assert_eq!(4, test_macro!{4});
		assert_eq!(5, test_macro!{5});
		assert_eq!(6, test_macro!{6});
		assert_eq!(7, test_macro!{7});
		assert_eq!(8, test_macro!{8});
		assert_eq!(9, test_macro!{9});
	}
}

mod test_produces_eager_macro{
	/*
	Test that a declared macro will work with eager!
	*/
	eager_macro_rules!{
		test_macro $eager_var_1 $eager_var_2
		{1} => { + 1 };
		{2} => {eager!{1 test_macro!(1)}};
		{3} => {eager!{1 test_macro!(1) test_macro!(1)}};
		{4} => {test_macro!(2) + test_macro!(2)};
	}
	#[test]
	fn test(){
		assert_eq!(2, test_macro!{2});
		assert_eq!(3, test_macro!{3});
		assert_eq!(4, test_macro!{4});
	}
}

mod test_eager_vs_non_eager_expansion_order{
	/*
	Test that the expanded macro has the eager versions of each rule first.
	This is required because the other way around may create uncallable
	macros.
	For example, if 'mac1' below is expanden to:
	
	macro_rules! mac1{
		{
			$($to_encapsulate:tt)*
		}=>{
			{$($to_encapsulate)*}
		}
		
		<and then the eager version>
	}
	
	The compiler will complain:
	
	error: macro expansion ignores token `{` and any following
	  --> tests\macros\eager.rs:89:4
	   |
	89 |             {$($to_encapsulate)*}
	
	*/
	
	eager_macro_rules! {
		mac1 $eager_1 $eager_2
		{
			$($to_encapsulate:tt)*
		}=>{
			{$($to_encapsulate)*}
		}
	}
	macro_rules! mac2{
		($some:ident)=>{
			eager!{
				struct $some
				mac1!{x: u32}
			}
		};
	}
	mac2!{SomeStruct}
}