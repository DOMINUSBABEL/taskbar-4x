# Plan de Despliegue de Subagentes: TASK BAR 4X Alpha
*Orquestación Multi-Agente para la construcción modular del prototipo en Rust*

---

## 🎯 Objetivo
Desplegar una red de **10 subagentes especializados** coordinados por un **Orquestador Principal** (`taskbar_4x_orchestrator`) para programar el núcleo lógico del juego, el HUD de Windows y el motor gráfico en Rust nativo, consiguiendo un ejecutable Alpha funcional y jugable.

```
                          +-------------------------------+
                          |   ORQUESTADOR DE DESARROLLO   |
                          |  (taskbar_4x_orchestrator)    |
                          +-------------------------------+
                                          |
        +-------+-------+-------+-------+--+---+-------+-------+-------+-------+
        |       |       |       |       |      |       |       |       |       |
        v       v       v       v       v      v       v       v       v       v
      [Ag-1]  [Ag-2]  [Ag-3]  [Ag-4]  [Ag-5] [Ag-6]  [Ag-7]  [Ag-8]  [Ag-9] [Ag-10]
      Win32   Simul   Relic  Investi  Gráfic   UI    Cursor  Steam   Sonido  Utils
```

---

## 👥 Asignación de Roles y Funciones de Desarrollo

### 1. Agente de Ventana y Sistema Win32 (`developer_win32_window`)
*   **Archivo a generar**: `src/main.rs`
*   **Responsabilidad**: Crear el bucle WndProc, inicializar la ventana y registrarla como AppBar de Windows usando `SHAppBarMessage`. Configurar los estilos extendidos `WS_EX_NOACTIVATE` y `WS_EX_TOOLWINDOW`.

### 2. Agente de Simulación de Estrategia (`developer_simulation`)
*   **Archivo a generar**: `src/simulation.rs`
*   **Responsabilidad**: Diseñar el bucle lógico del imperio (ticks físicos, acumulación de recursos por segundo, crecimiento de población y transiciones de eras).

### 3. Agente de Reliquias y Artefactos (`developer_relics`)
*   **Archivo a generar**: `src/relics.rs`
*   **Responsabilidad**: Programar la base de datos de reliquias de la partida y artefactos del vacío persistentes, con sus modificadores porcentuales y efectos sobre la economía.

### 4. Agente de Tecnologías e Investigaciones (`developer_research`)
*   **Archivo a generar**: `src/research.rs`
*   **Responsabilidad**: Estructurar las 90 elecciones binarias de las 15 edades, validando dependencias y aplicando dinámicamente los bufos y retos a las estadísticas del imperio.

### 5. Agente de Renderizado Direct2D (`developer_renderer`)
*   **Archivo a generar**: `src/renderer.rs`
*   **Responsabilidad**: Configurar la canalización gráfica en Direct2D para el HUD de 48px y el panel a pantalla completa, optimizando el dibujado por hardware sin desperdicio de GPU.

### 6. Agente de Interfaz y Capa de UI (`developer_ui`)
*   **Archivo a generar**: `src/ui.rs`
*   **Responsabilidad**: Programar la cuadrícula del HUD, el dibujo de textos esmerilados, popovers explicativos y botones de interacciones.

### 7. Agente de Entrada y Clics Passthrough (`developer_input`)
*   **Archivo a generar**: `src/input.rs`
*   **Responsabilidad**: Detectar coordenadas de ratón en tiempo real y alternar dinámicamente `WS_EX_TRANSPARENT` para permitir hacer clics a través de zonas libres de la barra. Capturar el atajo `Win + Alt + X`.

### 8. Agente de Conexión Steamworks (`developer_steam`)
*   **Archivo a generar**: `src/steam.rs`
*   **Responsabilidad**: Integrar el SDK de Steam mediante `steamworks-rs`. Configurar guardado en la nube y triggers para logros de era.

### 9. Agente de Audio y Efectos (`developer_audio`)
*   **Archivo a generar**: `src/audio.rs`
*   **Responsabilidad**: Implementar el reproductor de sonido local mapeando pistas y efectos según la era. Controlar niveles de volumen estrictos (-22dB para música, -6dB para transiciones).

### 10. Agente de Ofuscación y Utilidades (`developer_utils`)
*   **Archivo a generar**: `src/utils.rs`
*   **Responsabilidad**: Desarrollar la estructura `ValorProtegido` con encriptación XOR para evitar cheats, y configurar Crashpad para recolección de minidumps en caso de error.

---

## 🛠️ Ejecución y Cronograma del Orquestador
1. **Fase de Consolidación de Interfaces (Día 1)**: El Orquestador define las firmas de funciones y las estructuras de datos compartidas en un archivo `src/types.rs`.
2. **Despliegue de Agentes Modulares (Día 1-2)**: Los 10 subagentes se instancian en paralelo para rellenar el código fuente de sus respectivos módulos.
3. **Integración y Compilación Cruzada (Día 3)**: El Orquestador depura errores de compilación del compilador de Rust (`cargo check`), resolviendo conflictos de dependencias y optimizando el consumo de RAM.
4. **Distribución en el Portal (Día 3)**: Empaquetamiento final del instalador MSI para subirlo a la ruta de descargas corporativa.
