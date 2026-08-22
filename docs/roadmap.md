# Hoja de Ruta de Desarrollo (Roadmap): TASK BAR 4X
*El Gran Tránsito de las 15 Edades: Del Prototipo Alpha al Lanzamiento Comercial en Steam*

---

## 📌 Resumen de Sprints y Fases de Despliegue

```
  [Sprint 1: Núcleo Win32 & ECS] ➔ [Sprint 2: Dioramas & 15 Edades] ➔ [Sprint 3: Demo Open Source]
                ↓
  [Sprint 4: Steam Next Fest] ➔ [Sprint 5: Early Access & v1.0 Comercial]
```

---

## 📋 Detalle de Fases, Entregables y Criterios de Aceptación

### 🏁 Sprint 1 (v0.1 - v0.2): Refactorización Multicrate, Win32 `WM_NCHITTEST` y Simulación ECS
* **Enfoque Técnico**: Reestructurar el repositorio monolítico hacia una arquitectura desacoplada de 5 crates en Rust, eliminar el gancho global `WH_MOUSE_LL` en favor de `WM_NCHITTEST`, y estabilizar el bucle reactivo de bajo consumo.
* **Hitos & Entregables**:
  * Descomposición en `crates/sim_core`, `crates/render_engine`, `crates/platform_win32`, `crates/steam_bridge` y `src/main.rs`.
  * Implementación del procesamiento de `WM_NCHITTEST` que devuelve `HTCLIENT` en zonas interactivas y `HTTRANSPARENT` en áreas vacías (cero latencia de clics passthrough).
  * Auto-docking como AppBar mediante `SHAppBarMessage` con selección dinámica de monitor y soporte para densidad DPI (100% a 200%).
  * Motor de simulación headless determinista con ECS (`hecs`/custom) implementando las **Eras 1 a 3** (Paleolítico, Neolítico, Calcolítico).
* **Criterio de Validación**: RAM &lt; 15 MB, CPU 0.0% en reposo verificado en el Administrador de Tareas de Windows.

---

### 🎨 Sprint 2 (v0.3 - v0.4): Micro-Dioramas Históricos, Pipelines y Matriz de 90 Decisiones
* **Enfoque Jugable**: Dar vida visual a la barra de tareas con animaciones de época, flujos logísticos y el árbol completo de tecnologías binarias.
* **Hitos & Entregables**:
  * Integración del sistema de materiales del HUD que muta con la era (Piedra tosca, Cerámica, Bronce, Mármol, Roble, Hierro, Silicio, Cuántico).
  * Spritesheets en pixel art animado (16px a 24px) de cazadores, carretas de bueyes, barcazas de junco y trenes de vapor.
  * Implementación de la matriz de **90 decisiones binarias (180 tecnologías)** organizadas en las 6 disciplinas (Militar, Economía, Política, Cultura, Tecnología, Religión).
  * Sistema de Tooltips y Popovers contextuales rápidos de 1 clic sin robo de foco (`WS_EX_NOACTIVATE`).
  * Construcción progresiva de las **Grandes Maravillas** de cada época con andamiajes animados.

---

### 🌐 Sprint 3 (v0.5 - v0.6): Modo Pantalla Completa & Lanzamiento del Demo Open Source
* **Enfoque de Comunidad**: Completar el modo pantalla completa táctico y publicar la primera versión jugable para la comunidad de código abierto y plataformas indie.
* **Hitos & Entregables**:
  * Transición elástica acelerada por hardware (`F11` / `Win+Alt+X` / doble clic en el Orbe) hacia el lienzo táctico 2.5D.
  * Mapa provincial de campaña con niebla de guerra procedural, facciones de IA vecinas y rutas comerciales.
  * Sistema de **Crisis de Colapso de Era** (Invasiones bárbaras, Peste Negra, Huelgas obreras, etc.).
  * Motor de **Cálculo Determinista Offline** (hasta 24 horas de producción simulada al reabrir el juego).
  * Empaquetado y publicación del **Demo Gratuito v0.3 (Eras 1 a 6)** en **GitHub Releases** e **Itch.io**.

---

### 🌟 Sprint 4 (v0.7 - v0.8): Integración Steamworks, Audio Adaptativo y Steam Next Fest
* **Enfoque Comercial**: Conectar el juego al ecosistema de Valve y ejecutar la campaña de acumulación de listas de deseos (*Wishlists*).
* **Hitos & Entregables**:
  * Integración completa con `steamworks-rs`: 50 Logros por era, guardado en la nube (*Steam Cloud*) y estadísticas.
  * Subsistema de audio adaptativo con bandas sonoras por era (instrumentos acústicos medievales, música clásica ilustrada, jazz industrial, sintetizadores espaciales) mezcladas a **-24 dB**.
  * Publicación formal de la página de la tienda en Steam (*Coming Soon*) con tráiler cinematográfico y capturas de alta definición de las 15 edades.
  * Participación en el **Steam Next Fest** con una demo optimizada (Eras 1 a 8) apuntando a superar las **20,000 listas de deseos**.

---

### 🏆 Sprint 5 (v0.9 - v1.0): Early Access, Pulido Extremo y Lanzamiento Comercial
* **Enfoque de Producción**: Publicación del juego comercial completo y establecimiento del soporte post-lanzamiento.
* **Hitos & Entregables**:
  * Las **15 Edades completas** (desde el Paleolítico hasta la Singularidad cuántica) y meta-progresión por *Polvo de Singularidad*.
  * Integración con el *Steam Inventory Service* para drops de cosméticos negociables en el Mercado de la Comunidad.
  * Auditoría final de rendimiento con Tracy Profiler para certificar consumo mínimo y estabilidad total.
  * Creación del instalador automatizado con WiX Toolset (.msi / setup ejecutable firmado).
  * Lanzamiento en **Steam Early Access ($5.99 USD)** y posterior transición a **v1.0 ($8.99 USD)** en Steam, GOG y Epic Games Store.
