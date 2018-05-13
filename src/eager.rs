///
/// Emulates eager expansion of macros.
///
/// # Example
/// ```
/// #[macro_use]
/// extern crate dmutil;
///
/// eager_macro_rules!{ $eager_1
///     macro_rules! plus_1{
///         ()=>{+ 1};
///     }
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
/// * All macros will be fully expanded before `eager!` expands, meaning - otherwise - illegal
/// intermediate expansion steps are possible.
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
/// eager_macro_rules!{ $eager_1
///     macro_rules! add{
///         ($e1:expr, $e2:expr)=> {$e1 + $e2}
///     }
///
///     macro_rules! two_and_three{
///     	()=>{2,3}
///     }
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
/// eager_macro_rules!{ $eager_1
///     macro_rules! id{
///         ()=> {SomeStruct}
/// 	}
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
/// eager_macro_rules!{ $eager_1
///     macro_rules! op{
///         ( plus ) => { + };
///         ( minus ) => { - };
///     }
///
///     macro_rules! integer{
///         ( one ) => { 1 };
///         ( two ) => { 2 };
///     }
///
///     macro_rules! calculate{
///         ( $lhs:tt $op:tt $rhs:tt ) => {
///              eager!{integer!{$lhs} op!{$op} integer!{$rhs}}
///         };
/// 	}
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
/// ## Auxiliary variable
///
/// The auxiliary variable that must always be provided to `eager_macro_rules!`
/// must use the identifier `eager_1`. This makes it easier for everyone to
/// get used to its presence and ignore it. By having it be the same in every project,
/// no one has to think about why a given project uses some specific identifier.
///
/// # Trivia
///
/// * Ironically, `eager!` is not technically `eager!`-enabled. Instead, it ignores itself if
/// it is nested or a macro expands into an `eager!` block.
/// Likewise, `eager_macro_rules!` is not `eager!`-enabled, though this might be possible.
///
/// * `lazy!` is treated by `eager!` as a keyword and not a macro.
///
/// * `eager_macro_rules!`'s auxiliary variable is affectionately called `Simon`.
/// This nickname should probably not be used as the identifier in production code.
/// Before reaching production, though...
///
/// * Simon once had a brother called `Garkel`.
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
				[[][][][]]
			]
			$($all)*
		}
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! eager_internal{
	(
		@from_macro[
			[$lazy:tt $modefix:tt $prefix:tt[$($postfix:tt)*]]
			$($rest_decoded:tt)*
		]
		$($expanded:tt)*
	) => {
		eager_internal!{
			@check_expansion[
				[$lazy $modefix $prefix []]
				$($rest_decoded)*
			]
			$($expanded)* $($postfix)*
		}
	};
// Decode input stream
	(	// If the next token is a block, check it (brace type)
		@check_expansion[
			[$lazy:tt $modefix:tt [$($prefix:tt)*][]]
			$($rest_decoded:tt)*
		]
		{$($body:tt)*} $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[$lazy [][][]]
				[$lazy $modefix [$($prefix)*][$($rest)*]{}]
				$($rest_decoded)*
			]
			$($body)*
		}
	};
	(	// If the next token is a block, check it (parenthesis type)
		@check_expansion[
			[$lazy:tt $modefix:tt [$($prefix:tt)*][]]
			$($rest_decoded:tt)*
		]
		($($body:tt)*) $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[$lazy [][][]]
				[$lazy $modefix [$($prefix)*][$($rest)*]()]
				$($rest_decoded)*
			]
			$($body)*
		}
	};
