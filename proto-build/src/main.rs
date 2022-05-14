use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process,
    sync::atomic::{self, AtomicBool},
};

use common::tracing::{self, error, info};
use prost_wkt_build::{FileDescriptorSet, Message as _};
use regex::Regex;
use walkdir::WalkDir;

/// The tag to be cloned for the osmosis-labs fork of cosmos-sdk.
const OSMOSIS_COSMOS_SDK_REV: &str = "v0.45.1";
/// The tag to be cloned for the osmosis repo.
const OSMOSIS_REV: &str = "mattverse/msg-defined-twice";

/// Directory where the osmosis cosmos-sdk submodule is located
const OSMOSIS_COSMOS_SDK_DIR: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../osmosis-cosmos-sdk-go");
/// Directory where the cosmos-sdk submodule is located
const OSMOSIS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../osmosis-go");

/// The directory generated osmosis proto files go into in this repo
const OSMOSIS_PROTO_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../osmosisrs/src/proto/");

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto`)
const EXCLUDED_PROTO_PACKAGES: &[&str] = &["gogoproto", "google", "tendermint"];
/// Regex for locating instances of `tendermint-proto` in prost/tonic build output
const TENDERMINT_PROTO_REGEX: &str = "(super::)+tendermint";
/// Attribute preceeding a Tonic client definition
const TONIC_CLIENT_ATTRIBUTE: &str = "/// Generated client implementations.";
/// Attributes to add to gRPC clients
const GRPC_CLIENT_ATTRIBUTES: &[&str] = &[
    TONIC_CLIENT_ATTRIBUTE,
    "#[cfg(feature = \"grpc\")]",
    "#[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
];

/// Suppress log messages
static QUIET: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::init();

    let proto_dir: PathBuf = OSMOSIS_PROTO_DIR.parse()?;
    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse()?;

    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone())?;
    }

    fs::create_dir(tmp_build_dir.clone())?;

    update_submodules();
    output_osmosis_cosmos_sdk_version(&tmp_build_dir);
    output_osmosis_version(&tmp_build_dir);
    compile_osmosis_protos_and_services(&tmp_build_dir);
    copy_generated_files(&tmp_build_dir, &proto_dir);

    Ok(())
}

fn is_quiet() -> bool {
    QUIET.load(atomic::Ordering::Relaxed)
}

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = if is_quiet() {
        process::Stdio::null()
    } else {
        process::Stdio::inherit()
    };

    let exit_status = process::Command::new("git")
        .args(args)
        .stdout(stdout)
        .status()
        .expect("git exit status missing");

    if !exit_status.success() {
        panic!("git exited with error code: {:?}", exit_status.code());
    }
}

fn update_submodules() {
    info!("Updating osmosis-labls/osmosis submodule...");
    run_git(&["submodule", "update", "--init"]);
    run_git(&["-C", OSMOSIS_DIR, "fetch"]);
    run_git(&["-C", OSMOSIS_DIR, "reset", "--hard", OSMOSIS_REV]);
}

fn output_osmosis_cosmos_sdk_version(out_dir: &Path) {
    let path = out_dir.join("OSMOSIS_COSMOS_SDK_COMMIT");
    fs::write(path, OSMOSIS_COSMOS_SDK_REV).unwrap();
}

fn output_osmosis_version(out_dir: &Path) {
    let path = out_dir.join("OSMOSIS_COMMIT");
    fs::write(path, OSMOSIS_REV).unwrap();
}

fn compile_osmosis_protos_and_services(out_dir: &Path) {
    info!(
        "Compiling osmosis .proto files to Rust into '{}'...",
        out_dir.display()
    );

    let root = env!("CARGO_MANIFEST_DIR");
    let osmosis_dir = Path::new(OSMOSIS_DIR);
    let sdk_dir = Path::new(OSMOSIS_COSMOS_SDK_DIR);

    let proto_includes_paths = [
        format!("{}/../proto", root),
        format!("{}/proto", sdk_dir.display()),
        format!("{}/third_party/proto", sdk_dir.display()),
        format!("{}/proto", osmosis_dir.display()),
    ];

    // Paths
    let proto_paths = [
        // cosmos
        format!("{}/../proto/definitions/mock", root),
        format!("{}/proto/cosmos/auth", sdk_dir.display()),
        format!("{}/proto/cosmos/authz", sdk_dir.display()),
        format!("{}/proto/cosmos/feegrant", sdk_dir.display()),
        format!("{}/proto/cosmos/bank", sdk_dir.display()),
        format!("{}/proto/cosmos/base", sdk_dir.display()),
        format!("{}/proto/cosmos/base/tendermint", sdk_dir.display()),
        format!("{}/proto/cosmos/capability", sdk_dir.display()),
        format!("{}/proto/cosmos/crisis", sdk_dir.display()),
        format!("{}/proto/cosmos/crypto", sdk_dir.display()),
        format!("{}/proto/cosmos/distribution", sdk_dir.display()),
        format!("{}/proto/cosmos/evidence", sdk_dir.display()),
        format!("{}/proto/cosmos/genutil", sdk_dir.display()),
        format!("{}/proto/cosmos/gov", sdk_dir.display()),
        format!("{}/proto/cosmos/mint", sdk_dir.display()),
        format!("{}/proto/cosmos/params", sdk_dir.display()),
        format!("{}/proto/cosmos/slashing", sdk_dir.display()),
        format!("{}/proto/cosmos/staking", sdk_dir.display()),
        format!("{}/proto/cosmos/tx", sdk_dir.display()),
        format!("{}/proto/cosmos/upgrade", sdk_dir.display()),
        format!("{}/proto/cosmos/vesting", sdk_dir.display()),
        // osmosis
        format!("{}/proto/osmosis/epochs", osmosis_dir.display()),
        format!("{}/proto/osmosis/gamm", osmosis_dir.display()),
        format!("{}/proto/osmosis/incentives", osmosis_dir.display()),
        format!("{}/proto/osmosis/lockup", osmosis_dir.display()),
        format!("{}/proto/osmosis/mint", osmosis_dir.display()),
        format!("{}/proto/osmosis/pool-incentives", osmosis_dir.display()),
        format!("{}/proto/osmosis/store", osmosis_dir.display()),
        format!("{}/proto/osmosis/superfluid", osmosis_dir.display()),
        format!("{}/proto/osmosis/tokenfactory", osmosis_dir.display()),
        format!("{}/proto/osmosis/txfees", osmosis_dir.display()),
    ];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    let descriptor_file = out_dir.join("descriptors.bin");

    // Compile all of the proto files, along with grpc service clients
    info!("Compiling proto definitions and clients for GRPC services!");
    tonic_build::configure()
        .extern_path(".tendermint", "::tendermint_proto")
        .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
        .extern_path(".google.protobuf.Duration", "::prost_wkt_types::Duration")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .build_client(true)
        .build_server(false)
        .file_descriptor_set_path(&descriptor_file)
        .out_dir(out_dir)
        .compile(&protos, &includes)
        .unwrap();

    let descriptor_bytes = std::fs::read(descriptor_file).unwrap();

    let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();

    prost_wkt_build::add_serde(out_dir.to_path_buf(), descriptor);

    info!("=> Done!");
}

/// collect_protos walks every path in `proto_paths` and recursively locates all .proto
/// files in each path's subdirectories, adding the full path of each file to `protos`
///
/// Any errors encountered will cause failure for the path provided to WalkDir::new()
fn collect_protos(proto_paths: &[String], protos: &mut Vec<PathBuf>) {
    for proto_path in proto_paths {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }
}

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    info!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    std::fs::remove_dir_all(&to_dir).unwrap_or_default();
    std::fs::create_dir_all(&to_dir).unwrap();

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        // .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_type().is_file() && e.file_name() != "descriptors.bin")
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            copy_and_patch(e.path(), format!("{}/{}", to_dir.display(), &filename))
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            error!("Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}

fn copy_and_patch(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in EXCLUDED_PROTO_PACKAGES {
        if let Some(filename) = src.as_ref().file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let contents = fs::read_to_string(src)?;

    // `prost-build` output references types from `tendermint-proto` crate
    // relative paths, which we need to munge into `tendermint_proto` in
    // order to leverage types from the upstream crate.
    let contents = Regex::new(TENDERMINT_PROTO_REGEX)
        .unwrap()
        .replace_all(&contents, "tendermint_proto");

    // Patch each service definition with a feature attribute
    let patched_contents =
        contents.replace(TONIC_CLIENT_ATTRIBUTE, &GRPC_CLIENT_ATTRIBUTES.join("\n"));

    fs::write(dest, patched_contents)
}
