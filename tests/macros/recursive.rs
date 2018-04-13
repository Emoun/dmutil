
mod test_prefix{
	macro_rules ! test_macro{
		( ! ! ) =>{
			recursive !{
				[ struct ]
				test_macro!{? ?}
				[]
			}
		};
		( @ recursive [[ $ ( $ prefix: tt) * ] $ ( $ postfix:tt) * ] ? ? ) =>{
			recursive !{
				[ $ ( $ prefix) * SomeName]
				test_macro!{| |}
				$ ( $ postfix) *
			}
		};
		( @ recursive [[ $ ( $ prefix: tt) * ] $ ( $ postfix:tt) * ] | | ) =>{
			recursive !{
				[ $ ( $ prefix) * {field: u32}]
				$ ( $ postfix) *
			}
		};
	}
	test_macro!(!!);
}
mod test_postfix{
	macro_rules! test_macro{
		(!!)=>{
			recursive!{
				[struct]
				test_macro!{??}
				[]
			}
		};
		(@recursive [[$($prefix:tt)*] [$($postfix:tt)*]] ??)=>{
			recursive!{
				[$($prefix)*]
				test_macro!{||}
				[ {field: u32} $($postfix)*]
			}
		};
		(@recursive [[$($prefix:tt)*] $($postfix:tt)*] ||)=>{
			recursive!{
				[$($prefix)* SomeStruct]
				$($postfix)*
			}
		};
	}
	test_macro!(!!);
}
mod test_multiple_calls{
	use std::marker::PhantomData;
	macro_rules! mac1{
		{
			@recursive [[$($prefix:tt)*] $($postfix:tt)*]
			$typ:tt
		}=>{
			recursive!{[$($prefix)* $typ] $($postfix)*}
		};
	}
	macro_rules! mac2{
		($V:ident,$eq:tt)=>{
			recursive!{
				[struct $V<V,W> where W:]
				mac1!{$eq}
				[, V:]
				mac1!{$eq}
				[{ph1: PhantomData<W>,ph2: PhantomData<V>}]
			}
		}
	}
	
	mac2!(SomeStruct, PartialEq);
}



