
mod test_lazy_block_is_ignored {
	/*
	Tests that a non-eager!-enabled macro can be used inside a 'lazy!' block
	*/
	macro_rules! lazy_macro{
		() => {1 + 1};
	}
	eager_macro_rules!{
		eager_macro $eager_1 $eager_2
		() => {1 + };
	}
	
	#[test]
	fn test(){
		let x = eager!{
			eager_macro!{}
			lazy!{
				lazy_macro!{}
			}
		};
		assert_eq!(3, x)
	}
}

// Same tests as above, but with the '()' block type
mod paren_test_lazy_block_is_ignored {
	/*
	Tests that a non-eager!-enabled macro can be used inside a 'lazy!' block
	*/
	macro_rules! lazy_macro{
		() => {1 + 1};
	}
	eager_macro_rules!{
		eager_macro $eager_1 $eager_2
		() => {1 + };
	}
	
	#[test]
	fn test(){
		let x = eager!{
			eager_macro!()
			lazy!(
				lazy_macro!()
			)
		};
		assert_eq!(3, x)
	}
}