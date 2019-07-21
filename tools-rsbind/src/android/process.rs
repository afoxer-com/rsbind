use ast::AstResult;
use bridge::prj::Unpack;
use bridges::BridgeGen::JavaGen;
use android::dest::JavaCodeGen;
use errors::ErrorKind::*;
use errors::*;
use fs_extra;
use fs_extra::dir::CopyOptions;
use process::BuildProcess;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use unzip;
use super::config::Android;

const MAGIC_NUM: &'static str = "*521%";

pub(crate) struct AndroidProcess<'a> {
    origin_prj_path: &'a PathBuf,
    dest_prj_path: &'a PathBuf,
    bridge_prj_path: &'a PathBuf,
    ast_path: &'a PathBuf,
    bin_path: &'a PathBuf,
    host_crate_name: &'a str,
    ast_result: &'a AstResult,
    config: Option<Android>,
    ast: &'a AstResult
}

impl<'a> AndroidProcess<'a> {
    pub fn new(
        origin_prj_path: &'a PathBuf,
        dest_prj_path: &'a PathBuf,
        bridge_prj_path: &'a PathBuf,
        ast_path: &'a PathBuf,
        bin_path: &'a PathBuf,
        host_crate_name: &'a str,
        ast_result: &'a AstResult,
        config: Option<Android>,
        ast: &'a AstResult
    ) -> Self {
        AndroidProcess {
            origin_prj_path,
            dest_prj_path,
            bridge_prj_path,
            ast_path,
            bin_path,
            host_crate_name,
            ast_result,
            config,
            ast
        }
    }
}

impl<'a> AndroidProcess<'a> {
    fn lib_name(&self) -> String {
        format!(
            "lib{}_android_bridge_prj.so",
            &self.host_crate_name.replace("-", "_")
        )
    }
    
    fn config(&self) -> Android {
        match self.config {
            Some(ref config) => config.to_owned(),
            None => Android::default()
        }
    }
}

impl<'a> BuildProcess for AndroidProcess<'a> {
    fn unpack(&self) -> Result<()> {
        Ok(())
    }

    fn gen_bridge_src(&self) -> Result<()> {
        println!("begin unzip rust template for android");
        // unpack the bridge project.
        {
            let buf: &[u8] = include_bytes!("res/template_bridge_android.zip");
            let unpack = Unpack {
                path: self.bridge_prj_path,
                host_crate: self.host_crate_name,
                buf,
                features: &self.config().features(),
            };

            unpack.unpack()?;
        }

        let bridge_c_src_path = self.bridge_prj_path.join("src").join("java").join("bridge");
        fs::create_dir_all(&bridge_c_src_path)?;
        JavaGen(
            self.host_crate_name.to_owned(),
            self.ast_result,
            &bridge_c_src_path,
            self.config().namespace(),
        )
        .gen_bridges()
        .unwrap();

        let _ = Command::new("cargo")
            .arg("fmt")
            .current_dir(&self.bridge_prj_path)
            .output();

        Ok(())
    }

    fn build_bridge_prj(&self) -> Result<()> {
        println!("building android bridge project");

        let phone_archs = self.config().phone_archs();
        let mut build_cmds = String::from("true");
        for phone_arch in phone_archs.iter() {
            let tmp = format!(
                "cargo rustc --target {}  --lib {} --target-dir {} {}",
                phone_arch,
                self.config().release_str(),
                "target",
                &self.config().rustc_param()
            );
            build_cmds = format!("{} && {}", &build_cmds, &tmp);
        }

        let phone64_archs = self.config().phone64_archs();
        for phone64_arch in phone64_archs.iter() {
            let tmp = format!(
                "cargo rustc --target {}  --lib {} --target-dir {} {}",
                phone64_arch,
                self.config().release_str(),
                "target",
                &self.config().rustc_param()
            );
            build_cmds = format!("{} && {}", &build_cmds, &tmp);
        }

        let x86_archs = self.config().x86_archs();
        for x86_arch in x86_archs.iter() {
            let tmp = format!(
                "cargo rustc --target {}  --lib {} --target-dir {} {}",
                x86_arch,
                self.config().release_str(),
                "target",
                &self.config().rustc_param()
            );
            build_cmds = format!("{} && {}", &build_cmds, &tmp);
        }

        let debug_release = if self.config().is_release() {
            "release"
        } else {
            "debug"
        };

        let mut strip_cmds = String::from("true");
        for phone_arch in phone_archs.iter() {
            let tmp = format!(
                "arm-linux-androideabi-strip -s target/{}/{}/{}",
                phone_arch,
                debug_release,
                self.lib_name()
            );
            strip_cmds = format!("{} && {}", &strip_cmds, &tmp);
        }

        for phone64_arch in phone64_archs.iter() {
            let tmp = format!(
                "aarch64-linux-android-strip -s target/{}/{}/{}",
                phone64_arch,
                debug_release,
                self.lib_name()
            );
            strip_cmds = format!("{} && {}", &strip_cmds, &tmp);
        }

        let cmds = format!("{} && {}", &build_cmds, &strip_cmds);

        println!("run building => {}", &cmds);

        let output = Command::new("sh")
            .arg("-c")
            .arg(cmds)
            .current_dir(self.bridge_prj_path)
            .output()?;

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;

        if !output.status.success() {
            return Err(
                CommandError(format!("run build android rust project build failed. ")).into(),
            );
        }

        Ok(())
    }

