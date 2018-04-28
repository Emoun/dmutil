///
/// Emulates eager expansion of macros.
///
/// # Example
/// ```
/// #[macro_use]
/// extern crate dmutil;
///
/// eager_macro_rules!{
///     plus_1 $eager_1 $eager_2
///     ()=>{
///         + 1
///     };
/// }
///
/// fn main(){
/// 	assert_eq!(4, eager!{2 plus_1!() plus_1!()});
/// }
/// ```
///
/// # Usage
///
/// `eager!` can wrap any code, and if that code contains a macro call, that macro will be
/// expanded before its consumer. This means:
///
/// * If a macro call is given as an argument to another macro, the first macro will be expanded
/// first.
/// * All macros will be fully expanded before `eager!` expands, meaning otherwise illegal
/// intermediate expansion steps can be made possible.
///
/// `eager!` does not work with any macro. Only macros declared using [eager_macro_rules!] may be
/// used inside `eager!`. Such macros are said to be `eager!`-enabled.
///
/// [eager_macro_rules!]: macro.eager_macro_rules.html
///
/// # Cons
///
/// * Because of the way `eager!` is implemented; being a hack of recursive macroes, the compiler's
/// default macro recursion limit is quickly exceeded. Therefore, `#![recursion_limit="256"]`
/// must be used in most situations (potentially with a higher limit)
/// such that expansion can happen.
///
/// * Debugging an eagerly expanded macro is very difficult and requires intimate knowledge
/// of the implementation of `eager!`. There is no way to mitigate this, except to try and
/// recreate the bug without using `eager!`. Likewise, the error messages the compiler will
/// emit are exponentially more cryptic than they already would have been.
///
/// * Only works with `eager!`-enabled macros. Additionally, none of the macros may
/// expand to something containing a non-`eager!`-enabled macro, not even as an intermediate
/// expansion.
///
/// ---
/// # Macro expansions
///
/// Rust is lazy when it comes to macro expansion. When the compiler sees a macro call, it will
/// try to expand the macro without looking at its arguments or what the expansion becomes.
/// Using `eager!`, previously illegal macro expansions can be made possible.
///
/// ## Macro restrictions
/// This puts a few restrictions on what can and cannot be done with macros:
///
/// ### The arguments to a macro usually cannot be the resulting expansion of another macro call:
/// Say you have a macro that adds two numbers:
/// ```ignore
/// macro_rules! add{
///     ($e1:expr, $e2:expr)=> {$e1 + $e2}
/// }
/// ```
/// And a macro that expands to two comma-separated numbers:
///
/// ```ignore
/// macro_rules! two{
///     ()=>{2,3}
/// }
/// ```
///
/// You cannot use the expansion of `two!` as an argument to `add!`:
/// ```ignore
/// let x = add!(two!()); // error
/// ```
/// The compiler will complain about no rule in `add!` accepting `two`, since `two!()` does not
/// get expanded before the `add!` who requires two expression and not just one.
///
/// With eager expansion, this can be made possible:
/// ```
/// #[macro_use]
/// extern crate dmutil;
///
/// eager_macro_rules!{
///     add $eager_1 $eager_2
///     ($e1:expr, $e2:expr)=> {$e1 + $e2}
/// }
///
/// eager_macro_rules!{
///     two $eager_1 $eager_2
///     ()=>{2,3}
/// }
///
/// fn main(){
/// 	let x = eager!{add!(two!())};
/// 	assert_eq!(5, x);
/// }
/// ```
///
/// ### An intermediate expansion step cannot result in invalid syntax
///
/// Say you have a macro that expands to an identifier:
/// ```ignore
/// macro_rules! id{
///     ()=> {SomeStruct}
/// }
/// ```
/// And want a macro that, using the previous macro, expands to a struct declaration:
/// ```ignore
/// macro_rules! some_struct{
///     ()=> {struct id!(){}}
/// }
/// ```
/// Calling this macro will not compile. Looking at the result of the first expansion
/// step we can see why:
/// ```ignore
/// struct id!() {}
/// ```
/// Since macro calls are illegal in identifier positions, the compiler will refuse to continue
/// expanding.
///
///  With eager expansion, this can be made possible:
/// ```
/// #[macro_use]
/// extern crate dmutil;
///
/// eager_macro_rules!{
///     some_struct $eager_1 $eager_2
///     ()=> {SomeStruct}
/// }
///
/// eager_macro_rules!{
///     some_struct $eager_1 $eager_2
///     ()=>{struct id!(){}}
/// }
/// some_struct!{}
/// ```
///
/// # Trivia
///
/// Ironically, `eager!` is not itself `eager!`-enabled,
/// though it does ignore itself if it is nested.
///
///
#[macro_export]
macro_rules! eager{
	(
		$($all:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[][]]
			]
			$($all)*
		}
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! eager_internal{
	(	// From macro expansion
		@from_macro[ $prefix:tt $($rest:tt)* ]
		$($input:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[$prefix []]
				$($rest)*
			]
			$($input)*
		}
	};
// Decode input stream
	(	// If the next token is a block, check it (brace type)
		@check_expansion[
			[[$($prefix:tt)*][]]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
		{$($body:tt)*} $($rest:tt)*
	)=>{
		eager_internal!{
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
		eager_internal!{
			@check_expansion[
				[[][]]
				[[$($prefix)*][$($rest)*]()]
				$([$prefix_rest $postfix_rest $block_type])*
			]
			$($body)*
		}
	};
	(	// If the next token is a n 'eager!' macro call, ignore it,
		// extracting the body. (brace type)
		@check_expansion[
			[[$($prefix:tt)*][]]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
		eager!{$($body:tt)*} $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[$($prefix)*][]]
				$([$prefix_rest $postfix_rest $block_type])*
			]
			$($body)* $($rest)*
		}
	};
	(	// If the next token is a n 'eager!' macro call, ignore it,
		// extracting the body. (brace type)
		@check_expansion[
			[[$($prefix:tt)*][]]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
		eager!($($body:tt)*) $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[$($prefix)*][]]
				$([$prefix_rest $postfix_rest $block_type])*
			]
			$($body)* $($rest)*
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
		eager_internal!{
			@check_expansion[
				[[$next $($prefix)*][]]
				$([$prefix_rest $postfix_rest $block_type])*
			]
			$($rest)*
		}
	};
	
// Done decoding input
	(	// When there is no more input and the last input was a macro call
		// (brace type)
		@check_expansion[
			[[! $macro_name:tt $($prefix:tt)*][$($postfix:tt)*]{$($body:tt)*}]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
	)=>{
		$macro_name!{
			@eager[
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
			@eager[
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
		eager_internal!{
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
		eager_internal!{
			@check_expansion[
				[[$($last_rest)*][]]
				[$prefix $postfix ($last $($body)*)]
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
		eager_internal!{
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
		eager_internal!{
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
		eager_internal!{
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
		eager_internal!{
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
		eager_internal!{
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
		eager_internal!{
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
		eager_internal!{
			@check_expansion[
				[[$promoted $($prefix)*][]]
				$([$prefix_rest $postfix_rest $block_type_rest])*
			]
			$($postfix)*
		}
	};
}


