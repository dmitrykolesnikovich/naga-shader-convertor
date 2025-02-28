// A lot of the code can be unused based on configuration flags,
// the corresponding warnings aren't helpful.
#![allow(dead_code, unused_imports)]

use std::{
    fs,
    path::{Path, PathBuf},
};

const BASE_DIR_IN: &str = "tests/in";
const BASE_DIR_OUT: &str = "tests/out";

bitflags::bitflags! {
    struct Targets: u32 {
        const IR = 0x1;
        const ANALYSIS = 0x2;
        const SPIRV = 0x4;
        const METAL = 0x8;
        const GLSL = 0x10;
        const DOT = 0x20;
        const HLSL = 0x40;
        const WGSL = 0x80;
    }
}

#[derive(serde::Deserialize)]
struct SpvOutVersion(u8, u8);
impl Default for SpvOutVersion {
    fn default() -> Self {
        SpvOutVersion(1, 1)
    }
}

#[derive(Default, serde::Deserialize)]
struct SpirvOutParameters {
    version: SpvOutVersion,
    #[serde(default)]
    capabilities: naga::FastHashSet<spirv::Capability>,
    #[serde(default)]
    debug: bool,
    #[serde(default)]
    adjust_coordinate_space: bool,
    #[serde(default)]
    force_point_size: bool,
    #[serde(default)]
    clamp_frag_depth: bool,
    #[serde(default)]
    separate_entry_points: bool,
    #[serde(default)]
    #[cfg(all(feature = "deserialize", feature = "spv-out"))]
    binding_map: naga::back::spv::BindingMap,
}

#[derive(Default, serde::Deserialize)]
struct WgslOutParameters {
    #[serde(default)]
    explicit_types: bool,
}

#[derive(Default, serde::Deserialize)]
struct Parameters {
    #[serde(default)]
    god_mode: bool,
    #[cfg(feature = "deserialize")]
    #[serde(default)]
    bounds_check_policies: naga::proc::BoundsCheckPolicies,
    #[serde(default)]
    spv: SpirvOutParameters,
    #[cfg(all(feature = "deserialize", feature = "msl-out"))]
    #[serde(default)]
    msl: naga::back::msl::Options,
    #[cfg(all(feature = "deserialize", feature = "msl-out"))]
    #[serde(default)]
    msl_pipeline: naga::back::msl::PipelineOptions,
    #[cfg(all(feature = "deserialize", feature = "glsl-out"))]
    #[serde(default)]
    glsl: naga::back::glsl::Options,
    #[serde(default)]
    glsl_exclude_list: naga::FastHashSet<String>,
    #[cfg(all(feature = "deserialize", feature = "hlsl-out"))]
    #[serde(default)]
    hlsl: naga::back::hlsl::Options,
    #[serde(default)]
    wgsl: WgslOutParameters,
    #[cfg(all(feature = "deserialize", feature = "glsl-out"))]
    #[serde(default)]
    glsl_multiview: Option<std::num::NonZeroU32>,
}

