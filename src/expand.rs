
#[macro_export]
macro_rules! expand{
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

#[macro_export]
#[doc(hidden)]
macro_rules! expand_internal{
	(	// If the next token is a block, check it (brace type)
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
	(	// If the next token is a block, check it (parenthesis type)
		@check_expansion[
			[[$($prefix:tt)*][]]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
		($($body:tt)*) $($rest:tt)*
	)=>{
		expand_internal!{
			@check_expansion[
				[[][]]
				[[$($prefix)*][$($rest)*]()]
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
	(	// When there is no more input and the last input was a macro call
		// (brace type)
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
	(	// When there is no more input and the last input was a macro call
		// (parenthesis type)
		@check_expansion[
			[[! $macro_name:tt $($prefix:tt)*][$($postfix:tt)*]($($body:tt)*)]
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
		// insert it into the previous block (brace type)
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
	(	// When there is no more input and the last input wasn't a macro call
		// insert it into the previous block (parenthesis type)
		@check_expansion[
			[[$last:tt $($last_rest:tt)*][]]
			[$prefix:tt $postfix:tt ($($body:tt)*)]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
	)=>{
		expand_internal!{
			@check_expansion[
				[[$($last_rest)*][]]
				[$prefix $postfix ($last $($body)*)]
				$([$prefix_rest $postfix_rest $block_type])*
			]
		}
	};
	/*(	// When there is no more input and the last block's body has been promoted
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
	};*/
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
	(	// When there is no more input but a block,
		// the block must have already been checked,
		// therefore, begin promoting to prefix (brace type)
		@check_expansion[
			[[$($prefix:tt)*][$($postfix:tt)*]{$($body:tt)*}]
			$([$prefix_rest:tt $postfix_rest:tt $block_type_rest:tt])*
		]
	)=>{
		expand_internal!{
			@promote[
				[[{} $($prefix)*][$($postfix)*]{$($body)*}]
				$([$prefix_rest $postfix_rest $block_type_rest])*
			]
		}
	};
	(	// When there is no more input and but a block
		// the block must have already been checked,
		// so output everything (parenthesis type)
		@check_expansion[
			[[$($prefix:tt)*][$($postfix:tt)*]($($body:tt)*)]
			$([$prefix_rest:tt $postfix_rest:tt $block_type_rest:tt])*
		]
	)=>{
		expand_internal!{
			@promote[
				[[() $($prefix)*][$($postfix)*]($($body)*)]
				$([$prefix_rest $postfix_rest $block_type_rest])*
			]
		}
	};

// Promoting blocks
	(	// promote a checked block to prefix (brace type)
		// We dont reverse the order in the block when
		// it is promoted since the revert_tt! called later
		// wont touch it.
		@promote[
			[[{$($other:tt)*} $($prefix:tt)*][$($postfix:tt)*]{$next:tt $($body:tt)*}]
			$([$prefix_rest:tt $postfix_rest:tt $block_type_rest:tt])*
		]
	)=>{
		expand_internal!{
			@promote[
				[[{$($other)* $next} $($prefix)*][$($postfix)*]{$($body)*}]
				$([$prefix_rest $postfix_rest $block_type_rest])*
			]
		}
	};
	(	// promote a checked block to prefix (paren type)
		// We dont reverse the order in the block when
		// it is promoted since the revert_tt! called later
		// wont touch it.
		@promote[
			[[($($other:tt)*) $($prefix:tt)*][$($postfix:tt)*]($next:tt $($body:tt)*)]
			$([$prefix_rest:tt $postfix_rest:tt $block_type_rest:tt])*
		]
	)=>{
		expand_internal!{
			@promote[
				[[($($other)* $next) $($prefix)*][$($postfix)*]($($body)*)]
				$([$prefix_rest $postfix_rest $block_type_rest])*
			]
		}
	};
	(	// done promoting a checked block to prefix (brace type)
		@promote[
			[[$promoted:tt $($prefix:tt)*][$($postfix:tt)*]{}]
			$([$prefix_rest:tt $postfix_rest:tt $block_type_rest:tt])*
		]
	)=>{
		expand_internal!{
			@check_expansion[
				[[$promoted $($prefix)*][]]
				$([$prefix_rest $postfix_rest $block_type_rest])*
			]
			$($postfix)*
		}
	};
	(	// done promoting a checked block to prefix (paren type)
		@promote[
			[[$promoted:tt $($prefix:tt)*][$($postfix:tt)*]()]
			$([$prefix_rest:tt $postfix_rest:tt $block_type_rest:tt])*
		]
	)=>{
		expand_internal!{
			@check_expansion[
				[[$promoted $($prefix)*][]]
				$([$prefix_rest $postfix_rest $block_type_rest])*
			]
			$($postfix)*
		}
	};
}