    fn copy_bridge_outputs(&self) -> Result<()> {
        let mut copy_map = HashMap::new();
        copy_map.insert("arm-linux-androideabi", "armeabi");
        copy_map.insert("armv7-linux-androideabi", "armeabi-v7a");
        copy_map.insert("aarch64-linux-android", "arm64-v8a");
        copy_map.insert("i686-linux-android", "x86");

        let options = CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 1024,
            copy_inside: true,
            depth: 65535,
        };

        println!("copy output files to android project.");

        let debug_release = if self.config().is_release() {
            "release"
        } else {
            "debug"
        };

        for entry in copy_map.iter() {
            let armeabi_src = self
                .bridge_prj_path
                .join("target")
                .join(entry.0)
                .join(debug_release)
                .join(&self.lib_name());

            if !armeabi_src.exists() {
                continue;
            }

            let armeabi_dest = self
                .dest_prj_path
                .join("rustlib")
                .join("src")
                .join("main")
                .join("jniLibs")
                .join(entry.1);
            fs_extra::copy_items(&vec![armeabi_src], &armeabi_dest, &options)
                .map_err(|e| FileError(format!("copy android bridge outputs failed. {:?}", e)))?;
            fs::rename(
                &armeabi_dest.join(&self.lib_name()),
                &armeabi_dest.join(format!("lib{}.so", &self.config().so_name())),
            )?;
        }

        Ok(())
    }

    fn gen_bind_code(&self) -> Result<()> {
        // unpack the dest java project
        {
            println!("begin unzip android template");
            if self.dest_prj_path.exists() {
                fs::remove_dir_all(&self.dest_prj_path).unwrap();
            }
            fs::create_dir_all(&self.dest_prj_path).unwrap();
            let android_template_buf: &[u8] = include_bytes!("res/template_android.zip");
            unzip::unzip_to(android_template_buf, &self.dest_prj_path).unwrap();

            let manifest_path = self
                .dest_prj_path
                .join("rustlib")
                .join("src")
                .join("main")
                .join("AndroidManifest.xml");
            let manifest_text = fs::read_to_string(&manifest_path).map_err(|e| {
                FileError(format!(
                    "read android dest project AndroidManifest.xml error: {:?}",
                    e
                ))
            })?;
            let replaced =
                manifest_text.replace(&format!("$({}-namespace)", MAGIC_NUM), &self.config().namespace());
            fs::write(manifest_path, replaced).map_err(|e| {
                FileError(format!(
                    "write android dest project AndroidManifest  error {:?}",
                    e
                ))
            })?;
        }

        println!("generate java code.");
        let parent = self
            .dest_prj_path
            .parent()
            .ok_or(FileError("can't find parent dir for java".to_string()))?;
        let java_gen_path = parent.join("java_gen");
        if java_gen_path.exists() {
            fs::remove_dir_all(&java_gen_path)?;
        }
        fs::create_dir_all(&java_gen_path)?;
        
        JavaCodeGen{
            origin_prj: self.origin_prj_path,
            java_gen_dir: &java_gen_path,
            ast: &self.ast_result,
            namespace: self.config().namespace(),
            so_name: self.config().so_name(),
            ext_libs: self.config().ext_libs()
        }.gen_java_code()?;

        // get the output dir string
        println!("get output dir string");
        let mut output_dir = self.dest_prj_path
            .join("rustlib")
            .join("src")
            .join("main")
            .join("java");
        if output_dir.exists() {
            fs::remove_dir_all(&output_dir).unwrap();
        }
        fs::create_dir_all(&output_dir).unwrap();

        let namespace = self.config().namespace();
        let pkg_split = namespace.split(".").collect::<Vec<&str>>();
        for pkg_part in pkg_split.iter() {
           output_dir = output_dir.join(pkg_part);
        }

        let options = CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 1024,
            copy_inside: true,
            depth: 65535,
        };

        fs_extra::copy_items(&vec![java_gen_path], &output_dir, &options)
            .map_err(|e| FileError(format!("copy android bridge outputs failed. {:?}", e)))
            .unwrap();

        Ok(())
    }

    fn build_dest_prj(&self) -> Result<()> {
        println!("build java dest project.");

        let build_cmd = format!("chmod a+x ./gradlew && ./gradlew aR");

        let output = Command::new("sh")
            .arg("-c")
            .arg(&build_cmd)
            .current_dir(self.dest_prj_path)
            .output()?;

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;

        if !output.status.success() {
            return Err(CommandError(format!("run building java dest project failed.")).into());
        }

        let options = CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 1024,
            copy_inside: true,
            depth: 65535,
        };

        let src_arr = self
            .dest_prj_path
            .join("rustlib")
            .join("build")
            .join("outputs")
            .join("aar")
            .join("rustlib-release.aar");
        let target = self.origin_prj_path.join("target").join("android");
        if target.exists() {
            fs::remove_dir_all(&target).unwrap();
        }
        fs::create_dir_all(&target).unwrap();

        fs_extra::copy_items(&vec![src_arr], &target, &options)
            .map_err(|e| FileError(format!("copy android bridge outputs failed. {:?}", e)))
            .unwrap();

        Ok(())
    }
}
