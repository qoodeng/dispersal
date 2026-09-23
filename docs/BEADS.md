# Beads tracking

The master plan is represented by epic `disp-bi6`, ten work-package epics and twenty actionable tasks, with acceptance criteria and prerequisite edges. No scientific requirement was closed during migration. `disp-bi6.2.1` is active: acquire paired resource and relocation observations.

Installed upstream skill at revision `71c4cd08b9991e7ba4e53822769c78337e0f4ee4`, from https://github.com/steveyegge/beads/tree/71c4cd08b9991e7ba4e53822769c78337e0f4ee4/plugins/beads/skills/beads . CLI v1.3.0 release SHA-256 was verified against upstream checksums before execution. `bd prime` provides current command guidance; the skill's older version metadata is not the CLI reference.

## Persistence

Live state is embedded Dolt in `.beads/embeddeddolt/`. There is no configured Dolt remote. `.beads/issues.jsonl` is a reviewable export, not the full backup. `.beads/database-backup.tar.gz` contains a native `bd backup sync` destination including database history, committed alongside source for durable recovery.

After issue changes, run `bd backup sync`, refresh the archive from its configured destination and run `bd export --all -o .beads/issues.jsonl`. Commit and push these with the source using the existing authorized Sites repository workflow. Do not claim automatic cross-machine synchronization.

For a fresh checkout, extract the archive into a temporary directory and run `bd backup restore <absolute-path-to-extracted-database-backup>`. Do not initialize a replacement database over existing state. Verify with `bd list --json` and `bd ready`.

Skill instructions were read directly in this session; automatic discovery of the installed skill is available on the next turn. Installation/setup was authorized by the user's explicit request to use Beads.
