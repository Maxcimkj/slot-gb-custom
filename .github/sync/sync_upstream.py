#!/usr/bin/env python3
"""Sync the fork with upstream through a pull request that a coding agent resolves.

Run by .github/workflows/sync-upstream.yml from a checkout of the fork's main, in two parts:

prepare
    Copy upstream's branch to sync/upstream-<sha> in the fork and open a pull request from it
    into main; GitHub shows it as conflicted when the fork's changes clash. Then merge main into
    that branch locally, the way GitHub's Resolve conflicts button does. Binary files and files
    one side deleted take upstream's state here. Text conflicts are left in the files, with a
    prompt for the coding agent (OpenCode), which resolves them and runs the build until it passes.

verify
    After the agent: check it left no conflict markers and did not touch the files it must not,
    then stage the resolution. The workflow then runs the checks itself.

finish
    Commit the merge with a record in rebase-log.md, push the branch and put the record, the
    agent's notes and the check results in the pull request description. The pull request is
    never merged here: merging it is the owner's approval.

Upstream wins: its code is kept and the fork's changes are re-applied on top (FORK-NOTES.md).

Environment: UPSTREAM_REPO, UPSTREAM_BRANCH, GH_TOKEN for the gh CLI, CHECKS (finish), and
GitHub's GITHUB_OUTPUT, GITHUB_SERVER_URL, GITHUB_REPOSITORY, GITHUB_RUN_ID, RUNNER_TEMP.
"""
import datetime
import hashlib
import json
import os
import re
import subprocess
import sys
from pathlib import Path

UPSTREAM_REPO = os.environ.get("UPSTREAM_REPO", "BrandonKowalski/slot")
UPSTREAM_BRANCH = os.environ.get("UPSTREAM_BRANCH", "main")
UPSTREAM_URL = os.environ.get("UPSTREAM_URL", f"https://github.com/{UPSTREAM_REPO}.git")
REPO = os.environ.get("GITHUB_REPOSITORY", "Maxcimkj/slot-gb-custom")
UPSTREAM = f"upstream/{UPSTREAM_BRANCH}"
# The fork branch being synced: main, or a test branch when the workflow runs from one.
BASE = os.environ.get("BASE_BRANCH") or "main"
# Kept under sync/: a branch named after BASE would clash with BASE itself in git's ref tree.
BRANCH_PREFIX = "sync/upstream-" if BASE == "main" else f"sync/{BASE}-upstream-"
TEMP = Path(os.environ.get("RUNNER_TEMP", "/tmp"))
STATE = TEMP / "sync-state.json"
PROMPT = TEMP / "sync-prompt.md"
LOG = Path("rebase-log.md")
NOTES = Path("FORK-NOTES.md")
AGENT_NOTES = Path(".sync-notes.md")
# The agent may not change these, unless one of them is itself a conflicted file.
PROTECTED = (".github/", str(NOTES), str(LOG))
MAX_LISTED = 100

MARKER = re.compile(r"^(?:<{7}|\|{7}|={7}|>{7})(?: |\r?$)", re.M)

# Checks the workflow runs after the agent, in order: (output name, label, required).
CHECKS = [
    ("fmt", "`cargo fmt --all -- --check`", True),
    ("clippy", "`cargo clippy --workspace --all-targets -- -D warnings`", True),
    ("clippy_device", "`cargo clippy -p slot --no-default-features --features device -- -D warnings`", True),
    ("device", "`cargo build --profile device -p slot --no-default-features --features device`", True),
    ("tests", "`cargo test --workspace --no-fail-fast` (advisory: some tests need cores the runner lacks)", False),
]

