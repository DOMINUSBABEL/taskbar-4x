# Especificación Técnica de UI/UX: TASK BAR 4X
## Sistema de Interfaz Histórico Evolutivo y Diorama Viviente para Estrategia 4X

Este documento define la arquitectura visual, el diseño de interacción, las especificaciones técnicas y las pautas de experiencia de usuario (UI/UX) para **TASK BAR 4X**, un videojuego de estrategia y civilización histórica que opera de forma dual: como una barra HUD compacta (48px) de tránsito histórico integrada en el flujo de trabajo diario y como una aplicación inmersiva a pantalla completa.

---

## 1. Filosofía de Diseño y Principios Rectores

El diseño de TASK BAR 4X se basa en tres pilares fundamentales de interacción:

1. **Cero Obstrucción del Flujo de Trabajo (Diseño de Coexistencia Positiva):** En su modo Barra de Tareas, el juego convive armoniosamente con las herramientas de productividad del usuario (IDEs, suites ofimáticas, navegadores). No roba el foco del teclado involuntariamente (`WS_EX_NOACTIVATE`), ni ocluye elementos críticos del sistema operativo.
2. **Diorama Histórico Viviente y Mutación de Materiales:** La barra no es un contenedor estático ni genéricamente cyberpunk. **Muta orgánicamente de aspecto, textura y paleta de color con cada una de las 15 Edades históricas**: piedra tallada, arcilla cocida, bronce bruñido, mármol clásico, madera de roble heráldica, hierro remachado a vapor, aluminio industrial y cristal cuántico.
3. **Densidad y Claridad de Información (Microinteracciones de 1 Clic):** Exponer datos complejos de un 4X (producción, investigación, fronteras territoriales y crisis) en espacios reducidos mediante jerarquía visual estricta, tooltips contextuales de un solo clic y retroalimentación táctil inmediata.

---

## 2. Modo Barra de Tareas (48px HUD & Diorama)

El modo Barra de Tareas es la interfaz de juego activa por defecto durante el uso general del ordenador. Se presenta como una franja horizontal fija de **48px de altura** (con opciones configurables a 32px y 64px).

### 2.1. Layout General y Distribución de Zonas

```
+-------------------------------------------------------------------------------------------------------------------------------+
| ZONA 1 (130px) | ZONA 2 (180px)          | ZONA 3 (Flexible)                 | ZONA 4 (Flexible)         | ZONA 5 (130px)     |
| Época y Orbe   | Recursos de la Era      | Diorama Viviente & Convoyes       | Frontera 1D & Expedición  | Sistema / Modo F11 |
+-------------------------------------------------------------------------------------------------------------------------------+
```

#### Detalle de Dimensiones y Espaciados:
* **Altura del HUD:** `48px` fijos (modo estándar).
* **Márgenes externos (Paddings laterales):** `8px` izquierdo y derecho.
* **Margen interno de celda (Cell Padding):** `4px` arriba/abajo, `8px` izquierda/derecha.
* **Espaciado entre elementos (Gap):** `6px` de separación entre los bloques de recursos y controles.
* **Radio de borde (Border Radius):** Adaptativo según la era (`2px` en Piedra, `0px` en Mármol Clásico, `6px` en Industrial, `10px` en Singularidad).

---

### 2.2. Desglose de las Zonas de la Barra de Tareas

#### Zona 1: Orbe del Imperio y Época Actual (Ancho Fijo: 130px)
* **Orbe Imperial (36px x 36px):** Insignia viva de la civilización que muestra el avatar del líder/edad actual con una corona circular que se rellena con el progreso hacia la siguiente Era. Hacer doble clic o presionar `F11` abre la vista táctica de pantalla completa.
* **Nombre de la Era:** Tipografía de época en `9px` (ej. `EDAD DEL BRONCE`, `ERA INDUSTRIAL`, `SINGULARIDAD`).
* **Año de Civilización:** Contador de tiempo histórico transcurrido.

#### Zona 2: Recursos Dinámicos de Época (Ancho Fijo: 180px)
Los recursos expuestos evolucionan con la tecnología de la civilización:
* **Edad Antigua (1-4):** Comida 🌾, Madera 🪵, Piedra/Cobre ⛏️, Fe 🕯️.
* **Edad Clásica y Medieval (5-8):** Alimentos 🥖, Hierro/Acero ⚔️, Oro 🪙, Cultura 📜.
* **Edad Moderna e Industrial (9-12):** Carbón 🚂, Maquinaria ⚙️, Capital 💰, Ciencia 🔬.
* **Edad Espacial y Singularidad (13-15):** Energía Solar ⚡, Antimateria 🌌, Cómputo Cuántico ⚛️.

Cada recurso muestra su valor neto y su tasa de producción por segundo con colorimetría de tendencia (Verde = Crecimiento, Rojo = Déficit).

