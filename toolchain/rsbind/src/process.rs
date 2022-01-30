use errors::*;

pub(crate) trait BuildProcess {
    fn unpack(&self) -> Result<()>;
    fn gen_bridge_src(&self) -> Result<()>;
    fn build_bridge_prj(&self) -> Result<()>;
    fn copy_bridge_outputs(&self) -> Result<()>;
    fn gen_artifact_code(&self) -> Result<()>;
    fn build_artifact_prj(&self) -> Result<()>;
}
