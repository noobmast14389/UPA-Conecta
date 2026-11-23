# UPA Comunica

Aplicación web y programa en Solana (Devnet) para la Universidad Politécnica de Atlautla (UPAtlautla) enfocada en la gestión de reportes y objetos perdidos o encontrados, devoluciones, códigos QR, muro comunitario y operación con respaldo local[span_3](start_span)[span_3](end_span).

## Identificación
* **Nombre para el certificado:** Adrián Vergara González

## Ruta elegida
* **Modalidad:** Proyecto propio[span_4](start_span)[span_4](end_span)

## Propósito
Facilitar a la comunidad de la Universidad Politécnica de Atlautla un espacio centralizado y conectado a Solana Devnet para:
* Registrar reportes de objetos perdidos o encontrados (por ejemplo, `FOL-777` con título `'Credencial perdida'` y `estado: 0`)[span_5](start_span)[span_5](end_span).
* Consultar el estado y datos de reportes almacenados en cuenta/PDA en Solana Devnet[span_6](start_span)[span_6](end_span).
* Gestionar devoluciones, códigos QR y avisos comunitarios desde la interfaz web (`upa-comunica.html`)[span_7](start_span)[span_7](end_span).
* Operar con respaldo/restauración local cuando no hay conexión continua[span_8](start_span)[span_8](end_span).

## Despliegue (Solana Devnet)
* **Red:** Solana Devnet
* **Program ID:** [Pega aquí el Program ID de tu programa desplegado en Devnet][span_9](start_span)[span_9](end_span)
* **Wallet activa:** `2xkneht66CAu49reCdrJFa4CL2ua6ZDL1p7FRTqKsD9K`
* **Folio de prueba:** `FOL-777`
* **PDA generada:** `Ghp75Tdx78kfkos9h2ri4UZ3LHN8rWSPj9zWJYRTsknn`
* **Transacción verificable:** `4YWxhQeAf8roB5498P9yMasK56XrgJmRh7YRuhVcR4sJk3yB6o2Vrpscyi3z1iY4fyJvCjaCb6JgFpxKuuLvDdus`

## Estructura del proyecto
* `lib.rs` — Programa en Rust / Anchor con la lógica de cuentas, PDAs y validaciones del contrato.
* `client.ts` — Script cliente para interactuar con el programa en Solana Devnet y consultar cuentas.
* `anchor.test.ts` — Pruebas de integración del programa.
* `upa-comunica.html` — Interfaz web / frontend de UPA Comunica[span_10](start_span)[span_10](end_span).

## Configuración y pasos de ejecución
1. Abre `upa-comunica.html` en un navegador web moderno (Chrome, Edge, Firefox, etc.) o sírvelo en local / Codespaces conectado a Solana Devnet (no requiere hosting externo de pago)[span_11](start_span)[span_11](end_span)[span_12](start_span)[span_12](end_span).
2. Para revisar o ejecutar el programa y el cliente, abre `lib.rs`, `client.ts` y `anchor.test.ts` en Solana Playground (`beta.solpg.io`) o en tu entorno local configurado en `devnet` con el `Program ID` correspondiente[span_13](start_span)[span_13](end_span)[span_14](start_span)[span_14](end_span)[span_15](start_span)[span_15](end_span).
3. En la interfaz web, configura tu nombre de usuario y navega por las secciones (`Inicio`, `Objetos`, `Publicar`, `QR`, `Comunidad`, `Ajustes`)[span_16](start_span)[span_16](end_span).

## Forma de probarlo y evidencias
1. Ejecuta el flujo de creación del reporte con folio `FOL-777` y título `'Credencial perdida'` en Solana Devnet[span_17](start_span)[span_17](end_span).
2. Verifica la transacción `4YWxhQeAf8roB5498P9yMasK56XrgJmRh7YRuhVcR4sJk3yB6o2Vrpscyi3z1iY4fyJvCjaCb6JgFpxKuuLvDdus` y la lectura de la cuenta PDA `Ghp75Tdx78kfkos9h2ri4UZ3LHN8rWSPj9zWJYRTsknn`, confirmando `folio: 'FOL-777'`, `titulo: 'Credencial perdida'` y `estado: 0`[span_18](start_span)[span_18](end_span).
3. Prueba las funciones de la interfaz web `upa-comunica.html` y el respaldo/restauración de datos[span_19](start_span)[span_19](end_span).

## Aporte propio
Adaptación de la aplicación a la temática universitaria **UPA Comunica** para UPAtlautla, integración del flujo de reportes con semilla/folio (`FOL-777`) y PDA en Solana Devnet, verificación de lectura de estado en cadena y diseño de interfaz enfocada en la comunidad estudiantil[span_20](start_span)[span_20](end_span)[span_21](start_span)[span_21](end_span).
