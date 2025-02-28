use color_eyre::eyre::Context;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = super::super::SetContext)]
#[interactive_clap(output_context = TextDataFileContext)]
pub struct TextDataFile {
    /// Enter the path to the data file:
    path: near_cli_rs::types::path_buf::PathBuf,
    #[interactive_clap(named_arg)]
    /// Specify signer account ID
    sign_as: super::super::sign_as::Signer,
}

#[derive(Clone)]
pub struct TextDataFileContext(super::DataContext);

impl TextDataFileContext {
    pub fn from_previous_context(
        previous_context: super::super::SetContext,
        scope: &<TextDataFile as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let data = std::fs::read_to_string(&scope.path.0)
            .wrap_err_with(|| format!("Access to data file <{:?}> not found!", scope.path))?;
        let value = serde_json::Value::String(data);
        Ok(Self(super::DataContext {
            config: previous_context.config,
            set_to_account_id: previous_context.set_to_account_id,
            key: previous_context.key,
            value,
        }))
    }
}

impl From<TextDataFileContext> for super::DataContext {
    fn from(item: TextDataFileContext) -> Self {
        item.0
    }
}
