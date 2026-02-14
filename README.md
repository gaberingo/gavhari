# ERP Workspace Scaffold

Scaffold awal untuk ERP dengan pendekatan Rust modular monolith + event-driven internal.

## Struktur
- `apps/web-api`: entrypoint API (Actix-Web, placeholder).
- `apps/web-ui`: entrypoint UI (Yew, placeholder).
- `apps/worker-outbox`: worker background untuk outbox/retry (placeholder).
- `crates/shared`: type umum (`EventMeta`, error).
- `crates/bus`: kontrak event bus internal.
- `crates/infra`: infrastruktur lintas modul.
- `crates/modules/*`: boundary domain per modul ERP.

## Cek Build
```bash
cargo check --offline
```