AGENT_PROMPT = """You are resolving a merge of the fork {repo} with its upstream project {upstream}.

The working tree is in the middle of `git merge`: branch `{branch}` holds upstream's latest code
(`{new_upstream}`) and the fork's `{base}` (`{fork_main}`) is being merged into it. These files
still contain conflict markers:

{files}

In every conflict the first side (`<<<<<<< HEAD`) is UPSTREAM, the middle (`||||||| ...`) is the
common BASE, and the last side (`>>>>>>> ...`) is the FORK.

Rules, in priority order:
1. Upstream wins. Keep every upstream change: renames, moved code, new fields, new enum variants,
   new arguments, removed code, formatting. Never drop or undo upstream work to make the fork's
   change fit.
2. Then re-apply the fork's change on top of upstream's version, adapted to upstream's new code
   so it still does what the fork intended. Read FORK-NOTES.md first: it lists every fork change,
   what it must keep, and how to resolve likely conflicts.
3. If the fork's change is already in upstream, or no longer makes sense against upstream, keep
   upstream's version and say so in your notes.
4. Code must read like the surrounding code. Do not add comments about the merge.

Useful commands:
- `git diff {old_base} {upstream_ref} -- <file>`: what upstream changed since the last sync.
- `git diff {old_base} {fork_main} -- <file>`: what the fork changed.
- `git log --oneline {old_base}..{upstream_ref}`: upstream's new commits.

When the markers are gone, run these and fix what fails, also in files that did not conflict
(upstream may have changed an API the fork uses). Repeat until they pass:
1. `cargo fmt --all`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo clippy -p slot --no-default-features --features device -- -D warnings`
4. `cargo build --profile device --target-dir target-device -p slot --no-default-features --features device`
5. `cargo test -p <crate>` for the crates you changed. A test that fails because a libretro core,
   a BIOS or a display is missing on this machine is not your concern; mention it in the notes.

Do not run `git add`, `git commit`, `git checkout`, `git merge` or `git reset`, and do not edit
anything under `.github/`, `FORK-NOTES.md` or `rebase-log.md`; the workflow commits your work.

Finally write `.sync-notes.md` in the repository root. For each file you changed, a `### <path>`
heading followed by 2 to 5 plain sentences: what upstream changed, what the fork change does,
and how you combined them. Then a `### Checks` heading saying which commands passed and anything
you could not fix.
"""


def git(*args, check=True):
    r = subprocess.run(["git", *args], capture_output=True, text=True)
    if check and r.returncode:
        sys.exit(f"git {' '.join(args)} failed:\n{r.stdout}{r.stderr}")
    return r.stdout.strip()


def gh(*args):
    # --repo on every call: in a fork, gh otherwise targets the parent repository.
    r = subprocess.run(["gh", *args, "--repo", REPO], capture_output=True, text=True)
    if r.returncode:
        sys.exit(f"gh {' '.join(args)} failed:\n{r.stdout}{r.stderr}")
    return r.stdout.strip()


def blob(spec):
    return subprocess.run(["git", "show", spec], capture_output=True).stdout


def output(**values):
    path = os.environ.get("GITHUB_OUTPUT")
    for key, value in values.items():
        value = str(value)
        multiline = "\n" in value
        print(f"{key}=<{len(value)} characters>" if multiline else f"{key}={value}")
        if path:
            with open(path, "a") as f:
                if multiline:
                    f.write(f"{key}<<SYNC_EOF\n{value.rstrip()}\nSYNC_EOF\n")
                else:
                    f.write(f"{key}={value}\n")


def fingerprint(conflicted):
    """Hash every protected file that is not itself conflicted."""
    paths = git("ls-files", "--", *PROTECTED).splitlines()
    return {
        p: hashlib.sha256(Path(p).read_bytes()).hexdigest() if Path(p).exists() else None
        for p in sorted(set(paths))
        if p not in conflicted
    }


def auto_resolve(path):
    """Resolve what needs no agent, and stage it. Returns the note, or None for a text conflict."""
    stages = {line.split()[2] for line in git("ls-files", "-u", "--", path).splitlines()}
    # Merging main into the upstream branch: stage 2 ("ours") is upstream, 3 ("theirs") the fork.
    if "2" not in stages:
        git("rm", "-q", "--", path)
        return "Upstream deleted this file, so the fork's edits to it were dropped (upstream wins)."
    if "3" not in stages:
        git("checkout", "--ours", "--", path)
        git("add", "--", path)
        return "The fork deleted this file but upstream changed it; kept upstream's version."
    if any(b"\0" in blob(f":{s}:{path}") for s in stages):
        git("checkout", "--ours", "--", path)
        git("add", "--", path)
        return "Binary file changed on both sides; kept upstream's version and dropped the fork's."
    return None


def sync_prs():
    prs = json.loads(gh("pr", "list", "--base", BASE, "--state", "open", "--json", "number,url,headRefName"))
    return [p for p in prs if p["headRefName"].startswith(BRANCH_PREFIX)]


