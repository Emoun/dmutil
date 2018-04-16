
#[macro_export]
macro_rules! eager_macro_rules{
	(
		$(#[$($metas:tt)*])*
		$macro_name:ident $dollar1:tt $id_1:ident $dollar2:tt $id_2:ident
		$({$($rules_grammar:tt)*} => {$($rules_expansion:tt)*});+ $(;)*
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
	}
}