#### Zona 3: Diorama Viviente & Pipelines de Convoyes (Ancho Flexible, Central)
El centro de la barra es un **lienzo animado en pixel art** donde se observa el flujo logístico de la sociedad:
* **Paleolítico/Neolítico:** Aldeanos cargando cestas de bayas y troncos hacia la fogata central.
* **Antigüedad/Medieval:** Carretas de bueyes y barcazas transportando trigo y bloques de piedra hacia la cantera o la catedral en construcción.
* **Industrial:** Locomotoras a vapor humeantes cruzando vías férreas y llevando carbón a las fundiciones.
* **Futurista:** Drones de carga y cápsulas maglev flotando sobre guías electromagnéticas.

#### Zona 4: Frontera Lineal 1D y Expediciones (Ancho Flexible)
Visualización territorial simplificada:
* Puntos explorados (`●`), depósitos de recursos descubiertos (`◆`), asentamientos aliados (`▲`) y asentamientos hostiles o campamentos bárbaros (`✖`).
* El sprite del explorador o la patrulla militar se desplaza físicamente sobre la línea descubriendo nuevas celdas.

#### Zona 5: Sistema, Reloj y Selector de Modo (Ancho Fijo: 130px)
* **Reloj del Sistema / Tiempo de Juego:** Hora local o temporizador de descanso laboral estilo Pomodoro.
* **Botón de Modo Táctico:** Icono de maximizar/pantalla completa (`32px x 32px`).
* **Menú Rápido de Opciones:** Control de volumen sutil, pausa del juego y configuración de anclaje.

---

## 3. Modo Pantalla Completa (Vista Táctica Cartográfica)

Al desplegarse mediante `F11`, `Win + Alt + X` o doble clic en el Orbe, el juego expande su lienzo a una vista de Gran Estrategia clásica organizada en una rejilla fluida de 12 columnas:

```
+-------------------------------------------------------------------------------------------------------+
|  ZONA SUPERIOR (48px) - HUD Consolidado de Recursos, Estabilidad y Fe/Ciencia                        |
+-------------------------------------------------------------------------------------------------------+
|   PANEL IZQUIERDO     |                                                       |    PANEL DERECHO      |
|   (300px)             |                     LIENZO CENTRAL                    |    (300px)            |
|                       |                   DE VISTA CARTOGRÁFICA               |                       |
|   * Árbol de          |                       PROVINCIAL                      |    * Gabinete         |
|     90 Decisiones     |                                                       |      Diplomático      |
|   * Maravillas en     |                   * Mapa Geográfico 2.5D              |    * Reclutamiento de |
|     Construcción      |                   * Frentes de Batalla                |      Ejércitos        |
|                       |                   * Rutas de Comercio                 |    * Estado de Crisis |
+-------------------------------------------------------------------------------------------------------+
|  ZONA INFERIOR (48px) - Registro Histórico de Eventos y Botón de Retorno a Barra de Tareas            |
+-------------------------------------------------------------------------------------------------------+
```

---

## 4. Sistema de Materiales Mutables por Era (UI Chameleon)

La paleta cromática y las texturas de borde del HUD se adaptan al periodo histórico para maximizar la inmersión:

| Periodo | Material del Marco | Color Primario | Color de Acento | Iconografía |
| :--- | :--- | :--- | :--- | :--- |
| **Eras 1-3** | Piedra tosca y arcilla | `#292524` (Piedra) | `#d97706` (Fuego ámbar) | Tallado rústico, puntas de flecha y madera |
| **Eras 4-6** | Bronce bruñido y mármol | `#78350f` (Bronce) | `#f59e0b` (Oro imperial) | Columnas clásicas, espadas y pergaminos |
| **Eras 7-8** | Roble heráldico y hierro | `#1c1917` (Roble) | `#dc2626` (Rojo carmesí) | Escudos, cruces góticas y forja |
| **Eras 9-11** | Latón, caoba y hierro a vapor | `#451a03` (Caoba) | `#0284c7` (Azul vapor) | Engranajes, manómetros y compases |
| **Eras 12-15** | Aluminio, cerámica y cristal | `#0f172a` (Obsidiana) | `#00F0FF` (Cian cuántico) | Circuitos, órbitas y prismas de luz |

---

## 5. Ergonomía y Gestión de Entrada con `WM_NCHITTEST`

* **Cero Latencia en Passthrough:** El cálculo de colisión se realiza en tiempo constante evaluando rectángulos 2D dentro del procedimiento `WndProc`.
* **Compatibilidad Multimonitor:** Capacidad de anclarse a la pantalla principal o a pantallas secundarias con persistencia en el archivo de configuración `settings.json`.
* **Modo Suspensión por Aplicaciones 3D:** Si una aplicación 3D o juego toma la pantalla completa (`GetForegroundWindow`), TASK BAR 4X entra en reposo absoluto reduciendo los FPS a 0 para no competir por memoria VRAM ni ciclos de CPU.
