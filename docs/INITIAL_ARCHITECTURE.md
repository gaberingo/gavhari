# Initial Architecture ERP (Rust Modular Monolith + Internal Event-Driven)

## 1) Tujuan

Membangun ERP internal yang:

- Cepat dikembangkan (tanpa overhead microservices di awal).
- Modular dan mudah dipecah nanti jika diperlukan.
- Konsisten secara domain dengan komunikasi antar-modul berbasis event internal.

## 2) Prinsip Arsitektur

- Bentuk sistem: **modular monolith**.
- Integrasi antar-modul: **event bus internal** (publish/subscribe).
- Pola baca/tulis: **command-query separation ringan**.
- Konsistensi side-effect lintas modul: **outbox pattern + idempotent consumer**.
- Sumber data utama: PostgreSQL.
- Cache/queue ringan: Redis.

## 3) High-Level Component

- `Actix-Web API`
  - Auth, authorization, endpoint command/query, middleware audit.
- `Application Core (Modules)`
  - Domain use-case dan business rules per modul.
- `Internal Event Bus`
  - Dispatch event in-process antar modul.
- `Persistence`
  - Diesel + PostgreSQL.
- `Outbox Worker`
  - Membaca `outbox_events`, retry publish event yang gagal, tandai status.
- `Frontend`
  - Yew + DaisyUI + Tailwind untuk dashboard ERP.

## 4) Struktur Workspace (Awal)

```text
erp/
  Cargo.toml
  apps/
    web-api/                 # actix-web
    web-ui/                  # yew
    worker-outbox/           # retry outbox + background jobs
  crates/
    shared/                  # type umum: Error, Id, EventMeta, Result
    bus/                     # trait EventBus + in-memory impl
    infra/                   # db pool, redis, repository impl, tracing
    modules/
      auth/
      organization/
      master-data/
      purchasing/
      inventory/
      sales/
      finance/
      audit/
  migrations/
  docs/
    APP_STRUCTURE.md
    CRATE_STRUCTURE.md
    INITIAL_ARCHITECTURE.md
```

## 5) Kontrak Modul (Wajib Konsisten)

Setiap modul memiliki pola paket:

- `domain/` entity, value object, invariant.
- `application/commands` operasi tulis.
- `application/queries` operasi baca.
- `application/events` event domain.
- `application/handlers` consumer event dari modul lain.
- `infrastructure/` adapter repository, mapper DB.

Aturan:

- Modul lain tidak boleh memodifikasi state modul secara langsung.
- Side-effect lintas modul dilakukan lewat event.
- Query lintas modul boleh via read model atau view SQL, bukan mutasi langsung.

## 6) Event Envelope Standard

Contoh metadata event:

- `event_id` (UUID)
- `event_type` (string)
- `aggregate_id` (string/UUID)
- `occurred_at` (timestamp UTC)
- `version` (u16)
- `payload` (JSONB)
- `correlation_id` (opsional)
- `causation_id` (opsional)

## 7) Tabel Minimal Awal

- `users`, `roles`, `permissions`, `user_roles`
- `products`, `warehouses`, `stock_items`, `stock_movements`
- `customers`, `suppliers`
- `purchase_orders`, `purchase_order_items`
- `sales_orders`, `sales_order_items`
- `journal_entries`, `journal_lines`
- `outbox_events`
- `processed_events`
- `audit_logs`

## 8) Flow Event Inti (Contoh)

1. Purchasing approve PO -> publish `PurchaseOrderApproved`.
2. Inventory handler buat expected incoming movement.
3. Warehouse receiving selesai -> publish `GoodsReceived`.
4. Inventory update `stock_items` + `stock_movements`.
5. Finance consume `GoodsReceived` -> buat jurnal persediaan/hutang.
6. Audit module simpan trail dari command + event penting.

## 9) Reliability Guardrail

- Semua publish event bisnis penting melalui outbox dalam satu transaksi DB.
- Consumer cek `processed_events(event_id)` agar idempotent.
- Retry bertahap (exponential backoff) di worker-outbox.
- Dead-letter sederhana (status gagal permanen + alasan) untuk observability.

## 10) Fase Implementasi (8 Minggu)

1. Minggu 1: Workspace, auth dasar, org/tenant boundary, observability.
2. Minggu 2: Master data (product/customer/supplier/warehouse).
3. Minggu 3: Purchasing command + event + receiving.
4. Minggu 4: Inventory movement + stock on-hand.
5. Minggu 5: Sales order + delivery.
6. Minggu 6: Finance journal otomatis dari event transaksi.
7. Minggu 7: Dashboard Yew + report dasar.
8. Minggu 8: Hardening (outbox retry, idempotency, audit, load test ringan).

## 11) Definition of Done Arsitektur Awal

- Boundary modul jelas dan tidak ada dependency melingkar.
- Semua side-effect lintas modul melewati event.
- Outbox + idempotency aktif untuk event bisnis kritis.
- Audit log untuk command/event utama tersedia.
- Satu flow E2E berjalan: PO -> GR -> Stock -> Journal.
