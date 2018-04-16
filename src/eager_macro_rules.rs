
#[macro_export]
macro_rules! eager_macro_rules{

// Start by decoding the initial values
	(
		$(#[$($metas:tt)*])*
		$macro_name:ident $dollar1:tt $id_1:ident $dollar2:tt $id_2:ident
		$($rules:tt => $expansions:tt);* $(;)*
	)=>{
		eager_macro_rules_internal!{
			@first[
				$(#[$($metas)*])*
				$macro_name $dollar1 $id_1 $dollar2 $id_2
			]
			$($rules => $expansions)*
		}
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! eager_macro_rules_internal{
// If there are no more rules, finish
	(
		@first[
			$(#[$($metas:tt)*])*
			$macro_name:ident $dollar1:tt $id_1:ident $dollar2:tt $id_2:ident
			$($prev_grammar:tt => $prev_expansion:tt)*
		]
	) => {
		eager_macro_rules_internal!{
			@final[
				$(#[$($metas)*])*
				$macro_name$dollar1 $id_1 $dollar2 $id_2
				$($prev_grammar => $prev_expansion)*
			]
		}
	};

//Handle the 3 different block type before the '=>'
	(
		@first[
			$(#[$($metas:tt)*])*
			$macro_name:ident $dollar1:tt $id_1:ident $dollar2:tt $id_2:ident
			$($prev_grammar:tt => $prev_expansion:tt)*
		]
		{$($next_grammar:tt)*} $($rest:tt)+
	) => {
		eager_macro_rules_internal!{
			@expansion[
				$(#[$($metas)*])*
				$macro_name$dollar1 $id_1 $dollar2 $id_2
				$($prev_grammar => $prev_expansion)*
				[$($next_grammar)*]
			]
			$($rest)+
		}
	};
	(
		@first[
			$(#[$($metas:tt)*])*
			$macro_name:ident $dollar1:tt $id_1:ident $dollar2:tt $id_2:ident
			$($prev_grammar:tt => $prev_expansion:tt)*
		]
		($($next_grammar:tt)*) $($rest:tt)+
	) => {
		eager_macro_rules_internal!{
			@expansion[
				$(#[$($metas)*])*
				$macro_name$dollar1 $id_1 $dollar2 $id_2
				$($prev_grammar => $prev_expansion)*
				[$($next_grammar)*]
			]
			$($rest)+
		}
	};
	(
		@first[
			$(#[$($metas:tt)*])*
			$macro_name:ident $dollar1:tt $id_1:ident $dollar2:tt $id_2:ident
			$($prev_grammar:tt => $prev_expansion:tt)*
		]
		[$($next_grammar:tt)*] $($rest:tt)+
	) => {
		eager_macro_rules_internal!{
			@expansion[
				$(#[$($metas)*])*
				$macro_name$dollar1 $id_1 $dollar2 $id_2
				$($prev_grammar => $prev_expansion)*
				[$($next_grammar)*]
			]
			$($rest)+
		}
	};
	
// Handle the 3 different block types after the '=>'
	(
		@expansion[
			$(#[$($metas:tt)*])*
			$macro_name:ident $dollar1:tt $id_1:ident $dollar2:tt $id_2:ident
			$({$($prev_grammar:tt)*} => $prev_expansion:tt)*
			[$($next_grammar:tt)*]
		]
		 => {$($next_expansion:tt)*} $($rest:tt)*
	) => {
		eager_macro_rules_internal!{
			@first[
				$(#[$($metas)*])*
				$macro_name$dollar1 $id_1 $dollar2 $id_2
				$({$($prev_grammar)*}  => $prev_expansion)*
				{$($next_grammar)*} => {$($next_expansion)*}
			]
			$($rest)*
		}
	};
	(
		@expansion[
			$(#[$($metas:tt)*])*
			$macro_name:ident $dollar1:tt $id_1:ident $dollar2:tt $id_2:ident
			$({$($prev_grammar:tt)*} => $prev_expansion:tt)*
			[$($next_grammar:tt)*]
		]
		 => ($($next_expansion:tt)*) $($rest:tt)*
	) => {
		eager_macro_rules_internal!{
			@first[
				$(#[$($metas)*])*
				$macro_name$dollar1 $id_1 $dollar2 $id_2
				$({$($prev_grammar)*}  => $prev_expansion)*
				{$($next_grammar)*} => {$($next_expansion)*}
			]
			$($rest)*
		}
	};
	(
		@expansion[
			$(#[$($metas:tt)*])*
			$macro_name:ident $dollar1:tt $id_1:ident $dollar2:tt $id_2:ident
			$({$($prev_grammar:tt)*} => $prev_expansion:tt)*
			[$($next_grammar:tt)*]
		]
		 => [$($next_expansion:tt)*] $($rest:tt)*
	) => {
		eager_macro_rules_internal!{
			@first[
				$(#[$($metas)*])*
				$macro_name$dollar1 $id_1 $dollar2 $id_2
				$({$($prev_grammar)*}  => $prev_expansion)*
				{$($next_grammar)*} => {$($next_expansion)*}
			]
			$($rest)*
		}
	};

// Output
	(	@final[
			$(#[$($metas:tt)*])*
			$macro_name:ident $dollar1:tt $id_1:ident $dollar2:tt $id_2:ident
			$({$($rules_grammar:tt)*} => {$($rules_expansion:tt)*})+
		]
	)=>{
		$(#[$($metas)*])*
		macro_rules! $macro_name{
			$(
				// First the pure version
				{$($rules_grammar)*} => {$($rules_expansion)*};
			)+
			$(
				// Then the expand supporting version
				{
					@expand[[$dollar1($dollar1 $id_1:tt)*] $dollar2($dollar2 $id_2:tt)*]
					$($rules_grammar)*
				} => {
					check_expand!{
						[$dollar2($dollar2$id_2)*]
						$($rules_expansion)*
						$dollar1($dollar1$id_1)*
					}
				};
			)+
		}
	};
}


