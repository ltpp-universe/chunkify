use crate::*;

pub type ChunkStrategyResult = Result<(), ChunkStrategyError>;

pub struct ChunkStrategy<'a> {
    pub(crate) upload_dir: &'a str,
    pub(crate) file_name_func: Box<dyn ChunkNaming<'a>>,
}
