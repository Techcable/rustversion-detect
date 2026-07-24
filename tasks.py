from invoke import task
from pathlib import Path


@task
def check(ctx):
    ctx.run("cargo clippy --workspace --all-features", pty=True, echo=True)
    rdme(ctx, check=True)
    format(ctx, check=True)


@task
def rdme(ctx, check=False, force=False):
    flags = " --check" if check else ""
    if force:
        flags += " --force"
    ctx.run("cargo rdme" + flags, pty=True, echo=True)
    # TODO: Add CI for this
    ctx.run(
        "cargo rdme --manifest-path maybe-const-fn/Cargo.toml" + flags,
        pty=True,
        echo=True,
    )


@task
def format(ctx, check=False):
    maybe_check = " --check" if check else ""
    ctx.run("cargo +nightly fmt --all" + maybe_check, pty=True, echo=True)
    ctx.run("tombi format" + maybe_check, pty=True, echo=True)
    # TODO: Add this to CI?
    ctx.run("ruff format" + maybe_check, pty=True, echo=True)