def prepare():
    git("config", "user.name", "github-actions[bot]")
    git("config", "user.email", "41898282+github-actions[bot]@users.noreply.github.com")
    git("config", "merge.conflictStyle", "diff3")
    git("fetch", "--no-tags", UPSTREAM_URL, f"+refs/heads/{UPSTREAM_BRANCH}:refs/remotes/{UPSTREAM}")
    fork_main = git("rev-parse", "HEAD")
    old_base = git("merge-base", "HEAD", UPSTREAM)
    new_upstream = git("rev-parse", UPSTREAM)
    if old_base == new_upstream:
        print(f"{BASE} already contains upstream {new_upstream[:7]}, nothing to do")
        output(changed="false")
        return
    branch = f"{BRANCH_PREFIX}{new_upstream[:7]}"
    for pr in sync_prs():
        if pr["headRefName"] == branch:
            print(f"{pr['url']} already syncs upstream {new_upstream[:7]} and waits for review")
            output(changed="false", pr=pr["url"])
            return
    upstream_commits = git("log", "--no-merges", "--format=`%h` %s (%an)", f"{old_base}..{new_upstream}").splitlines()
    fork_commits = git("log", "--no-merges", "--format=`%h` %s", f"{new_upstream}..{fork_main}").splitlines()
    print(f"{len(upstream_commits)} upstream commits, {len(fork_commits)} fork commits")

    git("push", "-f", "origin", f"{new_upstream}:refs/heads/{branch}")
    pr = gh("pr", "create", "--base", BASE, "--head", branch,
            "--title", f"Sync upstream {UPSTREAM_REPO}@{new_upstream[:7]}",
            "--body", f"{len(upstream_commits)} upstream commits. The sync workflow is resolving "
                      "the conflicts; this description is replaced when it is done.")
    for old in sync_prs():
        if old["headRefName"] != branch:
            gh("pr", "close", str(old["number"]), "--delete-branch",
               "--comment", f"Superseded by {pr}, which syncs a newer upstream.")
    output(pr=pr, branch=branch)

    git("checkout", "-q", "-B", branch, new_upstream)
    merge = subprocess.run(["git", "merge", "--no-ff", "--no-commit", fork_main], capture_output=True, text=True)
    unmerged = git("diff", "--name-only", "--diff-filter=U").splitlines()
    if merge.returncode and not unmerged:
        sys.exit(f"git merge failed:\n{merge.stdout}{merge.stderr}")
    auto, text = {}, []
    for path in unmerged:
        note = auto_resolve(path)
        if note:
            auto[path] = note
        else:
            text.append(path)
    state = dict(old_base=old_base, new_upstream=new_upstream, fork_main=fork_main, branch=branch, pr=pr,
                 upstream_commits=upstream_commits, fork_commits=fork_commits, auto=auto, text=text,
                 protected=fingerprint(set(text)))
    STATE.write_text(json.dumps(state))
    prompt = AGENT_PROMPT.format(
        repo=REPO, upstream=UPSTREAM_REPO, base=BASE, branch=branch, new_upstream=new_upstream[:7],
        fork_main=fork_main[:7], old_base=old_base[:7], upstream_ref=UPSTREAM,
        files="\n".join(f"- `{p}`" for p in text) or "- (none: the merge is clean, only check the build)",
    )
    PROMPT.write_text(prompt)
    output(changed="true", upstream=new_upstream[:7], conflicts=len(text), prompt=prompt)


def verify():
    state = json.loads(STATE.read_text())
    problems = [f"`{p}` still has conflict markers" for p in state["text"]
                if Path(p).exists() and MARKER.search(Path(p).read_text(errors="replace"))]
    now = fingerprint(set(state["text"]))
    problems += [f"`{p}` was changed, and the agent may not touch it"
                 for p, h in state["protected"].items() if now.get(p) != h]
    if AGENT_NOTES.exists():
        state["agent_notes"] = AGENT_NOTES.read_text().strip()
        AGENT_NOTES.unlink()
        STATE.write_text(json.dumps(state))
    if problems:
        sys.exit("The agent's resolution cannot be used:\n" + "\n".join(f"- {p}" for p in problems))
    git("add", "-u")
    if state["text"]:
        git("add", "--", *[p for p in state["text"] if Path(p).exists()])
    left = git("diff", "--name-only", "--diff-filter=U")
    if left:
        sys.exit(f"Still unmerged after the agent:\n{left}")
    print("resolution staged:", git("diff", "--cached", "--stat", "HEAD"))