// eager/lazy mode changes
	(	// If the next token is an 'eager!' macro call and we are already
		// in eager mode, ignore it, extracting the body. (brace type)
		@check_expansion[
			[[]$modefix:tt[$($prefix:tt)*][]]
			$($rest_decoded:tt)*
		]
		eager!{$($body:tt)*} $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[]$modefix[$($prefix)*][]]
				$($rest_decoded)*
			]
			$($body)* $($rest)*
		}
	};
	(	// If the next token is an 'eager!' macro call and we are already
		// in eager mode, ignore it, extracting the body. (parenthesis type)
		@check_expansion[
			[[]$modefix:tt[$($prefix:tt)*][]]
			$($rest_decoded:tt)*
		]
		eager!($($body:tt)*) $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[]$modefix[$($prefix)*][]]
				$($rest_decoded)*
			]
			$($body)* $($rest)*
		}
	};
	(	// If the next token is an 'lazy!' macro call and we are already
		// in lazy mode, ignore it, extracting the body. (brace type)
		@check_expansion[
			[[@lazy]$modefix:tt[$($prefix:tt)*][]]
			$($rest_decoded:tt)*
		]
		lazy!{$($body:tt)*} $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[@lazy]$modefix[$($prefix)*][]]
				$($rest_decoded)*
			]
			$($body)* $($rest)*
		}
	};
	(	// If the next token is an 'lazy!' macro call and we are already
		// in lazy mode, ignore it, extracting the body. (parenthesis type)
		@check_expansion[
			[[@lazy]$modefix:tt[$($prefix:tt)*][]]
			$($rest_decoded:tt)*
		]
		lazy!($($body:tt)*) $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[@lazy]$modefix[$($prefix)*][]]
				$($rest_decoded)*
			]
			$($body)* $($rest)*
		}
	};
	(	// If the next token is an 'eager!' macro call and we are
		// in lazy mode (brace type)
		@check_expansion[
			[[@lazy][][$($prefix:tt)*][]]
			$($rest_decoded:tt)*
		]
		eager!{$($body:tt)*} $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[][$($rest)*][$($prefix)*][]]
				$($rest_decoded)*
			]
			$($body)*
		}
	};
	(	// If the next token is an 'eager!' macro call and we are
		// in lazy mode, ignore it, extracting the body. (parenthesis type)
		@check_expansion[
			[[@lazy][][$($prefix:tt)*][]]
			$($rest_decoded:tt)*
		]
		eager!($($body:tt)*) $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[][$($rest)*][$($prefix)*][]]
				$($rest_decoded)*
			]
			$($body)*
		}
	};
	(	// If the next token is an 'lazy!' macro call and we are
		// in eager mode, ignore it, extracting the body. (brace type)
		@check_expansion[
			[[][][$($prefix:tt)*][]]
			$($rest_decoded:tt)*
		]
		lazy!{$($body:tt)*} $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[@lazy][$($rest)*][$($prefix)*][]]
				$($rest_decoded)*
			]
			$($body)*
		}
	};
	(	// If the next token is an 'eager!' macro call and we are already
		// in eager mode, ignore it, extracting the body. (parenthesis type)
		@check_expansion[
			[[][][$($prefix:tt)*][]]
			$($rest_decoded:tt)*
		]
		lazy!($($body:tt)*) $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[[@lazy][$($rest)*][$($prefix)*][]]
				$($rest_decoded)*
			]
			$($body)*
		}
	};
// end eager/lazy mode switches
	(	// If the next token isn't any of the above
		// it is safe to add it to the prefix
		@check_expansion[
			[$lazy:tt $modefix:tt [$($prefix:tt)*][]]
			$($rest_decoded:tt)*
		]
		$next:tt $($rest:tt)*
	)=>{
		eager_internal!{
			@check_expansion[
				[$lazy $modefix[$next $($prefix)*][]]
				$($rest_decoded)*
			]
			$($rest)*
		}
	};
// Done decoding input
// Eager macro expansions
	(	// When there is no more input, the last input was a macro call,
		// and we are in eager mode, call the macro eagerly
		// (brace type)
		@check_expansion[
			[[]$modefix:tt[! $macro_name:tt $($prefix:tt)*][$($postfix:tt)*]{$($body:tt)*}]
			$($rest_decoded:tt)*
		]
	)=>{
		$macro_name!{
			@eager[
				[[]$modefix[$($prefix)*][$($postfix)*]]
				$($rest_decoded)*
			]
			$($body)*
		}
	};
	(	// When there is no more input and the last input was a macro call,
		// and we are in eager mode, call the macro eagerly
		// (parenthesis type)
		@check_expansion[
			[[]$modefix:tt[! $macro_name:tt $($prefix:tt)*][$($postfix:tt)*]($($body:tt)*)]
			$($rest_decoded:tt)*
		]
	)=>{
		$macro_name!{
			@eager[
				[[]$modefix[$($prefix)*][$($postfix)*]]
				$($rest_decoded)*
			]
			$($body)*
		}
	};
// redecode modefixes
	(	// When there is no more input, but there is some postfix,
		// if the current mode is eager, redecode the postfix in lazy mode
		@check_expansion[
			[[][$($modefix:tt)+] $prefix:tt []]
			$($rest:tt)*
		]
	)=>{
		eager_internal!{
			@check_expansion[
				[[@lazy][] $prefix []]
				$($rest)*
			]
			$($modefix)+
		}
	};
	(	// When there is no more input, but there is some postfix,
		// if the current mode is lazy, redecode the postfix in eager mode
		@check_expansion[
			[[@lazy][$($modefix:tt)+] $prefix:tt []]
			$($rest:tt)*
		]
	)=>{
		eager_internal!{
			@check_expansion[
				[[][] $prefix []]
				$($rest)*
			]
			$($modefix)+
		}
	};
