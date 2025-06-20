defmodule ExExtractous.Native do
  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :ex_extractous,
    crate: "ex_extractous",
    targets: ~w(
      x86_64-unknown-linux-gnu
      x86_64-unknown-linux-musl
    ),
    nif_versions: ["2.17"],
    base_url: "https://github.com/lawadvisor-apollo/ex_extractous/releases/download/v#{version}",
    force_build: System.get_env("EXTRACTOUS_BUILD") in ["1", "true"],
    version: version

  def extract(_path, _as_xml), do: :erlang.nif_error(:nif_not_loaded)
  def extract_with_ocr(_path, _as_xml), do: :erlang.nif_error(:nif_not_loaded)
end
