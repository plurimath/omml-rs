"""Build against the pinned local framework and Chrono source candidates."""
import argparse
import json
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[1]
cli = argparse.ArgumentParser(description=__doc__)
cli.add_argument("--framework-source", type=Path, required=True)
cli.add_argument("--chrono-source", type=Path, required=True)
cli.add_argument("--cargo", default="cargo")
cli.add_argument("--offline", action="store_true")
args = cli.parse_args()
pins = json.loads((root / "native-source.json").read_bytes())
framework = args.framework_source.resolve()
revision = subprocess.check_output(["git", "-C", framework, "rev-parse", "HEAD"], text=True).strip()
if revision != pins["rust_framework_candidate"]:
    raise RuntimeError("Framework source differs from the pinned candidate")
if subprocess.check_output(["git", "-C", framework, "status", "--porcelain"]):
    raise RuntimeError("Framework source must be clean")
subprocess.run([
    args.cargo, *(["--offline"] if args.offline else []),
    "--config", f'patch.crates-io.lutaml-rs.path="{framework}"',
    "--config", f'patch.crates-io.chrono.path="{args.chrono_source.resolve()}"',
    "build", "--locked", "--features", "model-serde", "--manifest-path", str(root / "Cargo.toml"),
], check=True)
