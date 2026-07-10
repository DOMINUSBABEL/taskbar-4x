# Especificación Técnica de UI/UX: TASK BAR 4X
## Sistema de Interfaz Compacto y de Alto Rendimiento para Estrategia 4X

Este documento define la arquitectura visual, el diseño de interacción, las especificaciones técnicas y las pautas de experiencia de usuario (UI/UX) para **TASK BAR 4X**, un videojuego de estrategia y gestión imperial que opera de forma dual: como una barra de herramientas HUD compacta (48px) integrada en el flujo de trabajo diario y como una aplicación inmersiva a pantalla completa.

---

## 1. Filosofía de Diseño y Principios Rectores

El diseño de TASK BAR 4X se basa en tres pilares fundamentales de interacción:

1. **Cero Obstrucción del Flujo de Trabajo (Diseño Parasitario Positivo):** En su modo Barra de Tareas, el juego debe convivir armoniosamente con las herramientas de productividad del usuario. No debe robar el foco del teclado involuntariamente, ni ocluir elementos críticos del sistema operativo (como la barra de tareas de Windows o el Dock de macOS).
2. **Densidad de Información de Nivel AAA:** Exponer datos complejos de un juego 4X (economía, diplomacia, investigación, operaciones militares) en espacios extremadamente reducidos mediante el uso de jerarquía visual estricta, iconografía codificada por colores y microinteracciones de precisión.
3. **Estética Sci-Fi "Glassmorphic" Avanzada:** Utilizar capas translúcidas de aspecto vítreo que actúen como "cristales de instrumentación espacial". Esto permite que el fondo del escritorio del usuario se intuya sutilmente a través de la interfaz, reduciendo la fatiga visual y mejorando la integración estética.

---

## 2. Modo Barra de Tareas (48px HUD)

El modo Barra de Tareas es la interfaz de juego activa por defecto durante el uso general del ordenador. Se presenta como una franja horizontal fija de **48px de altura**.

### 2.1. Layout General y Distribución de Zonas

La barra de tareas se divide en los siguientes bloques funcionales fijos distribuidos horizontalmente sobre una rejilla de píxeles perfecta:

```
+-------------------------------------------------------------------------------------------------------------------------------+
| ZONA 1 (140px) | ZONA 2 (Flexible)                       | ZONA 3 (Flexible)                 | ZONA 4 (220px) | ZONA 5 (160px)|
| Imperio / Turno| Panel de Recursos del Imperio            | Hilos de Producción y Eventos     | Alertas/Flotas | Sistema/Modos |
+-------------------------------------------------------------------------------------------------------------------------------+
```

#### Detalle de Dimensiones y Espaciados:
*   **Altura del HUD:** `48px` fijos.
*   **Márgenes externos (Paddings laterales):** `8px` izquierdo y derecho.
*   **Margen interno de celda (Cell Padding):** `4px` arriba/abajo, `8px` izquierda/derecha.
*   **Espaciado entre elementos (Gap):** `6px` de separación entre los bloques de recursos y controles.
*   **Radio de borde (Border Radius):** `8px` para todos los paneles interactivos integrados en el HUD.

---

### 2.2. Desglose de las Zonas de la Barra de Tareas

#### Zona 1: Control del Imperio y Turno (Ancho Fijo: 140px)
*   **Botón de Ciclo/Turno (40px x 40px):** Un botón circular central que indica visualmente el progreso del turno actual (en tiempo real o por turnos activos). Una barra de progreso circular perimetral (espesor de `2px`) se rellena dinámicamente.
*   **Logotipo del Imperio (32px x 32px):** Icono personalizado que sirve como botón de acceso al menú de configuración rápida.
*   **Indicador de Fase:** Texto ultracompacto en tipografía *Outfit Medium* de `9px` con el número de turno actual (ej. `TURNO 142`).

#### Zona 2: Panel de Recursos (Ancho Flexible, Alineado a la Izquierda)
Muestra los recursos globales del imperio de forma modular. Cada recurso ocupa un bloque de `64px` a `80px` de ancho y consta de:
*   **Icono del Recurso (16px x 16px):** Con color identificativo de alta visibilidad (ver sección 4.1).
*   **Valor Neto:** Tipografía *Outfit SemiBold* de `11px`, color blanco de alta intensidad.
*   **Tendencia por Turno:** Tipografía *Inter Medium* de `9px`, color verde esmeralda si es positiva (ej. `+14`) o rojo carmesí si es negativa (ej. `-3`).

