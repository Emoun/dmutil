

#[macro_export]
macro_rules! recursive{
	{
		[$($prefix:tt)*]
		[$($postfix:tt)*]
	}=>{
		$($prefix)* $($postfix)*
	};
	{
		[$($prefix:tt)*]
		[$($postfix:tt)*]
		$($rest:tt)+
	}=>{
		recursive!{[$($prefix)* $($postfix)*] $($rest)*}
	};
    {
		[$($prefix:tt)*]
		$m:ident!{$($argument:tt)*}
		$($rest:tt)*
    }=>{
        $m!{@recursive
        	[[$($prefix)*] $($rest)*]
			$($argument)*
		}
    };
}