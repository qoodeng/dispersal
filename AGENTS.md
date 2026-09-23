# Dispersal work tracking

Use the Beads skill and `bd` for persistent task tracking. Run `bd prime`, `bd ready`, and `bd show <id>` when resuming. Claim work before implementation; add evidence and decisions to the issue; close only when its acceptance criteria pass.

Master epic: `disp-bi6`. Scientific/product specification: `docs/DELIVERY-PLAN.md`. Beads is authoritative for work status and dependencies; the specification remains authoritative for requirements. Website deployment does not close scientific gates.

CLI is currently `/root/.local/bin/bd` (v1.3.0). Upstream skill: `steveyegge/beads`, `plugins/beads/skills/beads`, pinned revision `71c4cd08b9991e7ba4e53822769c78337e0f4ee4`.

See `docs/BEADS.md` for persistence and restoration. Preserve the current owner-private Site audience. Do not put tracker backups into public/ or dist/.
