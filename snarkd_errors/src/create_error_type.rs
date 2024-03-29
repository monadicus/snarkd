#[macro_export]
macro_rules! CreateErrorType {
    (@step) => {

    };
    (@step ($(#[$doc:meta])* unwrapped, $fn_name:ident, $($arg_name:ident,)*, $($err_msg:expr,)*, $($suggestion:expr,)*), $(($(#[$docs:meta])* $error_type:ident, $fn_names:ident, $($arg_names:ident,)*, $($err_msgs:expr,)*, $($suggestions:expr,)*),)* ) => {
        $(#[$doc])*
        #[track_caller]
        pub fn $fn_name($($arg_name: impl core::fmt::Display,)*) -> error_stack::Report<Self>
        {
            error_stack::Report::new(Self)
                $(.attach_printable(super::ErrorMsg::from(format!($err_msg))))*
                $(.attach_printable(super::Suggestion::from(format!($suggestion))))*
        }

        CreateErrorType!(@step $(($(#[$docs])* $error_type, $fn_names, $($arg_names,)*, $($err_msgs,)*, $($suggestions,)*),)*);
    };
    (@step ($(#[$doc:meta])* wrapped, $fn_name:ident, $($arg_name:ident,)*, $($err_msg:expr,)*, $($suggestion:expr,)*), $(($(#[$docs:meta])* $error_type:ident, $fn_names:ident, $($arg_names:ident,)*, $($err_msgs:expr,)*, $($suggestions:expr,)*),)* ) => {
        $(#[$doc])*
        #[track_caller]
        pub fn $fn_name<C>($($arg_name: impl core::fmt::Display,)*) -> super::Result<C>
        {
            Err(
                error_stack::Report::new(Self)
                    $(.attach_printable(super::ErrorMsg::from(format!($err_msg))))*
                    $(.attach_printable(super::Suggestion::from(format!($suggestion))))*
            )?
        }

        CreateErrorType!(@step $(($(#[$docs])* $error_type, $fn_names, $($arg_names,)*, $($err_msgs,)*, $($suggestions,)*),)*);
    };
    (@step ($(#[$doc:meta])* from_error, $fn_name:ident, $($arg_name:ident,)*, $($err_msg:expr,)*, $($suggestion:expr,)*), $(($(#[$docs:meta])* $error_type:ident, $fn_names:ident, $($arg_names:ident,)*, $($err_msgs:expr,)*, $($suggestions:expr,)*),)* ) => {
        $(#[$doc])*
        #[track_caller]
        pub fn $fn_name<F, C>(into_report: F, $($arg_name: impl core::fmt::Display,)*) -> super::Result<C>
        where
            F: error_stack::IntoReport<Ok = C>,
        {
            use error_stack::ResultExt;

            Ok(
                into_report
                    .into_report()
                    .change_context(Self)
                    $(.attach_printable(super::ErrorMsg::from(format!($err_msg))))*
                    $(.attach_printable(super::Suggestion::from(format!($suggestion))))*
                    ?
            )
        }

        CreateErrorType!(@step $(($(#[$docs])* $error_type, $fn_names, $($arg_names,)*, $($err_msgs,)*, $($suggestions,)*),)*);
    };
    ($struct_name:ident $($(#[$docs:meta])* $error_type:ident $fn_names:ident { args: ($($arg_names:ident$(,)?)*), error_msgs: [$($err_msgs:expr$(,)?)*], suggestions: [$($suggestions:expr$(,)?)*], })* ) => {
        use colored::Colorize;

        #[derive(Debug, Default, thiserror::Error)]
        pub struct $struct_name;

        impl core::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let error_ident = stringify!($struct_name);
                let (name, error) = error_ident.split_at(error_ident.len() - 5);
                write!(f, "{} {}:", name.bold().red(), error.bold().red())
            }
        }

        impl From<error_stack::Report<$struct_name>> for super::Error {
            fn from(r: error_stack::Report<$struct_name>) -> Self {
                Self::$struct_name(r)
            }
        }

        impl $struct_name {
            CreateErrorType!(@step $(($(#[$docs])* $error_type, $fn_names, $($arg_names,)*, $($err_msgs,)*, $($suggestions,)*),)*);
        }
    };

}