def record(state, results):
    now = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
    server, run = os.environ.get("GITHUB_SERVER_URL"), os.environ.get("GITHUB_RUN_ID")
    upstream_commits, fork_commits = state["upstream_commits"], state["fork_commits"]
    lines = [f"## {now} · upstream `{state['old_base'][:7]}` → `{state['new_upstream'][:7]}`", "",
             f"Pull request: {state['pr']}"]
    if server and run:
        lines.append(f"Run: {server}/{REPO}/actions/runs/{run}")
    lines += ["", f"### Upstream changes ({len(upstream_commits)} commits)", ""]
    lines += [f"- {c}" for c in upstream_commits[:MAX_LISTED]]
    if len(upstream_commits) > MAX_LISTED:
        lines.append(f"- … and {len(upstream_commits) - MAX_LISTED} more")
    lines += ["", f"### Fork changes kept on top ({len(fork_commits)} commits)", ""]
    lines += [f"- {c}" for c in fork_commits[:MAX_LISTED]]
    lines += ["", "### Conflicts", ""]
    if not state["auto"] and not state["text"]:
        lines.append("None. Upstream merged cleanly into the fork.")
    for path, note in state["auto"].items():
        lines.append(f"- `{path}`: {note}")
    if state["text"]:
        model = os.environ.get("AGENT_MODEL") or "the coding agent"
        lines.append(f"- Resolved by OpenCode with {model}: {', '.join(f'`{p}`' for p in state['text'])}")
    if state.get("agent_notes"):
        lines += ["", "#### Agent notes", "", re.sub(r"^#{1,4} ", "##### ", state["agent_notes"], flags=re.M)]
    lines += ["", "### Checks", "", "| Check | Result |", "|---|---|"]
    for name, label, required in CHECKS:
        result = results.get(name, "skipped")
        mark = "✅" if result == "success" else ("❌" if required else "⚠️")
        lines.append(f"| {label} | {mark} {result} |")
    return "\n".join(lines) + "\n"


def write_log(entry):
    header = (
        "# Rebase log\n\n"
        "One record per upstream sync, newest first. Written by "
        "`.github/workflows/sync-upstream.yml`; see `FORK-NOTES.md` for how the fork is kept.\n\n"
    )
    old = LOG.read_text() if LOG.exists() else header
    body = old[len(header):] if old.startswith(header) else old
    LOG.write_text(header + entry + "\n" + body)


def finish():
    state = json.loads(STATE.read_text())
    results = dict(item.split("=", 1) for item in os.environ.get("CHECKS", "").split(",") if "=" in item)
    passed = all(results.get(name) == "success" for name, _, required in CHECKS if required)
    entry = record(state, results)
    write_log(entry)
    git("add", "--", str(LOG))
    git("commit", "-q", "-m", f"Merge upstream {UPSTREAM_REPO}@{state['new_upstream'][:7]} with the fork", "-m",
        f"{len(state['upstream_commits'])} upstream commits, {len(state['auto']) + len(state['text'])} "
        f"conflicted file(s). Pull request: {state['pr']}\nDetails in rebase-log.md.")
    git("push", "origin", f"HEAD:refs/heads/{state['branch']}")
    status = ("**All required checks passed. Review the changes and merge this pull request to sync "
              f"`{BASE}`.**" if passed else
              "**Required checks failed: do not merge as is.** Fix the branch "
              f"`{state['branch']}` (or close this and run the sync again).")
    gh("pr", "edit", state["pr"], "--body", f"{status}\n\n{entry}")
    output(passed=str(passed).lower())
    if not passed:
        sys.exit("Required checks failed; the attempt is pushed to the pull request for a person to fix")


if __name__ == "__main__":
    steps = {"prepare": prepare, "verify": verify, "finish": finish}
    if len(sys.argv) != 2 or sys.argv[1] not in steps:
        sys.exit(f"usage: sync_upstream.py {{{'|'.join(steps)}}}")
    steps[sys.argv[1]]()