*Orden de Recursos de Izquierda a Derecha:*
1.  **Créditos de Energía (EC)**
2.  **Minerales de Titanio (MIN)**
3.  **Materia Oscura (DM)**
4.  **Puntos de Ciencia (SCI)**
5.  **Influencia Política (INF)**
6.  **Suministros Alimentarios (SUP)**

#### Zona 3: Hilos de Producción Activa (Ancho Flexible, Alineado al Centro)
Contenedor dinámico que muestra el progreso del desarrollo imperial:
*   **Icono de Investigación Actual (28px x 28px):** Con una mini-barra de progreso horizontal en la parte inferior (`2px` de alto, color azul cian).
*   **Icono de Construcción Orbital (28px x 28px):** Indica la nave o estructura espacial que se está ensamblando.
*   **Icono de Infraestructura Urbana (28px x 28px):** Muestra el desarrollo edilicio en el planeta principal.

Al pasar el ratón (hover) sobre cualquiera de estos iconos, se despliega un panel contextual rápido (Popover) de gestión.

#### Zona 4: Alertas de Eventos e Hilos de Flotas (Ancho Flexible: Máx. 220px)
*   **Bandeja de Eventos de Crisis (Iconos de 24px x 24px):** Notificaciones de combate, anomalías descubiertas o mensajes diplomáticos.
*   **Estados de Flota (Compacto):** Un indicador dinámico del número de flotas activas en movimiento con un icono de vector de movimiento orbital (`16px`).

#### Zona 5: Sistema y Control de Ventana (Ancho Fijo: 160px)
*   **Reloj del Sistema / Tiempo de Juego (42px):** Muestra la hora local o un temporizador de productividad para el usuario.
*   **Selector de Pantalla Completa:** Botón cuadrado de `32px x 32px` con el icono de maximizar/restaurar (`14px`), que realiza la transición sin cortes al modo Pantalla Completa.
*   **Acceso a Opciones Rápidas:** Botón de `32px x 32px` para volumen, pausa del juego y anclaje de la barra.

---

### 2.3. Comportamiento en Pantalla y Alineación Multimonitor
*   **Acoplamiento Inteligente (Screen Docking):**
    *   **Inferior (Recomendado):** Se sitúa inmediatamente por encima de la barra de tareas nativa del sistema operativo (ej. a `40px` de altura del borde inferior en Windows 11 no oculta, o directamente en el borde a `0px` si la barra de tareas está en modo ocultación automática).
    *   **Superior:** Se acopla al píxel `0` del borde superior de la pantalla.
    *   **Flotante (HUD Desacoplado):** Se comporta como un panel compacto con un tirador de arrastre en el extremo izquierdo.
*   **Gestión Multimonitor:**
    *   La barra HUD puede configurarse en cualquier monitor del sistema mediante el menú de configuración (Monitor Principal, Monitor Secundario 1, Monitor Secundario 2).
    *   **Evitación de Oclusión Dinámica:** Si se detecta una aplicación a pantalla completa en el monitor donde reside la barra de tareas del juego, el HUD de TASK BAR 4X se ocultará automáticamente o se trasladará al monitor secundario activo para no interferir con las tareas del usuario.
    *   **Soporte de DPI Ultra-Alto (Ultra-DPI & Retina scaling):** La interfaz utiliza coordenadas lógicas relativas al factor de escala del monitor (`100%`, `125%`, `150%`, `200%`) para garantizar que la altura se mantenga físicamente en torno a los `12mm` de pantalla física.

---

## 3. Modo Pantalla Completa

El modo Pantalla Completa transforma TASK BAR 4X en una experiencia inmersiva clásica de gran estrategia, expandiendo la información de forma modular en una cuadrícula (Grid) clásica de 12 columnas.