// end redecode modefixes
// Promote prefixes
	(	// When there is no more input and the last input wasn't a macro call in eager mode
		// insert it into the previous block (brace type)
		@check_expansion[
			[$lazy_0:tt $modefix_0:tt [$last:tt $($last_rest:tt)*] []]
			[$lazy:tt $modefix:tt $prefix:tt $postfix:tt {$($body:tt)*}]
			$($rest:tt)*
		]
	)=>{
		eager_internal!{
			@check_expansion[
				[$lazy_0 $modefix_0 [$($last_rest)*] []]
				[$lazy $modefix $prefix $postfix {$last $($body)*}]
				$($rest)*
			]
		}
	};
	(	// When there is no more input and the last input wasn't a macro call in eager mode
		// insert it into the previous block (parenthesis type)
		@check_expansion[
			[$lazy_0:tt $modefix_0:tt[$last:tt $($last_rest:tt)*] []]
			[$lazy:tt $modefix:tt $prefix:tt $postfix:tt ($($body:tt)*)]
			$($rest:tt)*
		]
	)=>{
		eager_internal!{
			@check_expansion[
				[$lazy_0 $modefix_0 [$($last_rest)*] []]
				[$lazy $modefix $prefix $postfix ($last $($body)*)]
				$($rest)*
			]
		}
	};
	(	// When there is no more input, prefix or postfix,
		// but there is a previous block, remove the input catcher
		@check_expansion[
			[$lazy_0:tt[][][]]
			$([$lazy:tt $modefix:tt $prefix:tt $postfix:tt $body:tt])+
			
		]
	)=>{
		eager_internal!{
			@check_expansion[
				$([$lazy $modefix $prefix $postfix $body])+
			]
		}
	};
// end promote prefixes

	(	// When there is no more input and no block
		// output the result, reversing it to ensure correct order
		@check_expansion[
			[$lazy:tt [][$($result:tt)*][]]
		]
	)=>{
		reverse_tt!{ [$($result)*] }
	};
	(	// When there is no more input but a block,
		// the block must have already been checked,
		// therefore, begin promoting to prefix (brace type)
		@check_expansion[
			[$lazy:tt $modefix:tt [$($prefix:tt)*][$($postfix:tt)*]{$($body:tt)*}]
			$($rest:tt)*
		]
	)=>{
		eager_internal!{
			@promote[
				[$lazy $modefix [{} $($prefix)*][$($postfix)*]{$($body)*}]
				$($rest)*
			]
		}
	};
	(	// When there is no more input and but a block
		// the block must have already been checked,
		// so output everything (parenthesis type)
		@check_expansion[
			[$lazy:tt $modefix:tt [$($prefix:tt)*][$($postfix:tt)*]($($body:tt)*)]
			$($rest:tt)*
		]
	)=>{
		eager_internal!{
			@promote[
				[$lazy $modefix [() $($prefix)*][$($postfix)*]($($body)*)]
				$($rest)*
			]
		}
	};

// Promoting blocks
	(	// promote a checked block to prefix (brace type)
		// We don't reverse the order in the block when
		// it is promoted since the revert_tt! called later
		// wont touch it.
		@promote[
			[$lazy:tt $modefix:tt [{$($other:tt)*} $($prefix:tt)*][$($postfix:tt)*]{$next:tt $($body:tt)*}]
			$($rest:tt)*
		]
	)=>{
		eager_internal!{
			@promote[
				[$lazy $modefix [{$($other)* $next} $($prefix)*][$($postfix)*]{$($body)*}]
				$($rest)*
			]
		}
	};
	(	// promote a checked block to prefix (paren type)
		// We don't reverse the order in the block when
		// it is promoted since the revert_tt! called later
		// wont touch it.
		@promote[
			[$lazy:tt $modefix:tt [($($other:tt)*) $($prefix:tt)*][$($postfix:tt)*]($next:tt $($body:tt)*)]
			$($rest:tt)*
		]
	)=>{
		eager_internal!{
			@promote[
				[$lazy $modefix [($($other)* $next) $($prefix)*][$($postfix)*]($($body)*)]
				$($rest)*
			]
		}
	};
	(	// done promoting a checked block to prefix (brace type)
		@promote[
			[$lazy:tt $modefix:tt [$promoted:tt $($prefix:tt)*][$($postfix:tt)*]{}]
			$($rest:tt)*
		]
	)=>{
		eager_internal!{
			@check_expansion[
				[$lazy $modefix [$promoted $($prefix)*][]]
				$($rest)*
			]
			$($postfix)*
		}
	};
	(	// done promoting a checked block to prefix (paren type)
		@promote[
			[$lazy:tt $modefix:tt [$promoted:tt $($prefix:tt)*][$($postfix:tt)*]()]
			$($rest:tt)*
		]
	)=>{
		eager_internal!{
			@check_expansion[
				[$lazy $modefix [$promoted $($prefix)*][]]
				$($rest)*
			]
			$($postfix)*
		}
	};
}


