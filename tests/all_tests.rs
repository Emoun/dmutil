#![feature(trace_macros)] //trace_macros!(true);
#[macro_use]
extern crate dmutil;

#[test]
fn test(){
	assert!(reverse_tt!{[< 1]{2 -}|[6 -]{5 -}});
}


use std::marker::PhantomData;

macro_rules! dec_struct{
	{
		[$name:ident]
		[$($generics:tt)*]
		[$($constraints:tt)*]
		[$($body:tt)*]
	}=>{
		struct $name<$($generics)*>
		where $($constraints)*
		{$($body)*}
	};
}

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

trace_macros!{true}
mac2!(SomeStruct, PartialEq);
/* We want it to expand to:

	struct SomeStruct < V , W >
		where W : PartialEq , V : PartialEq
	{
		ph1 : PhantomData < W > ,
		ph2 : PhantomData < V >
	}
*/

