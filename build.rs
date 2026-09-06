fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = "proto/arcrelay.proto";
    let common_schema = "proto/common.proto";
    println!("cargo:rerun-if-changed=proto");
    println!("cargo:rerun-if-changed={common_schema}");
    let protoc = protoc_bin_vendored::protoc_bin_path().expect("vendored protoc is unavailable");
    std::env::set_var("PROTOC", protoc);
    let mut config = prost_build::Config::new();
    config.bytes([
        ".arcrelay.v1.BlobStreamChunk.data",
        ".arcrelay.v1.PrintDocumentChunk.data",
    ]);
    config.compile_protos(&[schema], &["proto"])?;
    let mut common = prost_build::Config::new();
    common.bytes([
        ".arcrelay.transport.v1.SessionHello.nonce",
        ".arcrelay.transport.v1.SessionHello.root_public_key",
        ".arcrelay.transport.v1.SessionHello.endpoint_certificate_sha256",
        ".arcrelay.transport.v1.SessionHello.endpoint_binding_signature",
        ".arcrelay.transport.v1.SessionAuth.signature",
        ".arcrelay.transport.v1.ContentDescriptor.sha256",
        ".arcrelay.transport.v1.ContentTicket.token",
        ".arcrelay.transport.v1.FeaturePayload.body",
    ]);
    common.compile_protos(&[common_schema], &["proto"])?;
    Ok(())
}
