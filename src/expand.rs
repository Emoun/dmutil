
#[macro_export]
macro_rules! expand{
	()=>{};
	(
		$($all:tt)*
	)=>{
		expand_internal!{
			@check_expansion[
				[[][]]
			]
			$($all)*
		}
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! expand_internal{

	(	// If the next tokens are a macro call. Call the macro
		// to handle its body
		[$($safe:tt)*] $macro_call:ident ! {$($body:tt)*} $($rest:tt)*
	)=>{
		$macro_call!{ @expand [[$($safe)*][$($rest)*]] $($body)* }
	};
	
	
	(	// If the next tokens isn't any of the above, it is safe to output.
		[$($safe:tt)*] $otherwise:tt $($rest:tt)*
	)=>{
		expand_internal!{ [$($safe)* $otherwise] $($rest)* }
	};
	(	// When there are no more token, you are done.
		[$($safe:tt)*]
	)=>{
		$($safe)*
	};
	(
		@expand_block
		[$($prefix:tt)*][$($postfix:tt)*]
		{}
	)=>{
		expand!{ $($prefix)* @expanded_block{} $($postfix)*}
	};
// ------------------------------------------------------------------------------------------------
	
	(	// If the next token is a braced block
		// check it
		@check_expansion[
			[[$($prefix:tt)*][]]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
		{$($body:tt)*} $($rest:tt)*
	)=>{
		expand_internal!{
			@check_expansion[
				[[][]]
				[[$($prefix)*][$($rest)*]{}]
				$([$prefix_rest $postfix_rest $block_type])*
			]
			$($body)*
		}
	};
	(	// If the next token isn't any of the above
		// it is safe to add it to the prefix
		@check_expansion[
			[[$($prefix:tt)*][]]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
		$next:tt $($rest:tt)*
	)=>{
		expand_internal!{
			@check_expansion[
				[[$next $($prefix)*][]]
				$([$prefix_rest $postfix_rest $block_type])*
			]
			$($rest)*
		}
	};
	
// Done ready input
	(	// When there is no more input and the last input was a macro call (using braces)
		//
		@check_expansion[
			[[! $macro_name:tt $($prefix:tt)*][$($postfix:tt)*]{$($body:tt)*}]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
	)=>{
		$macro_name!{
			@expand[
				 [$($postfix)*] [$($prefix)*]
				 $([$prefix_rest $postfix_rest $block_type])*
			]
			$($body)*
		}
	};
	(	// When there is no more input and the last input wasn't a macro call
		// insert it into the previous block (braces in this case)
		@check_expansion[
			[[$last:tt $($last_rest:tt)*][]]
			[$prefix:tt $postfix:tt {$($body:tt)*}]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
	)=>{
		expand_internal!{
			@check_expansion[
				[[$($last_rest)*][]]
				[$prefix $postfix {$last $($body)*}]
				$([$prefix_rest $postfix_rest $block_type])*
			]
		}
	};
	(	// When there is no more input and the last block's body has been promoted
		// remote it
		@check_expansion[
			[[][]$block_type_remove:tt]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
	)=>{
		expand_internal!{
			@check_expansion[
				$([$prefix_rest $postfix_rest $block_type])*
			]
		}
	};
	(	// When all input has been promoted to the previous block
		// remove the input catcher
		@check_expansion[
			[[][]]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])+
		]
	)=>{
		expand_internal!{
			@check_expansion[
				$([$prefix_rest $postfix_rest $block_type])+
			]
		}
	};
	(	// When there is no more input and no block
		// output the result, reversing it to ensure correct order
		@check_expansion[
			[[$($result:tt)*][]]
		]
	)=>{
		reverse_tt!{ [$($result)*] }
	};
	(	// When there is no more input and but a block
		// the block must have already been checked,
		// so output everything (brace type)
		@check_expansion[
			[[$($prefix:tt)*][$($postfix:tt)*]{$($body:tt)*}]
		]
	)=>{
		reverse_tt!{ [$($prefix)*]|{$($postfix)*}|{{$($body)*}} }
	};
}

#[macro_export]
macro_rules! check_expand{
	(
		[ $prefix:tt $($rest:tt)* ]
		$($input:tt)*
	)=>{
		expand_internal!{
			@check_expansion[
				[$prefix []]
				$($rest)*
			]
			$($input)*
		}
	};
}