```
+-------------------------------------------------------------------------------------------------------+
|  ZONA SUPERIOR (48px) - HUD de Recursos Consolidado                                                   |
+-------------------------------------------------------------------------------------------------------+
|   PANEL IZQUIERDO     |                                                       |    PANEL DERECHO      |
|   (320px)             |                     LIENZO CENTRAL                    |    (320px)            |
|                       |                     DE VISTA 3D                       |                       |
|   * Árbol de          |                     Y MAPA GALÁCTICO                  |    * Detalles de la   |
|     Tecnologías       |                                                       |      Flota            |
|   * Gestión de        |                                                       |    * cola de          |
|     Planetas          |                                                       |      Producción       |
|                       |                                                       |                       |
+-------------------------------------------------------------------------------------------------------+
|  ZONA INFERIOR (48px) - Control de Turno, Alertas Críticas y Acceso Rápido                            |
+-------------------------------------------------------------------------------------------------------+
```

### 3.1. Rejilla y Distribución Espacial (Full Screen Grid)
*   **Rejilla Base:** Rejilla fluida de `12 columnas` con canalones (`gutter`) de `16px` y márgenes perimetrales de `24px`.
*   **Paneles Laterales Extensibles (Drawer Panels):**
    *   **Ancho estándar:** `320px` (apropiado para relaciones de aspecto `16:9` y `21:9`).
    *   **Comportamiento:** Se deslizan lateralmente mediante transiciones de hardware aceleradas. Se pueden colapsar individualmente con la tecla rápida `Tab` para liberar el mapa estelar de obstrucciones visuales.
*   **Lienzo de Simulación (Central Viewport):**
    *   Área interactiva en 3D que renderiza los sistemas estelares, órbitas y trayectorias de flotas.
    *   Los elementos de UI de esta capa (nombres de planetas, rutas comerciales, vectores) están proyectados en coordenadas espaciales pero utilizan un escalado inteligente de fuentes para no ser ilegibles a altos niveles de zoom.

### 3.2. Transiciones Fluidas de Cambio de Modo
La transición entre el modo Barra de Tareas (HUD) y el modo Pantalla Completa debe ser suave y orgánica:
1.  **Preservación del Contexto:** Al maximizar, el HUD de 48px se expande vertical y horizontalmente. Los iconos de recursos de la Zona 2 se reubican fluidamente en la parte superior central de la pantalla mediante interpolación lineal de posiciones.
2.  **Animación de Apertura (Morphing Effect):**
    *   El contenedor de la barra de tareas se expande cubriendo la pantalla.
    *   Duración: `300ms` exactos.
    *   Curva de animación: `cubic-bezier(0.34, 1.56, 0.64, 1)` (efecto elástico suave de expansión).
    *   Durante el proceso, el fondo de escritorio del usuario se oscurece gradualmente mediante una transición CSS de opacidad en una capa de fondo (`overlay`) de `rgba(0, 0, 0, 0.75)`.

---

## 4. Especificaciones de Diseño Visual (Design Tokens)

### 4.1. Paleta de Colores
La paleta de colores utiliza tonos de emisión de luz altamente saturados y de base oscura, simulando pantallas holográficas tácticas militares.

| Nombre de Token | Color Hex/RGBA | Uso Aplicado |
| :--- | :--- | :--- |
| `--bg-base-dark` | `#080D16` | Fondo sólido de paneles bajo la capa vítrea. |
| `--bg-glass-panel` | `rgba(10, 18, 30, 0.65)` | Fondo con efecto glassmorphism para el HUD. |
| `--fg-high-intense` | `#F0F5FA` | Texto principal, cifras numéricas clave, títulos de paneles. |
| `--fg-muted` | `#8C9BB0` | Textos secundarios, unidades de medida, descripciones informativas. |
| `--color-energy` | `#FFE600` | Créditos de Energía (Icono, Gráficos, Alertas de déficit). |
| `--color-mineral` | `#FF8A00` | Minerales de Titanio (Estructuras, Costes de construcción). |
| `--color-science` | `#00F0FF` | Puntos de Ciencia, Tecnologías, Hilos de Investigación. |
| `--color-influence` | `#D946EF` | Puntos de Influencia, Diplomacia, Adquisición de Sectores. |
| `--color-success` | `#10B981` | Tendencias positivas, finalización de construcciones, alianzas. |
| `--color-danger` | `#EF4444` | Tendencias negativas, naves bajo ataque, fallos de soporte vital. |

---

## 4.2. Especificaciones de Glassmorphism (Efecto Vidrio Esmerilado)
Para conseguir el efecto de cristal táctico flotante sobre el escritorio del usuario, se debe implementar la siguiente pila de estilos CSS en los componentes contenedores:

