# UPA Comunica

Aplicación web para la **Universidad Politécnica de Atlautla (UPAtlautla)** enfocada en la gestión de objetos perdidos y encontrados, registro de devoluciones, generación de códigos QR, muro comunitario, música ambiental y operación en modo sin conexión.

---

## Ruta elegida
- **Modalidad:** Proyecto propio

---

## Propósito
Facilitar a la comunidad de la **Universidad Politécnica de Atlautla** un espacio centralizado y rápido para:
- Reportar objetos perdidos o encontrados e iniciar en cero (`items: []`, `communityPosts: []`) sin datos de ejemplo.
- Registrar devoluciones a su dueño con seguimiento claro.
- Generar códigos QR de apoyo para objetos o identificación rápida.
- Publicar mensajes y avisos en el muro de la comunidad.
- Operar en **modo sin conexión** con respaldo y restauración de datos en formato JSON.

---

## Configuración
1. Asegúrate de contar con el archivo principal **`upa-comunica.html`**.
2. No requiere instalación de servidor ni dependencias externas obligatorias para su uso en navegador.
3. Una vez abierta la aplicación, entra a **Ajustes** para configurar tu **Nombre de usuario** (usado en el saludo *"Bienvenido/a, [Nombre]"*), ajustar la música ambiental y administrar el modo sin conexión o los respaldos JSON.

---

## Pasos de ejecución
1. Abre **`upa-comunica.html`** en un navegador web moderno (Chrome, Edge, Firefox, etc.).
2. Espera la pantalla de carga verde de 10 segundos con animación de lupa y temporizador (o usa el botón para avanzar si está disponible).
3. En la pestaña **Inicio**, revisa el saludo personalizado y la frase motivadora rotativa (que cambia al abrir la app o volver a **Inicio**).
4. Navega por los apartados de la barra inferior (**Inicio**, **Objetos**, **Publicar**, **QR**, **Comunidad**, **Ajustes**) para probar cada función.

---

## Aporte propio
En este proyecto realicé y personalicé:
- La identidad visual de **UPA Comunica** para la **Universidad Politécnica de Atlautla** con tonos rojo carmín y verde esmeralda y el emblema del gorrión.
- El arranque limpio en cero sin registros precargados, saludo personalizado en **Ajustes** y frases motivadoras rotativas en **Inicio**.
- La pantalla de carga verde de 10 segundos con lupa y temporizador en tiempo real, junto con los iconos visibles y animados en la barra de navegación inferior.
- La integración del **modo sin conexión** con detección de red, exportación/importación de respaldo en JSON y música ambiental.

---

## Forma de probarlo
1. **Saludo y frases:** Cambia tu nombre en **Ajustes**, ve a **Inicio** y sal/regresa a **Inicio** para verificar que cambie la frase motivadora y se conserve tu nombre.
2. **Objetos y devoluciones:** Crea un reporte de objeto perdido/encontrado y márcalo como devuelto; recarga la página para comprobar que se guarda en `localStorage`.
3. **QR y Comunidad:** Genera un código QR y publica una nota en **Comunidad**.
4. **Modo sin conexión y respaldo JSON:** Activa el modo sin conexión o desconecta la red, exporta el respaldo `.json`, borra o modifica datos y vuelve a importarlo para verificar la recuperación.

---

## Identificación
- **Nombre para el certificado:** Adrian Vergara González
