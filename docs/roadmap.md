# Hoja de Ruta de Desarrollo (Roadmap): TASK BAR 4X
*Hacia la versión 1.0: Del prototipo Alpha al lanzamiento comercial en Steam*

---

## 📌 Resumen de Versiones y Milestones

```
  v0.1 (Alpha)  --->  v0.2 (DWM/Win32)  --->  v0.3 (Eras/Logística)
       |
  v0.4 (Táctico) --->  v0.5 (Rogue-like) --->  v0.6 (IA/Combate)
       |
  v0.7 (Audio)   --->  v0.8 (Steamworks) --->  v0.9 (Optimización) --->  v1.0 (Lanzamiento)
```

---

## 📋 Detalle de Fases y Entregables

### 🏁 v0.1: Prototipo Alpha (Actual)
*   **Enfoque**: Establecer la arquitectura del proyecto en Rust, el plan de orquestación técnica, el diseño visual consolidado y la página web corporativa para descargas provisionales de testeo.
*   **Hitos**:
    *   Diseño del Master Blueprint.
    *   Inicialización del repositorio Git y publicación en GitHub.
    *   Estructura web de descarga del instalador en [taskbar-4x.component.ts](file:///C:/Users/jegom/BABYLON_REPO/src/app/pages/taskbar-4x.component.ts).

---

### 🖥️ v0.2: Control Físico de Ventana y Capa Passthrough
*   **Enfoque**: Conseguir la coexistencia perfecta de la interfaz del HUD con el sistema operativo Windows sin interferir en la productividad del usuario.
*   **Hitos**:
    *   Registro dinámico del HUD mediante la API `SHAppBarMessage` en los bordes de la pantalla.
    *   Implementación de detección de cursor en tiempo real para alternar `WS_EX_TRANSPARENT` (clics del ratón traspasan las zonas de aire de la barra).
    *   Alineación y escalado de píxeles automático para monitores con diferente densidad (DPI) y soporte multi-pantalla.
    *   Implementación del atajo de teclado global `Win + Alt + X` para la transición de ventana.

---

### 🪵 v0.3: Simulación Lógica e Investigaciones de Eras
*   **Enfoque**: Estructurar las 15 Edades históricas y la matriz completa de decisiones científicas.
*   **Hitos**:
    *   Implementación del bucle lógico del imperio (ticks físicos de recursos e incrementales).
    *   Desarrollo del motor de reglas para las **90 elecciones binarias**, aplicando los bufos y retos correspondientes a las estadísticas del imperio.
    *   Sistemas de pipelines logísticos 1D animados en la barra (recolección y transporte de recursos).

---

### 🗺️ v0.4: Modo Pantalla Completa y Mapa Táctico 2.5D
*   **Enfoque**: Desarrollar el modo de juego inmersivo profundo.
*   **Hitos**:
    *   Renderizado en pantalla completa mediante la integración de la canalización Direct2D/wgpu.
    *   Visualización del mapa de campaña con cuadrículas zonales, fronteras de facciones de IA y flotas en movimiento.
    *   Interfaz del gabinete diplomático para pactos y panel detallado de producción local.

---

### 🎲 v0.5: Motor Rogue-like y Meta-Progresión
*   **Enfoque**: Añadir el bucle de rejugabilidad pasiva.
*   **Hitos**:
    *   Generador procedimental de redes de autopistas y planetas para el mapa al iniciar cada run.
    *   Lógica de colapso y extinción del imperio (conversión del total acumulado a la divisa *Polvo de Singularidad*).
    *   Menú permanente del Árbol de Legado Cósmico para la compra de mejoras persistentes (ej. incremento de energía por CPU).
    *   Capa de carga de reliquias y artefactos con efectos temporales de partida.

---

### ⚔️ v0.6: Inteligencia Artificial y Simulación de Combate
*   **Enfoque**: Dinamizar la barra de tareas mediante conflictos territoriales y asaltos bárbaros.
*   **Hitos**:
    *   Lógica de combate lineal 1D (unidades de infantería, naves y defensas destruyéndose físicamente sobre el HUD).
    *   Comportamiento de la IA de imperios rivales (incursiones periódicas, presiones comerciales y declaraciones de guerra).
    *   Alertas visuales de crisis por era.

---

### 🔊 v0.7: Paisaje Sonoro y Audio Adaptativo
*   **Enfoque**: Evitar la monotonía acústica mediante variedad de música y atenuaciones adaptadas al género del juego.
*   **Hitos**:
    *   Mapeo dinámico de pistas de la biblioteca local (`C:\Users\jegom\shorts_project\music\`) por atmósfera e historia:
        *   *Medieval*: `Moorland.mp3`, `Rites.mp3`.
        *   *Futurista*: `Cipher2.mp3`, `Future Gladiator.mp3`.
        *   *Combate*: `Volatile Reaction.mp3`, `Severe Tire Damage.mp3`.
        *   *Meme/Tips*: `Sneaky Snitch.mp3`.
    *   Ajuste estricto de volumen: Música de fondo a **-22dB / -25dB**, efectos de transición (Whoosh/Pop) a **-6dB**, y voz en off (Jorge TTS) a volumen de referencia **1.0**.

---

### 🚀 v0.8: Integración con el Ecosistema de Steam
*   **Enfoque**: Conectar el juego con las APIs comerciales de la plataforma de Valve.
*   **Hitos**:
    *   Integración del SDK mediante `steamworks-rs`.
    *   Sincronización automatizada de guardados en la nube (*Steam Cloud*) a carpetas locales seguras.
    *   Activación de logros de progresión por era.
    *   Configuración del *Steam Inventory Service* para drops de skins del orbe y naves comerciables en el mercado comunitario.

---

### ⚡ v0.9: Pulido, Seguridad y Optimización de Sistemas
*   **Enfoque**: Asegurar la robustez del software y el consumo ultra-ligero exigido.
*   **Hitos**:
    *   Integración del manejador de errores *Crashpad* para capturar volcados minidump.
    *   Implementación de la estructura de datos `ValorProtegido` con encriptación XOR en memoria contra herramientas de trucos (antihack).
    *   Optimización del bucle `WaitMessage()` para asegurar **CPU ~0.0%** en reposo y **RAM < 15MB**.

---

### 📦 v1.0: Lanzamiento Comercial de Producción
*   **Enfoque**: Publicación del juego al público en general.
*   **Hitos**:
    *   Compilación final optimizada para lanzamiento (`cargo build --release`).
    *   Empaquetado definitivo con el instalador de WiX Toolset para distribución desatendida.
    *   Actualización de la web corporativa de Babylon.ia redireccionando los botones de descarga del Alpha hacia la tienda de Steam.