```css
.hud-glass-panel {
  background: linear-gradient(
    135deg,
    rgba(10, 18, 30, 0.70) 0%,
    rgba(5, 10, 20, 0.60) 100%
  );
  backdrop-filter: blur(16px) saturate(150%);
  -webkit-backdrop-filter: blur(16px) saturate(150%);
  
  /* Borde Doble de Precisión */
  border: 1px solid rgba(255, 255, 255, 0.08);
  box-shadow: 
    0 8px 32px 0 rgba(0, 0, 0, 0.40),
    inset 0 1px 0 0 rgba(255, 255, 255, 0.12),
    inset 0 -1px 0 0 rgba(0, 0, 0, 0.30);
}
```

*   **Resplandor Activo de Borde:** Para paneles que requieren llamar la atención del jugador (ej. anomalía científica detectada), el borde adquiere un matiz coloreado reactivo:
    *   Científico: `border-color: rgba(0, 240, 255, 0.3);`
    *   Militar: `border-color: rgba(239, 68, 68, 0.3);`

---

## 4.3. Especificación de Tipografía
Para asegurar una legibilidad impecable en tamaños extremadamente pequeños (`9px` a `12px`), se combinan dos familias tipográficas:
1.  **Outfit (Familia de Títulos y Cifras):** Diseñada con formas geométricas limpias y terminaciones abiertas. Excelente para valores numéricos dinámicos de recursos y cabeceras de paneles.
2.  **Inter (Familia de Lectura):** Diseñada específicamente para pantallas de ordenador y textos de tamaño reducido. Gran altura de la x y excelente legibilidad en descripciones densas.

#### Escala Tipográfica y Estilos CSS:

| Nivel / Rol | Fuente | Peso (Weight) | Tamaño (Size) | Interlineado | Espaciado de Letra | Aplicación |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `H1` | Outfit | `700 (Bold)` | `20px` | `24px` | `-0.02em` | Título de Panel Principal en Pantalla Completa |
| `H2` | Outfit | `600 (SemiBold)` | `14px` | `18px` | `-0.01em` | Títulos de Popover y Paneles Laterales |
| `Body Large` | Inter | `400 (Regular)` | `12px` | `16px` | `0` | Textos de descripción corta y efectos |
| `Body Small` | Inter | `400 (Regular)` | `10px` | `13px` | `0` | Descripciones secundarias de tecnologías |
| `Data Accent` | Outfit | `600 (SemiBold)` | `11px` | `12px` | `+0.02em` | Números de recursos en la barra (Zona 2) |
| `Micro Label` | Inter | `500 (Medium)` | `8px` | `10px` | `+0.04em` | Tendencias +/- y etiquetas de progreso |

---

## 4.4. Estados de Interacción (Microinteracciones)
Cada elemento interactivo (botones, celdas de recursos, opciones de menú) debe responder de forma visual e inequívoca a las acciones del jugador.

#### Botones Estándar del HUD (Estados visuales):
*   **Estado de Reposo (Default):**
    *   Fondo: `rgba(255, 255, 255, 0.03)`
    *   Borde: `1px solid rgba(255, 255, 255, 0.08)`
    *   Opacidad del icono: `0.7`
*   **Estado Flotando (Hover):**
    *   Fondo: `rgba(255, 255, 255, 0.08)`
    *   Borde: `1px solid rgba(255, 255, 255, 0.20)`
    *   Opacidad del icono: `1.0`
    *   Efecto: Transición suave de `100ms` en color de fondo y borde (`transition: all 0.1s ease-out`).
*   **Estado Presionado (Active/Click):**
    *   Fondo: `rgba(255, 255, 255, 0.15)`
    *   Borde: `1px solid rgba(255, 255, 255, 0.30)`
    *   Efecto: Desplazamiento visual hacia abajo de `1px` (`transform: translateY(1px)`), simulando un botón físico.
*   **Estado Enfocado (Focus):**
    *   Efecto: Un anillo de contorno exterior (`outline`) de `2px` de espesor en color `--color-science` (azul cian) con un espaciado de `2px` de la caja del elemento.
*   **Estado Deshabilitado (Disabled):**
    *   Opacidad del componente: `0.4`
    *   Cursor del sistema: `not-allowed`

---

## 5. Ventanas Emergentes Contextuales Rápidas (Quick Popovers)

