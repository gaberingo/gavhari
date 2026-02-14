# Struktur `web-api` dan `web-ui`

Dokumen ini melengkapi `docs/CRATE_STRUCTURE.md` khusus untuk crate aplikasi di `apps/`.

## 1) `apps/web-api`
Tujuan: adapter HTTP tipis untuk command/query, bukan tempat business logic.

```text
apps/web-api/
  Cargo.toml
  src/
    main.rs
    app.rs                  # bootstrap actix app + route wiring
    state.rs                # AppState: db pool, bus, config, services
    config.rs               # env/config loader
    error.rs                # HTTP error mapping
    http/
      mod.rs
      health.rs             # /health, /health/live, /health/ready
      purchase_order.rs     # endpoint command/query PO
      command_status.rs     # endpoint tracking status command async
    middleware/
      mod.rs
      request_id.rs
      auth.rs
      audit.rs
      tracing.rs
```

Aturan:
- Handler hanya validasi ringan + call use case module.
- Tidak menyimpan rule domain di layer HTTP.
- Response command event-driven idealnya `202 Accepted` + `command_id`.

## 2) `apps/web-ui`
Tujuan: renderer UI (dashboard ERP) yang mengirim intent command dan membaca read model.

```text
apps/web-ui/
  Cargo.toml
  Trunk.toml
  index.html
  input.css
  output.css
  daisyui.mjs
  daisyui-theme.mjs
  src/
    main.rs
    app.rs
    routes.rs
    pages/
      dashboard.rs
      purchase_orders.rs
      command_status.rs
    components/
      navbar.rs
      sidebar.rs
      data_table.rs
      status_badge.rs
      loading_state.rs
    services/
      api_client.rs         # wrapper HTTP client
      commands.rs           # POST command endpoint
      queries.rs            # GET read model/projection
      polling.rs            # polling command status/read model
    models/
      command_status.rs
      purchase_order_view.rs
    state/
      mod.rs
      command_tracker.rs
  dist/                     # generated assets (build output)
```

Aturan:
- UI tidak memegang business rule.
- UI kirim command, lalu render status/projection.
- Polling harus anti-overlap (request berikutnya setelah request sebelumnya selesai).

## 3) Alur Antar App (Ringkas)
1. UI kirim command (`POST`) ke `web-api`.
2. `web-api` validasi ringan + invoke module use case.
3. Module commit state + tulis outbox event.
4. API balas `202` dengan `command_id`.
5. UI polling `command_status` dan/atau read model.
6. UI render final state setelah projection terupdate.
