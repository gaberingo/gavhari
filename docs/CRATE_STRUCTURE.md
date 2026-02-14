# Struktur Tiap Crate

Dokumen ini menjadi acuan struktur internal setiap crate pada workspace ERP modular monolith event-driven.

## 1) `crates/shared`
Tujuan: tipe umum lintas seluruh sistem.

```text
crates/shared/
  Cargo.toml
  src/
    lib.rs
    event.rs        # event envelope + metadata + trait event
    error.rs        # error umum aplikasi
    result.rs       # alias result type
    types.rs        # value object umum (id, timestamp, money, dll)
```

Aturan:
- Tidak boleh import modul domain spesifik.
- Stabil, backward compatible semampunya.

## 2) `crates/bus`
Tujuan: kontrak dan implementasi event bus internal.

```text
crates/bus/
  Cargo.toml
  src/
    lib.rs
    traits.rs       # EventBus, EventHandler
    in_memory.rs    # in-process publish/subscribe
    router.rs       # registry event_type -> handler
```

Aturan:
- Tidak menyimpan business rule.
- Fokus pada delivery event internal proses.

## 3) `crates/infra`
Tujuan: adapter teknis (DB, Redis, observability, outbox/idempotency store).

```text
crates/infra/
  Cargo.toml
  src/
    lib.rs
    config.rs
    observability.rs
    db/
      mod.rs
      pool.rs
      schema.rs
    redis/
      mod.rs
      client.rs
    outbox/
      mod.rs
      repo.rs
    idempotency/
      mod.rs
      repo.rs
```

Aturan:
- Tidak menaruh domain invariant di sini.
- Hanya implementasi detail teknis interface dari module.

## 4) `crates/modules/auth`
Tujuan: autentikasi/otorisasi pengguna.

```text
crates/modules/auth/
  Cargo.toml
  src/
    lib.rs
    domain/
      mod.rs
      user.rs
      role.rs
      permission.rs
    application/
      commands/
      queries/
      events/
      handlers/
    infrastructure/
      mod.rs
      repo.rs
```

## 5) `crates/modules/organization`
Tujuan: boundary organisasi/tenant/company.

```text
crates/modules/organization/
  Cargo.toml
  src/
    lib.rs
    domain/
      mod.rs
      organization.rs
    application/
      commands/
      queries/
      events/
      handlers/
    infrastructure/
      mod.rs
      repo.rs
```

## 6) `crates/modules/master-data`
Tujuan: data referensi inti (product, supplier, customer, warehouse).

```text
crates/modules/master-data/
  Cargo.toml
  src/
    lib.rs
    domain/
      mod.rs
      product.rs
      customer.rs
      supplier.rs
      warehouse.rs
    application/
      commands/
      queries/
      events/
      handlers/
    infrastructure/
      mod.rs
      repo.rs
```

## 7) `crates/modules/purchasing`
Tujuan: alur purchase order hingga approval.

```text
crates/modules/purchasing/
  Cargo.toml
  src/
    lib.rs
    domain/
      mod.rs
      purchase_order.rs
    application/
      commands/
      queries/
      events/
      handlers/
    infrastructure/
      mod.rs
      repo.rs
```

## 8) `crates/modules/inventory`
Tujuan: stock on-hand, movement, receiving.

```text
crates/modules/inventory/
  Cargo.toml
  src/
    lib.rs
    domain/
      mod.rs
      stock_item.rs
      stock_movement.rs
    application/
      commands/
      queries/
      events/
      handlers/
    infrastructure/
      mod.rs
      repo.rs
```

## 9) `crates/modules/sales`
Tujuan: sales order, delivery, dan status fulfilment.

```text
crates/modules/sales/
  Cargo.toml
  src/
    lib.rs
    domain/
      mod.rs
      sales_order.rs
    application/
      commands/
      queries/
      events/
      handlers/
    infrastructure/
      mod.rs
      repo.rs
```

## 10) `crates/modules/finance`
Tujuan: jurnal dan posting akuntansi dari event transaksi.

```text
crates/modules/finance/
  Cargo.toml
  src/
    lib.rs
    domain/
      mod.rs
      journal_entry.rs
    application/
      commands/
      queries/
      events/
      handlers/
    infrastructure/
      mod.rs
      repo.rs
```

## 11) `crates/modules/audit`
Tujuan: audit trail command/event penting.

```text
crates/modules/audit/
  Cargo.toml
  src/
    lib.rs
    domain/
      mod.rs
      audit_log.rs
    application/
      commands/
      queries/
      events/
      handlers/
    infrastructure/
      mod.rs
      repo.rs
```

## Konvensi Umum Semua Module Crate
- `domain/`: invariant dan rule inti.
- `application/commands`: write use case.
- `application/queries`: read use case.
- `application/events`: event domain yang dipublish.
- `application/handlers`: consumer event dari modul lain.
- `infrastructure/`: adapter repository/external.

## Dependency Rule (Penting)
- `apps/*` -> boleh depend ke `modules`, `infra`, `shared`, `bus`.
- `modules/*` -> boleh depend ke `shared` (+ interface abstraction), tidak boleh ke `apps/*`.
- `infra` -> implement interface dari module, tidak memegang rule domain.
- `shared` -> paling bawah, tidak depend ke module.