#[allow(unused_variables)]
fn check_targets(module: &naga::Module, name: &str, targets: Targets) {
    let root = env!("CARGO_MANIFEST_DIR");
    let params = match fs::read_to_string(format!("{root}/{BASE_DIR_IN}/{name}.param.ron")) {
        Ok(string) => ron::de::from_str(&string).expect("Couldn't parse param file"),
        Err(_) => Parameters::default(),
    };

    let capabilities = if params.god_mode {
        naga::valid::Capabilities::all()
    } else {
        naga::valid::Capabilities::default()
    };

    let dest = PathBuf::from(root).join(BASE_DIR_OUT);

    #[cfg(feature = "serialize")]
    {
        if targets.contains(Targets::IR) {
            let config = ron::ser::PrettyConfig::default().new_line("\n".to_string());
            let string = ron::ser::to_string_pretty(module, config).unwrap();
            fs::write(dest.join(format!("ir/{name}.ron")), string).unwrap();
        }
    }

    let info = naga::valid::Validator::new(naga::valid::ValidationFlags::all(), capabilities)
        .validate(module)
        .expect("Naga module validation failed");

    #[cfg(feature = "serialize")]
    {
        if targets.contains(Targets::ANALYSIS) {
            let config = ron::ser::PrettyConfig::default().new_line("\n".to_string());
            let string = ron::ser::to_string_pretty(&info, config).unwrap();
            fs::write(dest.join(format!("analysis/{name}.info.ron")), string).unwrap();
        }
    }

    #[cfg(all(feature = "deserialize", feature = "spv-out"))]
    {
        if targets.contains(Targets::SPIRV) {
            write_output_spv(
                module,
                &info,
                &dest,
                name,
                &params.spv,
                params.bounds_check_policies,
            );
        }
    }
    #[cfg(all(feature = "deserialize", feature = "msl-out"))]
    {
        if targets.contains(Targets::METAL) {
            write_output_msl(
                module,
                &info,
                &dest,
                name,
                &params.msl,
                &params.msl_pipeline,
                params.bounds_check_policies,
            );
        }
    }
    #[cfg(all(feature = "deserialize", feature = "glsl-out"))]
    {
        if targets.contains(Targets::GLSL) {
            for ep in module.entry_points.iter() {
                if params.glsl_exclude_list.contains(&ep.name) {
                    continue;
                }
                write_output_glsl(
                    module,
                    &info,
                    &dest,
                    name,
                    ep.stage,
                    &ep.name,
                    &params.glsl,
                    params.bounds_check_policies,
                    params.glsl_multiview,
                );
            }
        }
    }
    #[cfg(feature = "dot-out")]
    {
        if targets.contains(Targets::DOT) {
            let string = naga::back::dot::write(module, Some(&info), Default::default()).unwrap();
            fs::write(dest.join(format!("dot/{name}.dot")), string).unwrap();
        }
    }
    #[cfg(all(feature = "deserialize", feature = "hlsl-out"))]
    {
        if targets.contains(Targets::HLSL) {
            write_output_hlsl(module, &info, &dest, name, &params.hlsl);
        }
    }
    #[cfg(all(feature = "deserialize", feature = "wgsl-out"))]
    {
        if targets.contains(Targets::WGSL) {
            write_output_wgsl(module, &info, &dest, name, &params.wgsl);
        }
    }
}

#[cfg(feature = "spv-out")]
fn write_output_spv(
    module: &naga::Module,
    info: &naga::valid::ModuleInfo,
    destination: &Path,
    file_name: &str,
    params: &SpirvOutParameters,
    bounds_check_policies: naga::proc::BoundsCheckPolicies,
) {
    use naga::back::spv;
    use rspirv::binary::Disassemble;

    println!("writing SPIR-V");

    let mut flags = spv::WriterFlags::LABEL_VARYINGS;
    flags.set(spv::WriterFlags::DEBUG, params.debug);
    flags.set(
        spv::WriterFlags::ADJUST_COORDINATE_SPACE,
        params.adjust_coordinate_space,
    );
    flags.set(spv::WriterFlags::FORCE_POINT_SIZE, params.force_point_size);
    flags.set(spv::WriterFlags::CLAMP_FRAG_DEPTH, params.clamp_frag_depth);

    let options = spv::Options {
        lang_version: (params.version.0, params.version.1),
        flags,
        capabilities: if params.capabilities.is_empty() {
            None
        } else {
            Some(params.capabilities.clone())
        },
        bounds_check_policies,
        binding_map: params.binding_map.clone(),
        zero_initialize_workgroup_memory: spv::ZeroInitializeWorkgroupMemoryMode::Polyfill,
    };

    if params.separate_entry_points {
        for ep in module.entry_points.iter() {
            let pipeline_options = spv::PipelineOptions {
                entry_point: ep.name.clone(),
                shader_stage: ep.stage,
            };
            let spv = spv::write_vec(module, info, &options, Some(&pipeline_options)).unwrap();
            let dis = rspirv::dr::load_words(spv)
                .expect("Produced invalid SPIR-V")
                .disassemble();
            let path = format!("spv/{}.{}.spvasm", file_name, ep.name);
            fs::write(destination.join(path), dis).unwrap();
        }
    } else {
        let spv = spv::write_vec(module, info, &options, None).unwrap();
        let dis = rspirv::dr::load_words(spv)
            .expect("Produced invalid SPIR-V")
            .disassemble();
        fs::write(destination.join(format!("spv/{file_name}.spvasm")), dis).unwrap();
    }
}

