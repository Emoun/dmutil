
mod test_lazy_block_is_ignored {
	/*
	Tests that a non-eager!-enabled macro can be used inside a 'lazy!' block
	*/
	macro_rules! lazy_macro{
		() => {1 + 1};
	}
	eager_macro_rules!{ $eager_1
		macro_rules! eager_macro{
			() => {1 + };
		}
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
/*
mod test_eager_in_lazy{
	/*
	Tests that eager! blocks are eagerly expanded even though they are inside a lazy block
	*/
	
	eager_macro_rules!{ $eager_1
		macro_rules! test_macro_1{
			()=> {1}
		}
	}
	macro_rules! test_macro_2{
		(! 1 ?) => {2};
	}
	
	#[test]
	fn test(){
		trace_macros!(true);
		x = eager!{
			1 +
			lazy!{
				test_macro_2!{
					!
					eager!{
						test_macro_1!{}
					}
					?
				}
			}
			+ 1
		};
		trace_macros!(false);
		assert_eq!(3, x);
	}
}
*/
// Same tests as above, but with the '()' block type
mod paren_test_lazy_block_is_ignored {
	/*
	Tests that a non-eager!-enabled macro can be used inside a 'lazy!' block
	*/
	macro_rules! lazy_macro{
		() => {1 + 1};
	}
	eager_macro_rules!{ $eager_1
		macro_rules! eager_macro{
			() => {1 + };
		}
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