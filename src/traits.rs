use crate::ftw_error::FtwError;
use crate::type_alias::{
    AppExt, CliArg, ExportArg, ExportName, FtwResult, GitUrl, LibExt, LibPrefix,
};

pub trait Processor {
    /// # Errors
    ///
    /// Will return `Err` if an error happened in the implementation
    fn process(&self) -> FtwResult;
}

pub trait Runner {
    /// # Errors
    ///
    /// Will return `Err` if an error happened in the implementation
    fn run(&mut self) -> Result<(), FtwError>;
}

pub trait ToGitUrl {
    fn to_git_url(&self) -> GitUrl;
}

pub trait ToCliArg {
    fn to_cli_arg(&self) -> CliArg;
}

pub trait ToExportArg {
    fn to_export_arg(&self) -> ExportArg;
}

pub trait ToExportName {
    fn to_export_name(&self) -> ExportName;
}

pub trait ToLibExt {
    fn to_lib_ext(&self) -> LibExt;
}

pub trait ToAppExt {
    fn to_app_ext(&self) -> AppExt;
}

pub trait ToLibPrefix {
    fn to_lib_prefix(&self) -> LibPrefix;
}
