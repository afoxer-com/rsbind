use errors::*;

pub(crate) trait BuildProcess {
    fn unpack(&self) -> Result<()>;
    fn gen_bridge_src(&self) -> Result<()>;
    fn build_bridge_prj(&self) -> Result<()>;
    fn copy_bridge_outputs(&self) -> Result<()>;
    fn gen_bind_code(&self) -> Result<()>;
    fn build_dest_prj(&self) -> Result<()>;
}
