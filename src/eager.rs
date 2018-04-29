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
/// `eager!` does not work with any macro; only macros declared using [`eager_macro_rules!`] may be
/// used. Such macros are said to be `eager!`-enabled.
///
/// To enable the use of non-`eager!`-enabled macros inside an `eager!` call,
/// a `lazy!` block can be inserted. Everything inside the `lazy!` block will be lazily expanded,
/// while everything outside it will continue to be eagerly expanded. Since, `lazy!` reverts
/// to the usual rules for macro expansion, and `eager!` block can be inserted inside the `lazy!`
/// block, to re-enable eager expansion for some subset of it.
///
/// __Note:__ A [`lazy!`] macro is provided with this crate only to mitigate name collisions.
/// The macro does not do anything if called outside an `eager!` block.
/// It will actually refuse to compile, emitting a message explaining the same.
///
/// [`eager_macro_rules!`]: macro.eager_macro_rules.html
/// [`lazy!`]: macro.lazy.html
/// # Cons
///
/// * Because of the way `eager!` is implemented - being a hack of recursive macros - the compiler's
/// default macro recursion limit is quickly exceeded. Therefore, `#![recursion_limit="256"]`
/// must be used in most situations - potentially with a higher limit -
/// such that expansion can happen.
///
/// * Debugging an eagerly expanded macro is very difficult and requires intimate knowledge
/// of the implementation of `eager!`. There is no way to mitigate this, except to try and
/// recreate the bug without using `eager!`. Likewise, the error messages the compiler will
/// emit are exponentially more cryptic than they already would have been.
///
/// * It can only eagerly expand `eager!`-enabled macros, so existing macros cannot be used
/// in new ways. The `lazy!` block alleviates this a bit,
/// by allowing the use of existing macros in it, while eager expansion can be done around them.
/// Luckily, `eager!`-enabling an existing macro should not be too much
/// trouble using [`eager_macro_rules!`].
///
/// ---
/// # Macro expansions
///
/// Rust is lazy when it comes to macro expansion. When the compiler sees a macro call, it will
/// try to expand the macro without looking at its arguments or what the expansion becomes.
/// Using `eager!`, previously illegal macro expansions can be made possible:
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
/// macro_rules! two_and_three{
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
///     two_and_three $eager_1 $eager_2
///     ()=>{2,3}
/// }
///
/// fn main(){
/// 	let x = eager!{add!(two_and_three!())};
/// 	assert_eq!(5, x);
/// }
/// ```
///
/// ### Macros are illegal in some contexts (e.g. as an identifier)
///
/// Say you have a macro that expands to an identifier:
/// ```ignore
/// macro_rules! id{
///     ()=> {SomeStruct}
/// }
/// ```
/// And want to use it to declare a struct:
/// ```ignore
/// struct id!()
/// {}
/// ```
///
/// This will not compile since macros are illegal in identifier position, and the compiler does
/// not check whether the expansion of the macro will result in valid Rust code.
///
/// With eager expansion, `id!` will expand before the `eager!` block , making it possible to use it
/// in an identifier position:
/// ```
/// #[macro_use]
/// extern crate dmutil;
///
/// eager_macro_rules!{
///     id $eager_1 $eager_2
///     ()=> {SomeStruct}
/// }
///
/// eager!{
///     struct id!(){
///         v: u32
///     }
/// }
///
/// fn main(){
/// 	let some_struct = SomeStruct{v: 4};
///     assert_eq!(4, some_struct.v);
/// }
/// ```
/// To circumvent any restriction on where macros can be used, we can therefore just wrap
/// the code surrounding the macro call with `eager!`. `eager!` must still be in a valid position,
/// but in the worst case it can be put around the whole item
/// (struct, trait, implement, function, etc.).
///
///
/// ### No intermediate expansion step can include invalid syntax
///
/// Say we want to create a macro that interprets words, converting them into an expression.
/// We start by declaring a macro that interprets operators:
/// ```ignore
/// macro_rules! op{
///     ( plus ) => { + };
///     ( minus ) => { - };
/// }
/// ```
///
/// We then declare a macro that interprets integers from words:
/// ```ignore
/// macro_rules! integer{
///     ( one ) => { 1 };
///     ( two ) => { 2 };
/// }
/// ```
///
/// Lastly, we declare the top-level macro that uses the previous two macros to
/// expand into an expression:
/// ```ignore
/// macro_rules! calculate{
///     ( $lhs:tt $op:tt $rhs:tt ) => {
///          integer!{$lhs} op!{$op} integer!{$rhs}
///     };
/// }
/// ```
///
/// Using this macro will not compile:
/// ```ignore
/// let x = calculate!(one plus two); //Error
/// ```
///
/// Looking at the first expansion step:
/// ```ignore
/// let x = integer!(one) op!{plus} integer!(two); //Error
/// ```
/// We can see that three macro calls in a sequence are not a valid expression.
///
/// We can use `eager!` to circumvent this restriction, by having `calculate!` expand
/// into an `eager!` block:
///
/// ```
/// #[macro_use]
/// extern crate dmutil;
///
/// eager_macro_rules! {
///     op $eager_1 $eager_2
///     ( plus ) => { + };
///     ( minus ) => { - };
/// }
/// eager_macro_rules! {
///     integer $eager_1 $eager_2
///     ( one ) => { 1 };
///     ( two ) => { 2 };
/// }
/// eager_macro_rules! {
///     calculate $eager_1 $eager_2
///     ( $lhs:tt $op:tt $rhs:tt ) => {
///          eager!{integer!{$lhs} op!{$op} integer!{$rhs}}
///     };
/// }
///
/// fn main(){
/// 	let x = calculate!(one plus two);
/// 	assert_eq!(3, x);
/// }
/// ```
/// In this case, `calculate!` does not actually have to be `eager!`-enabled, since it is not inserted
/// into an `eager!` block. Though - as per the [conventions](#conventions) - we do enable it such
/// that others may later use it inside an `eager!` block.
///
///
/// # Conventions
///
/// Since we expect the use of this macro to be broadly applicable, we propose the following
/// conventions for the Rust community to use, to ease interoperability.
///
/// ## Documentation
///
/// To make it clearly visible that a given macro is `eager!`-enabled, its short rustdoc description
/// must start with a pair of brackets, within which a link to the official `eager!` macro documentation
/// must be provided. The link's visible text must be 'eager!' and
/// the brackets must not be part of the link.
///
/// See the [`reverse_tt!`](macro.reverse_tt.html) documentation for an example.
///
/// ## Auxiliary variables
///
/// The two auxiliary variables that must always be provided to `eager_macro_rules!`
/// must use the identifiers `eager_1` and `eager_2`, in that order. This makes it easier for everyone to
/// get used to their presence and ignore them. By having them be the same in every project,
/// no one has to think about why a given project uses some specific identifiers.
///
///
///
/// # Trivia
///
/// * Ironically, `eager!` is not technically `eager!`-enabled. Instead, it ignores itself if
/// it is nested or a macro expands into an `eager!` block.
/// Likewise, `eager_macro_rules!` is not `eager!`-enabled, though this might be possible.
///
/// * `eager_macro_rules!`'s two auxiliary variables are affectionately called `Simon & Garkel`,
/// `eager_1` being `Simon` and `eager_2` being `Garkel`. These nicknames should probably not be
/// used as identifiers in production code. Before reaching production, though...
///
/// * It requires continuous effort from [Emoun](http://github.com/Emoun) to not
/// forcibly rename `eager_macro_rules!` to `eager_macros_rule`.
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
	(	// If the next token is an 'eager!' macro call, ignore it,
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
	(	// If the next token is an 'eager!' macro call, ignore it,
		// extracting the body. (parenthesis type)
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
	(	// If the next token is a 'lazy!' macro call, ignore it,
		// extracting its body (one at a time) such that it does not get eagerly expanded.
		// (brace type)
		@check_expansion[
			[[$($prefix:tt)*][]]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
		lazy!{$body_first:tt $($body_rest:tt)*} $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[$body_first $($prefix)*][]]
				$([$prefix_rest $postfix_rest $block_type])*
			]
			lazy!{$($body_rest)*} $($rest)*
		}
	};
	(	// If the next token is a 'lazy!' macro call, ignore it,
		// extracting its body (one at a time) such that it does not get eagerly expanded.
		// (paren type)
		@check_expansion[
			[[$($prefix:tt)*][]]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
		lazy!($body_first:tt $($body_rest:tt)*) $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[$body_first $($prefix)*][]]
				$([$prefix_rest $postfix_rest $block_type])*
			]
			lazy!($($body_rest)*) $($rest)*
		}
	};
	(	// If the next token is an empty 'lazy!' macro call, ignore it. (brace type)
		@check_expansion[
			[[$($prefix:tt)*][]]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
		lazy!{} $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[$($prefix)*][]]
				$([$prefix_rest $postfix_rest $block_type])*
			]
			$($rest)*
		}
	};
	(	// If the next token is an empty 'lazy!' macro call, ignore it. (paren type)
		@check_expansion[
			[[$($prefix:tt)*][]]
			$([$prefix_rest:tt $postfix_rest:tt $block_type:tt])*
		]
		lazy!() $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[$($prefix)*][]]
				$([$prefix_rest $postfix_rest $block_type])*
			]
			$($rest)*
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