#[cfg(feature = "msl-out")]
fn write_output_msl(
    module: &naga::Module,
    info: &naga::valid::ModuleInfo,
    destination: &Path,
    file_name: &str,
    options: &naga::back::msl::Options,
    pipeline_options: &naga::back::msl::PipelineOptions,
    bounds_check_policies: naga::proc::BoundsCheckPolicies,
) {
    use naga::back::msl;

    println!("writing MSL");

    let mut options = options.clone();
    options.bounds_check_policies = bounds_check_policies;
    let (string, tr_info) = msl::write_string(module, info, &options, pipeline_options)
        .unwrap_or_else(|err| panic!("Metal write failed: {err}"));

    for (ep, result) in module.entry_points.iter().zip(tr_info.entry_point_names) {
        if let Err(error) = result {
            panic!("Failed to translate '{}': {}", ep.name, error);
        }
    }

    fs::write(destination.join(format!("msl/{file_name}.msl")), string).unwrap();
}

#[cfg(feature = "glsl-out")]
#[allow(clippy::too_many_arguments)]
fn write_output_glsl(
    module: &naga::Module,
    info: &naga::valid::ModuleInfo,
    destination: &Path,
    file_name: &str,
    stage: naga::ShaderStage,
    ep_name: &str,
    options: &naga::back::glsl::Options,
    bounds_check_policies: naga::proc::BoundsCheckPolicies,
    multiview: Option<std::num::NonZeroU32>,
) {
    use naga::back::glsl;

    println!("writing GLSL");

    let pipeline_options = glsl::PipelineOptions {
        shader_stage: stage,
        entry_point: ep_name.to_string(),
        multiview,
    };

    let mut buffer = String::new();
    let mut writer = glsl::Writer::new(
        &mut buffer,
        module,
        info,
        options,
        &pipeline_options,
        bounds_check_policies,
    )
    .expect("GLSL init failed");
    writer.write().expect("GLSL write failed");

    fs::write(
        destination.join(format!("glsl/{file_name}.{ep_name}.{stage:?}.glsl")),
        buffer,
    )
    .unwrap();
}

#[cfg(feature = "hlsl-out")]
fn write_output_hlsl(
    module: &naga::Module,
    info: &naga::valid::ModuleInfo,
    destination: &Path,
    file_name: &str,
    options: &naga::back::hlsl::Options,
) {
    use naga::back::hlsl;
    use std::fmt::Write as _;

    println!("writing HLSL");

    let mut buffer = String::new();
    let mut writer = hlsl::Writer::new(&mut buffer, options);
    let reflection_info = writer.write(module, info).expect("HLSL write failed");

    fs::write(destination.join(format!("hlsl/{file_name}.hlsl")), buffer).unwrap();

    // We need a config file for validation script
    // This file contains an info about profiles (shader stages) contains inside generated shader
    // This info will be passed to dxc
    let mut config_str = String::new();
    let mut vertex_str = String::from("vertex=(");
    let mut fragment_str = String::from("fragment=(");
    let mut compute_str = String::from("compute=(");
    for (index, ep) in module.entry_points.iter().enumerate() {
        let name = match reflection_info.entry_point_names[index] {
            Ok(ref name) => name,
            Err(_) => continue,
        };
        match ep.stage {
            naga::ShaderStage::Vertex => {
                write!(
                    vertex_str,
                    "{}:{}_{} ",
                    name,
                    ep.stage.to_hlsl_str(),
                    options.shader_model.to_str(),
                )
                .unwrap();
            }
            naga::ShaderStage::Fragment => {
                write!(
                    fragment_str,
                    "{}:{}_{} ",
                    name,
                    ep.stage.to_hlsl_str(),
                    options.shader_model.to_str(),
                )
                .unwrap();
            }
            naga::ShaderStage::Compute => {
                write!(
                    compute_str,
                    "{}:{}_{} ",
                    name,
                    ep.stage.to_hlsl_str(),
                    options.shader_model.to_str(),
                )
                .unwrap();
            }
        }
    }

    writeln!(config_str, "{vertex_str})\n{fragment_str})\n{compute_str})").unwrap();

    fs::write(
        destination.join(format!("hlsl/{file_name}.hlsl.config")),
        config_str,
    )
    .unwrap();
}

