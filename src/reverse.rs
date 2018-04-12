#[macro_export]
///
/// A macro to reverse a stream of token trees (tt).
///
/// Given a token tree in brackets `[1 2 3]`it will reverse their order and remove
/// the parenthesis: `3 2 1`. Chaining is also possible; the first group
/// will expand first after which the second one is expanded __and__ put in front
/// of the first: `[4 3][2 1] -> 1 2 3 4`.
///
/// Given a token tree in braces `{1 2 3}` it will __not__ reverse the order
/// and just remove the brackets. Chaining also works here, and like before
/// when the second is expanded, it is put in __front__ of the previous:
/// `{3 4}{1 2} -> 1 2 3 4`.
///
/// Both the above chaining work together and for more than 2 groups. The expand
/// order is always left to right: `[8 7]{5 6}[4 3]{1 2} -> 1 2 3 4 5 6 7 8`.
///
/// the pipe `|` can be used to restrict how far to the left groups are expanded to.
/// Everything to the right of a pipe will be expanded just after the pipe:
/// `[4 3 ]{1 2}|[8 7]{4 5} -> 1 2 3 4 5 6 7 8`
/// The following is a stepwise expand with the pipes:
/// `[4 3 ]{1 2}|[8 7]{5 6} -> 3 4 {1 2}| 7 8 {5 6} -> 1 2 3 4 5 6 7 8`
///
/// ```
/// #[macro_use]
/// extern crate dmutil;
/// fn main(){
///
/// assert!(reverse_tt!([1 > 2])); // expands to '2 > 1'
///		// The following all expand to '3-1 == 2'
///
///		assert!(reverse_tt!(|{ == 2}[1 - 3]));
///		assert!(reverse_tt!(|{ == 2}[1][- 3]));
///		assert!(reverse_tt!(|{ == 2}[1][-][3]));
///		assert!(reverse_tt!(|{ == 2}{3 - 1}));
///		assert!(reverse_tt!(|{ == 2}{- 1}{3}));
///		assert!(reverse_tt!(|{2}[ == 1]{3 -}));
///		assert!(reverse_tt!(|{2}{1 == }[- 3]));
///
///		// The following expand to '1 < 2 && 3 < 4
///		assert!(reverse_tt!(|{2}[< 1]|{}[4 <]{&& 3}));
///
///		// Beware that only the order of the token trees is reversed
///		// and not the trees themselves.
///		// therefore, the following expands to '(3-1) == 2'
///		assert!(reverse_tt!(|{ == 2 }[(3-1)]));
///
///		assert!(reverse_tt!(|{2}|{>}|{1}));
///		assert!(reverse_tt!({4 > 3}));
///		assert!(reverse_tt!([< 1]{2 -}|[6 -]{5 -}));
/// }
/// ```
///
///
macro_rules! reverse_tt{
	{
		$(@done{$($prev:tt)*})* [$($all:tt)*] $($rest:tt)*
	}=>{
		reverse_tt!{
			$(@done{$($prev)*})*
			|{} [$($all)*] $($rest)*
		}
	};
	{
		$(@done{$($prev:tt)*})* |[$($all:tt)*] $($rest:tt)*
	}=>{
		reverse_tt!{
			$(@done{$($prev)*})*
			|{} [$($all)*] $($rest)*
		}
	};
	{
		{$($all:tt)*} $($rest:tt)*
	}=>{
		reverse_tt!{ |{$($all)*} $($rest)*}
	};
	{
		// Base case. A lone '|{}' is just unpacked
		|{$($first:tt)*} |{$($second:tt)*}
	}=>{
		$($all)*
	};
	{
		// Reverse
		$(@done{$($prev:tt)*})*
		|{$($reversed:tt)*} [$start:tt $($unreversed:tt)+] $($rest:tt)*
	}=>{
		reverse_tt!{
			$(@done{$($prev)*})*
			|{$start $($reversed)*} [$($unreversed)+] $($rest)*
		}
	};
	{
		// Last one in a reverse
		$(@done{$($prev:tt)*})*
		|{$($reversed:tt)*} [$last:tt] $($rest:tt)*
	}=>{
		reverse_tt!{
			$(@done{$($prev)*})*
			|{$last $($reversed)*} $($rest)*
		}
	};
	{
		// Non-reverse merge
		$(@done{$($prev:tt)*})*
		|{$($reversed:tt)*} {$($no_r:tt)*} $($rest:tt)*
	}=>{
		reverse_tt!{
			$(@done{$($prev)*})*
			|{$($no_r)* $($reversed)*} $($rest)*
		}
	};
	{
		// We done know the next '|{}' is done
		$(@done{$($prev:tt)*})* |{$($done:tt)*} | $($rest:tt)*
	}=>{
		reverse_tt!{
			$(@done{$($prev)*})* @done{$($done)*}
			| $($rest)*
		}
	};
	{
		//All done
		$(@done{$($done:tt)*})* |{$($last:tt)*}
	}=>{
		$($($done)*)* $($last)*
	};
	
}