Los **Quick Popovers** son la pieza clave de la UX en el modo Barra de Tareas. Permiten realizar acciones críticas de microgestión planetaria o urbana en **menos de 3 segundos**, sin necesidad de cambiar a pantalla completa y sin interferir con las tareas activas del sistema operativo del usuario.

```
        +---------------------------------------------+
        |   PANEL DE MICROGESTIÓN PLANETARIA   [X]     |  <-- Título del Planeta y Botón Cerrar
        +---------------------------------------------+
        | [ Recursos ] [ Producción ] [ Población ]   |  <-- Pestañas Rápidas
        +---------------------------------------------+
        |  Cola de Construcción:                      |
        |  - Reactor de Fusión V (2 turnos)           |  <-- Lista Interactiva
        |  - Domo Agrícola III (4 turnos)             |
        |  [+] Añadir Proyecto a la Cola              |  <-- Botón de Acción Directa
        +---------------------------------------------+
        |  Trabajadores Asignados:                     |
        |  Minas: [ - ]  4 / 6  [ + ]                 |  <-- Control de Asignación Rápida
        |  Energía: [ - ] 2 / 4  [ + ]                 |
        +---------------------------------------------+
```

### 5.1. Reglas de Comportamiento e Interacción
*   **Activación sin Clics Perdidos:** Se pueden activar mediante un clic simple sobre el elemento correspondiente en la barra de tareas.
*   **Comportamiento de Cierre Inteligente (Smart Dismiss):**
    *   Para evitar frustraciones, el popover permanecerá abierto mientras el cursor del ratón se mantenga dentro de los límites del panel o de la celda origen de la barra de tareas.
    *   Si el cursor sale de esta área por más de `450ms` (tiempo de amortiguación para evitar cierres accidentales al mover el ratón rápidamente), el panel se desvanecerá automáticamente.
    *   **Pin de Persistencia:** Los popovers cuentan con un icono de "chincheta" (pin) en su esquina superior derecha. Si se activa, el panel permanece abierto de forma permanente como una ventana flotante de escritorio en segundo plano, permitiendo al usuario monitorizar esa producción mientras trabaja en sus tareas cotidianas en el PC.

### 5.2. Dimensiones y Parámetros del Popover
*   **Ancho estándar:** `280px` fijos.
*   **Altura:** Flexible, acotada a un máximo de `340px`. Si el contenido excede esta altura, se activa una barra de desplazamiento interna ultra-delgada (`3px` de ancho, color `--fg-muted`).
*   **Distancia de Acoplamiento:** Se sitúa a exactamente `8px` de separación del HUD de la barra de tareas, orientado según la posición de la barra (ej. si la barra de tareas está abajo, el popover aparece flotando a `8px` arriba de ella).
*   **Animación de Despliegue:**
    *   Aparición progresiva: `opacity: 0` a `opacity: 1`.
    *   Desplazamiento sutil: `translateY(10px)` hacia su posición final `translateY(0)`.
    *   Duración: `180ms` con curva `cubic-bezier(0.16, 1, 0.3, 1)`.

---

## 6. Pautas de Accesibilidad e Inclusión

TASK BAR 4X debe garantizar una legibilidad impecable para todo tipo de usuarios, especialmente debido a su formato de interfaz compacta.

*   **Alto Contraste Textual:** La relación de contraste entre el texto y la capa de glassmorphism de fondo debe superar el ratio `4.5:1` en todas las configuraciones posibles (cumpliendo con la directriz WCAG AA). En caso de fondos claros en el sistema operativo del usuario, se aplicará dinámicamente un oscurecimiento adicional (`rgba(0, 0, 0, 0.85)`) a la base de la barra de tareas del juego.
*   **Modo Daltonismo Integrado:**
    *   **Protanopía / Deuteranopía:** La distinción verde/rojo para las tendencias de recursos se sustituye por:
        *   Tendencia Positiva: Azul brillante (`#00F0FF`)
        *   Tendencia Negativa: Naranja intenso (`#FF8A00`)
    *   **Tritanopía:** Ajuste de los indicadores de energía y escudos para evitar confusiones de azul/amarillo.
*   **Escalado de Texto Separado:** Permite aumentar el tamaño de la fuente de la interfaz (`+10%`, `+20%`, `+30%`) sin que el layout de la barra de 48px se rompa, convirtiendo el texto de datos secundarios en iconos informativos autoexplicativos en caso de colisión espacial.
