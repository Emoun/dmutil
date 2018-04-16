
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