#[cfg(feature = "wgsl-out")]
fn write_output_wgsl(
    module: &naga::Module,
    info: &naga::valid::ModuleInfo,
    destination: &Path,
    file_name: &str,
    params: &WgslOutParameters,
) {
    use naga::back::wgsl;

    println!("writing WGSL");

    let mut flags = wgsl::WriterFlags::empty();
    flags.set(wgsl::WriterFlags::EXPLICIT_TYPES, params.explicit_types);

    let string = wgsl::write_string(module, info, flags).expect("WGSL write failed");

    fs::write(destination.join(format!("wgsl/{file_name}.wgsl")), string).unwrap();
}

#[cfg(feature = "wgsl-in")]
#[test]
fn convert_wgsl() {
    let _ = env_logger::try_init();

    let root = env!("CARGO_MANIFEST_DIR");
    let inputs = [
        (
            "empty",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "quad",
            Targets::SPIRV
                | Targets::METAL
                | Targets::GLSL
                | Targets::DOT
                | Targets::HLSL
                | Targets::WGSL,
        ),
        (
            "bits",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::WGSL,
        ),
        (
            "bitcast",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "boids",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "skybox",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "collatz",
            Targets::SPIRV
                | Targets::METAL
                | Targets::IR
                | Targets::ANALYSIS
                | Targets::HLSL
                | Targets::WGSL,
        ),
        (
            "shadow",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "image",
            Targets::SPIRV | Targets::METAL | Targets::HLSL | Targets::WGSL | Targets::GLSL,
        ),
        ("extra", Targets::SPIRV | Targets::METAL | Targets::WGSL),
        ("push-constants", Targets::GLSL | Targets::HLSL),
        (
            "operators",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "functions",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        ("functions-webgl", Targets::GLSL),
        (
            "interpolate",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "access",
            Targets::SPIRV
                | Targets::METAL
                | Targets::GLSL
                | Targets::HLSL
                | Targets::WGSL
                | Targets::IR
                | Targets::ANALYSIS,
        ),
        ("atomicCompareExchange", Targets::SPIRV | Targets::WGSL),
        (
            "padding",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        ("pointers", Targets::SPIRV | Targets::WGSL),
        (
            "control-flow",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "standard",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        //TODO: GLSL https://github.com/gfx-rs/naga/issues/874
        (
            "interface",
            Targets::SPIRV | Targets::METAL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "globals",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        ("bounds-check-zero", Targets::SPIRV | Targets::METAL),
        ("bounds-check-zero-atomic", Targets::METAL),
        ("bounds-check-restrict", Targets::SPIRV | Targets::METAL),
        (
            "bounds-check-image-restrict",
            Targets::SPIRV | Targets::METAL | Targets::GLSL,
        ),
        (
            "bounds-check-image-rzsw",
            Targets::SPIRV | Targets::METAL | Targets::GLSL,
        ),
        ("policy-mix", Targets::SPIRV | Targets::METAL),
        (
            "texture-arg",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        ("cubeArrayShadow", Targets::GLSL),
        (
            "math-functions",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        ("cubeArrayShadow", Targets::GLSL),
        (
            "binding-arrays",
            Targets::WGSL | Targets::HLSL | Targets::METAL | Targets::SPIRV,
        ),
        ("multiview", Targets::SPIRV | Targets::GLSL | Targets::WGSL),
        ("multiview_webgl", Targets::GLSL),
        (
            "break-if",
            Targets::WGSL | Targets::GLSL | Targets::SPIRV | Targets::HLSL | Targets::METAL,
        ),
        ("lexical-scopes", Targets::WGSL),
        ("type-alias", Targets::WGSL),
        ("module-scope", Targets::WGSL),
        (
            "workgroup-var-init",
            Targets::WGSL | Targets::GLSL | Targets::SPIRV | Targets::HLSL | Targets::METAL,
        ),
        ("sprite", Targets::SPIRV),
        ("force_point_size_vertex_shader_webgl", Targets::GLSL),
        ("invariant", Targets::GLSL),
    ];

    for &(name, targets) in inputs.iter() {
        println!("Processing '{name}'");
        // WGSL shaders lives in root dir as a privileged.
        let file = fs::read_to_string(format!("{root}/{BASE_DIR_IN}/{name}.wgsl"))
            .expect("Couldn't find wgsl file");
        match naga::front::wgsl::parse_str(&file) {
            Ok(module) => check_targets(&module, name, targets),
            Err(e) => panic!("{}", e.emit_to_string(&file)),
        }
    }
}

#[cfg(feature = "spv-in")]
fn convert_spv(name: &str, adjust_coordinate_space: bool, targets: Targets) {
    let _ = env_logger::try_init();

    let root = env!("CARGO_MANIFEST_DIR");
    let module = naga::front::spv::parse_u8_slice(
        &fs::read(format!("{root}/{BASE_DIR_IN}/spv/{name}.spv")).expect("Couldn't find spv file"),
        &naga::front::spv::Options {
            adjust_coordinate_space,
            strict_capabilities: false,
            block_ctx_dump_prefix: None,
        },
    )
    .unwrap();
    check_targets(&module, name, targets);
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::default(),
    )
    .validate(&module)
    .unwrap();
}

#[cfg(feature = "spv-in")]
#[test]
fn convert_spv_all() {
    convert_spv(
        "quad-vert",
        false,
        Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
    );
    convert_spv("shadow", true, Targets::IR | Targets::ANALYSIS);
    convert_spv(
        "inv-hyperbolic-trig-functions",
        true,
        Targets::HLSL | Targets::WGSL,
    );
    convert_spv(
        "empty-global-name",
        true,
        Targets::HLSL | Targets::WGSL | Targets::METAL,
    );
    convert_spv("degrees", false, Targets::empty());
    convert_spv("binding-arrays.dynamic", true, Targets::WGSL);
    convert_spv("binding-arrays.static", true, Targets::WGSL);
}

#[cfg(feature = "glsl-in")]
#[test]
fn convert_glsl_variations_check() {
    let root = env!("CARGO_MANIFEST_DIR");
    let file = fs::read_to_string(format!("{root}/{BASE_DIR_IN}/variations.glsl"))
        .expect("Couldn't find glsl file");
    let mut parser = naga::front::glsl::Frontend::default();
    let module = parser
        .parse(
            &naga::front::glsl::Options {
                stage: naga::ShaderStage::Fragment,
                defines: Default::default(),
            },
            &file,
        )
        .unwrap();
    check_targets(&module, "variations-glsl", Targets::GLSL);
}

#[cfg(feature = "glsl-in")]
#[allow(unused_variables)]
#[test]
fn convert_glsl_folder() {
    let _ = env_logger::try_init();

    let root = env!("CARGO_MANIFEST_DIR");

    for entry in std::fs::read_dir(format!("{root}/{BASE_DIR_IN}/glsl")).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name().into_string().unwrap();

        if file_name.ends_with(".ron") {
            // No needed to validate ron files
            continue;
        }
        println!("Processing {file_name}");

        let mut parser = naga::front::glsl::Frontend::default();
        let module = parser
            .parse(
                &naga::front::glsl::Options {
                    stage: match entry.path().extension().and_then(|s| s.to_str()).unwrap() {
                        "vert" => naga::ShaderStage::Vertex,
                        "frag" => naga::ShaderStage::Fragment,
                        "comp" => naga::ShaderStage::Compute,
                        ext => panic!("Unknown extension for glsl file {ext}"),
                    },
                    defines: Default::default(),
                },
                &fs::read_to_string(entry.path()).expect("Couldn't find glsl file"),
            )
            .unwrap();

        let info = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .unwrap();

        #[cfg(feature = "wgsl-out")]
        {
            let dest = PathBuf::from(root).join(BASE_DIR_OUT);
            write_output_wgsl(
                &module,
                &info,
                &dest,
                &file_name.replace('.', "-"),
                &WgslOutParameters::default(),
            );
        }
    }
}
