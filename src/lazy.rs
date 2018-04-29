
///
/// Used within an [`eager!`](macro.eager.html) to revert to lazy expansion. Cannot be called directly.
///
/// This macro is only used to reserve the macro name 'lazy' that is used in `eager!`
/// to cancel eager expansion. Calling this macro outside `eager!` will fail to compile,
/// emitting an error explaining the same.
///
///
#[macro_export]
macro_rules! lazy {
	($($all:tt)*) => {compile_error!(
		"'lazy!' may only be used inside an 'eager!' call and cannot be called directly."
	)};


}
