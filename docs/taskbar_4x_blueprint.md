# Master Blueprint de Diseño: TASK BAR 4X
*El Simulador 4X e Histórico de Bajo Fricción para la Barra de Tareas*

---

## 🌌 1. Visión General del Proyecto

**TASK BAR 4X** es un simulador de estrategia espacial e histórica 4X (eXplorar, eXpandir, eXplotar, eXterminar) diseñado específicamente para ejecutarse de forma persistente y no intrusiva en Microsoft Windows. 

Inspirado en la interfaz ultra-compacta de **TBH: Task Bar Hero** y la adicción logística pasiva de **Rusty's Retirement**, el juego evoluciona el género de simulación en segundo plano al introducir una progresión de civilización que abarca desde los albores de la humanidad hasta la Singularidad cuántica, estructurada en **15 Edades**. 

El juego opera bajo un **Sistema de Doble Modo**:
*   **Modo Barra de Tareas (HUD Compacto)**: Una barra de 48px de alto que se acopla sobre la barra de tareas de Windows, ideal para el monitoreo pasivo de recursos y toma de decisiones rápidas de un solo clic sin robar el foco de las herramientas de trabajo del usuario.
*   **Modo Pantalla Completa (Vista Táctica)**: Una ventana de simulación profunda que se activa a pantalla completa, permitiendo gestionar mapas de campaña, negociaciones diplomáticas complejas, reclutamiento detallado de tropas e investigación tecnológica.

El juego incorpora un **Componente Rogue-like de Rejugabilidad**, estructurando la progresión en "Incursiones" (Runs) con mapas procedimentales, reliquias modificadoras del estilo de juego y un árbol de meta-progresión permanente alimentado por una divisa de ascensión cósmica.

---

## 🎭 2. Sistema de Doble Modo e Interfaz (UI/UX)

La interfaz gráfica está diseñada bajo conceptos de **Glassmorphism Premium** para integrarse estéticamente con los lenguajes de diseño modernos de Windows 11 (efectos de acrílico translúcido y bordes con iluminación de neón).

```
+---------------------------------------------------------------------------------+
|                                 MONITOR / DESKTOP                               |
|                                                                                 |
|  [ Modo Pantalla Completa (F11 / Doble Click Orbe) ]                            |
|  - Mapa táctico 2D/3D (Zonas de control de facciones, ejércitos, flotas).        |
|  - Panel de Diplomacia detallado y tratados bilaterales.                        |
|  - Árbol de Tecnologías ramificado e interactivo.                              |
|  - Gestión de Ciudades (Producción urbana, distritos, reclutamiento).           |
|                                                                                 |
+---------------------------------------------------------------------------------+
|  [ Modo Barra de Tareas (48px HUD persistente acoplado arriba de Windows) ]    |
|  [Orbe] [eXplore: Hilo Táctico] [eXpand: Colonias] [eXploit: Recs] [Mil] [Logs] |
+---------------------------------------------------------------------------------+
```

### A. Modo Barra de Tareas (HUD)
*   **Diseño Físico**: Franja horizontal de **48 píxeles** de alto acoplada en el borde inferior o superior de la pantalla principal o secundaria.
*   **Módulo Izquierdo (Mapeo 1D y Exploración)**: Muestra el mapa territorial simplificado como un grafo lineal. Los puntos indican celdas exploradas (`O`), celdas enemigas/bárbaras (`*`), y zonas inexploradas (`.`). Los exploradores y ejércitos se desplazan físicamente de izquierda a derecha.
*   **Módulo Central (Pipelines de Logística)**: Iconos animados que representan la cadena de suministro en tránsito físico por la barra de tareas (ej. troncos de madera moviéndose hacia una fundición).
*   **Módulo Derecho (Macro-Contadores)**: Recursos globales, población, nivel de la edad actual y botón de configuración/prestigio.
*   **Notificaciones Silenciosas**: El borde superior del HUD parpadea sutilmente con colores temáticos ante eventos (Rojo = Combate, Púrpura = Ciencia completada, Verde = Almacenes llenos).

### B. Modo Pantalla Completa (Vista Táctica)
*   **Activación**: Se despliega de manera inmediata al presionar el atajo global de teclado `Win + Alt + X`, pulsar la tecla `F11` con el juego activo, o hacer doble clic en el "Orbe Imperial" (el logo de facción en la esquina del HUD).
*   **Detalle Operativo**:
    *   **Mapa de Campaña**: Representación interactiva 2.5D de la geografía del juego, mostrando fronteras geopolíticas, distribución de ejércitos y recursos estratégicos.
    *   **Árbol Tecnológico**: Interfaz interactiva para el desbloqueo de mejoras y ciencias de la época.
    *   **Gabinete Diplomático**: Pantalla de negociación para establecer pactos de no agresión, declarar guerras u ofrecer rutas comerciales de importación/exportación con la IA.
    *   **Diseño y Producción de Unidades**: Menú detallado para reclutar tropas históricas y personalizar equipamiento táctico.

---

## ⏳ 3. La Progresión Histórica: Las 15 Edades de la Civilización

El imperio evoluciona cronológicamente a través de **15 Edades**. Cada transición de era requiere construir una **Estructura Emblemática** (Maravilla) y completar una investigación especial de transición. Esto activa un prestigio (reset parcial de las colonias y pipelines locales) a cambio de desbloquear multiplicadores permanentes y cambiar el estilo visual del HUD.

1.  **Edad de Piedra (Paleolítico)**: Clan nómada, fogatas y recolección manual.
2.  **Edad del Neolítico (Agricultura)**: Asentamiento permanente, domesticación de animales y alfarería.
3.  **Edad del Cobre (Calcolítico)**: Primeros metales martillados, minería primitiva y comercio inicial.
4.  **Edad del Bronce (Primeros Imperios)**: Nacimiento de la escritura, la burocracia y la infantería en formación.
5.  **Edad del Hierro (Imperios Clásicos)**: Metalurgia de alta resistencia, caminos empedrados e infantería pesada.
6.  **Antigüedad Tardía (Decadencia y Caída)**: Fortificación defensiva, pérdida de cohesión central y grandes migraciones.
7.  **Alta Edad Media (Feudalismo)**: Estructura social rígida, autarquía agraria e incursiones a pequeña escala.
8.  **Baja Edad Media (Renacimiento Urbano)**: Nacimiento de los gremios de oficios, comercio marítimo y universidades.
9.  **Renacimiento y Descubrimientos**: Exploración oceánica a gran escala, carabelas e imprenta.
10. **Era de la Ilustración y Revoluciones**: Revolución científica, manufacturas a gran escala y constitucionalismo.
11. **Era Industrial (Carbón y Vapor)**: Ferrocarriles, hiladoras mecánicas, energía a vapor y malestar obrero.
12. **Era Atómica e Informática**: Ordenadores de silicio, energía nuclear, Internet y automatización de fábricas.
13. **Era de la Expansión Solar**: Colonización lunar y marciana, minería de asteroides y energía solar concentrada.
14. **Era Interestelar (Megadiseños)**: Motores de curvatura, esferas de Dyson y propulsión por antimateria.
15. **Era de la Singularidad (Trascendencia)**: Civilización post-física, mentes colmena cuánticas y virtualización absoluta.

---

## 🌿 4. Árbol de Investigaciones y Matriz de Decisiones Binarias

En cada una de las 15 Edades se introducen **12 Investigaciones** agrupadas por pares en **6 Disciplinas**. El jugador debe tomar una **decisión binaria y mutuamente excluyente** por disciplina. Cada elección define un **Bufo** (ventaja) y un **Reto** (desventaja/penalización), lo que garantiza que cada partida cuente con configuraciones de imperio y estilos de juego completamente diferentes.

```
                  +--------------------------+
                  |  DISCIPLINA DE ESTUDIO   |
                  +--------------------------+
                               |
            +------------------+------------------+
            |                                     |
  [ VARIANTE A (Elección A) ]           [ VARIANTE B (Elección B) ]
  - Ventaja (Bufo)                      - Ventaja (Bufo)
  - Desventaja (Reto)                   - Desventaja (Reto)
```

A continuación se detalla la matriz de las **90 decisiones binarias (180 tecnologías)** a lo largo del juego:

---

### EDAD 1: Edad de Piedra (Paleolítico)
*   **MILITAR**:
    *   *Variante A (Lanzas de Caza)*: +20% Daño de exploradores contra bestias silvestres. (Reto: -15% Defensa en el campamento).
    *   *Variante B (Trampas de Foso)*: +25% Defensa del asentamiento. (Reto: Exploradores se mueven 10% más lentos).
*   **ECONOMÍA**:
    *   *Variante A (Nómadas Recolectores)*: +15% Recolección de bayas. (Reto: Límite de capacidad de almacenamiento bajo).
    *   *Variante B (Cazadores de Megafauna)*: +30% Obtención de carne y pieles. (Reto: Requiere 3 de población inactiva permanente).
*   **POLÍTICA**:
    *   *Variante A (Liderazgo de Fuerza)*: +10% Velocidad de construcción. (Reto: Inestabilidad al fallecer el líder).
    *   *Variante B (Consejo de Ancianos)*: +15% Estabilidad interna. (Reto: Investigaciones científicas tardan 10% más).
*   **CULTURA**:
    *   *Variante A (Pintura Chamánica)*: +20% Generación de Fe. (Reto: -10% producción de herramientas).
    *   *Variante B (Adornos de Hueso)*: +15% Influencia de fronteras. (Reto: Nula fe inicial).
*   **TECNOLOGÍA**:
    *   *Variante A (Bifaz de Sílex)*: +15% Recolección de madera. (Reto: Rápido desgaste de herramientas).
    *   *Variante B (Lanza de Hueso Pulido)*: +10% Daño a distancia de proyectiles. (Reto: Consume recursos de fauna).
*   **RELIGIÓN**:
    *   *Variante A (Animismo Natural)*: +25% Producción en celdas de bosque. (Reto: Gran penalización al talar).
    *   *Variante B (Culto a los Ancestros)*: +20% Tasa de natalidad. (Reto: Requiere celdas dedicadas a enterramiento).

---

### EDAD 2: Edad del Neolítico (Agricultura)
*   **MILITAR**:
    *   *Variante A (Guardia del Asentamiento)*: +20% Defensa contra clanes invasores. (Reto: Tropas lentas y estacionarias).
    *   *Variante B (Incursiones Rápidas)*: +25% Recursos saqueados de celdas bárbaras. (Reto: Asentamiento vulnerable a asaltos).
*   **ECONOMÍA**:
    *   *Variante A (Cultivo por Inundación)*: +25% Comida en celdas fluviales. (Reto: Extremadamente sensible a sequías).
    *   *Variante B (Pastoreo de Crianza)*: +20% Cuero y lana. (Reto: Reduce espacio para viviendas populares).
*   **POLÍTICA**:
    *   *Variante A (Líder Constructor)*: +15% Velocidad de construcción de monumentos. (Reto: Aumenta el descontento de la tribu).
    *   *Variante B (Asamblea Familiar)*: +20% Ciencia. (Reto: Toma de decisiones de construcción más lenta).
*   **CULTURA**:
    *   *Variante A (Figurillas de Arcilla)*: +25% Generación de Cultura. (Reto: Consume reservas de arcilla).
    *   *Variante B (Monolitos de Piedra)*: +15% Control de fronteras. (Reto: Alto coste de mano de obra).
*   **TECNOLOGÍA**:
    *   *Variante A (Hoz de Sílex)*: +20% Recolección de cereales. (Reto: Rápido desgaste).
    *   *Variante B (Alfarería de Horno)*: +15% Límite máximo de comida almacenada. (Reto: Requiere madera de forma continua).
*   **RELIGIÓN**:
    *   *Variante A (Ritos de Cosecha)*: +20% Natalidad. (Reto: -10% de efectividad militar).
    *   *Variante B (Danza de Lluvia)*: +15% Resistencia económica a desastres naturales. (Reto: Consume recursos agrícolas en ceremonias).

---

### EDAD 3: Edad del Cobre (Calcolítico)
*   **MILITAR**:
    *   *Variante A (Hachas de Cobre)*: +25% Daño de ataque cuerpo a cuerpo. (Reto: Equipamiento militar caro).
    *   *Variante B (Muros de Adobe)*: +30% Defensa en celdas de ciudad. (Reto: Consume mucha arcilla de construcción).
*   **ECONOMÍA**:
    *   *Variante A (Extracción en Galería)*: +25% Cobre y piedra. (Reto: Alto riesgo de derrumbes y accidentes en minas).
    *   *Variante B (Metalurgia a Fuego)*: +20% Refinado de metal. (Reto: Deforestación acelerada de bosques colindantes).
*   **POLÍTICA**:
    *   *Variante A (Cacicazgo de Linaje)*: +15% Recaudación de tributos. (Reto: Alto riesgo de guerra civil al morir el cacique).
    *   *Variante B (Jerarquía Religiosa)*: +20% Estabilidad social. (Reto: Los sacerdotes consumen el 10% de la producción de alimentos).
*   **CULTURA**:
    *   *Variante A (Vasijas Campaniformes)*: +25% Influencia comercial en fronteras. (Reto: Menor generación de fe).
    *   *Variante B (Joyas de Cobre)*: +20% Cultura. (Reto: Consume metales finos de las fundiciones).
*   **TECNOLOGÍA**:
    *   *Variante A (Fundición en Molde)*: +20% Herramientas producidas. (Reto: Requiere carbón de madera constante).
    *   *Variante B (Eje de Rueda Primitivo)*: +15% Velocidad de transporte de caravanas. (Reto: Reduce defensa en movimiento).
*   **RELIGIÓN**:
    *   *Variante A (Panteón Celestial)*: +25% Fe. (Reto: -10% de avance científico).
    *   *Variante B (Ofrendas Metalúrgicas)*: +20% Estabilidad del reino. (Reto: Requiere fundir recursos metálicos en ceremonias).

---

### EDAD 4: Edad del Bronce (Primeros Imperios)
*   **MILITAR**:
    *   *Variante A (Infantería de Falange)*: +25% Defensa en formación cerrada. (Reto: Velocidad de marcha reducida).
    *   *Variante B (Carros de Guerra)*: +30% Ataque en asaltos rápidos. (Reto: Requiere celdas de caballos y bronce abundante).
*   **ECONOMÍA**:
    *   *Variante A (Tributación Real)*: +20% Ingreso de oro estatal. (Reto: Descontento en colonias periféricas).
    *   *Variante B (Rutas de Caravanas)*: +25% Producción comercial general. (Reto: Rutas vulnerables a emboscadas enemigas).
*   **POLÍTICA**:
    *   *Variante A (Código de Leyes Escrito)*: +20% Estabilidad y orden público. (Reto: -10% de velocidad científica).
    *   *Variante B (Teocracia Imperial)*: +25% Fe y legitimidad del trono. (Reto: El clero controla el tesoro).
*   **CULTURA**:
    *   *Variante A (Escribas de Arcilla)*: +20% Ciencia. (Reto: Consume arcilla de forma continua).
    *   *Variante B (Arquitectura Monumental)*: +30% Cultura. (Reto: Elevado coste de construcción).
*   **TECNOLOGÍA**:
    *   *Variante A (Aleación de Bronce)*: +25% Eficiencia de herramientas y armas. (Reto: Dependencia total de estaño importado).
    *   *Variante B (Navegación Costera)*: +20% Rango de exploración marítima. (Reto: Riesgo de naufragios en alta mar).
*   **RELIGIÓN**:
    *   *Variante A (Templos Estatales)*: +25% Fe. (Reto: Elevado coste de mantenimiento en oro).
    *   *Variante B (Culto al Monarca Divino)*: +20% Fuerza militar ofensiva. (Reto: Revueltas ante derrotas militares).

---

### EDAD 5: Edad del Hierro (Imperios Clásicos)
*   **MILITAR**:
    *   *Variante A (Legiones Disciplinadas)*: +25% Ataque y defensa en batallas. (Reto: Alto coste de mantenimiento mensual).
    *   *Variante B (Infantería Auxiliar)*: +20% Velocidad de reclutamiento. (Reto: Menor moral y propensos a la retirada).
*   **ECONOMÍA**:
    *   *Variante A (Moneda Acuñada)*: +25% Eficiencia de mercados. (Reto: Riesgo de inflación y fluctuación comercial).
    *   *Variante B (Latifundios Agrícolas)*: +20% Producción de alimentos básicos. (Reto: Alto riesgo de revueltas de siervos/esclavos).
*   **POLÍTICA**:
    *   *Variante A (República Oligárquica)*: +20% Ciencia y cultura. (Reto: Facciones del senado en pugna constante).
    *   *Variante B (Imperio Autocrático)*: +25% Estabilidad y expansión territorial. (Reto: Sucesiones de gobernantes inestables).
*   **CULTURA**:
    *   *Variante A (Filosofía y Retórica)*: +25% Ciencia. (Reto: Cuestionamiento social de la fe tradicional).
    *   *Variante B (Juegos y Coliseos)*: +30% Felicidad y control de crisis de descontento. (Reto: Elevado consumo de oro).
*   **TECNOLOGÍA**:
    *   *Variante A (Forja de Hierro)*: +25% Producción minera. (Reto: Elevado consumo de carbón vegetal).
    *   *Variante B (Calzadas Imperiales)*: +20% Velocidad logística lineal. (Reto: Permite que los enemigos viajen más rápido a tu capital).
*   **RELIGIÓN**:
    *   *Variante A (Panteón Cívico)*: +20% Orden público. (Reto: Incompatibilidad cultural con minorías conquistadas).
    *   *Variante B (Misticismo de Misterio)*: +25% Fe. (Reto: Cismas internos constantes).

---

### EDAD 6: Antigüedad Tardía (Decadencia y Caída)
*   **MILITAR**:
    *   *Variante A (Murallas de Ladrillo Pesadas)*: +35% Defensa de ciudades. (Reto: Las guarniciones no pueden moverse de su celda).
    *   *Variante B (Caballería de Choque Catafracta)*: +25% Daño de carga. (Reto: Coste de equipamiento de hierro prohibitivo).
*   **ECONOMÍA**:
    *   *Variante A (Autarquía Local)*: +20% Comida en celdas locales. (Reto: Cero ingresos por comercio exterior).
    *   *Variante B (Impuestos de Emergencia)*: +25% Oro recolectado de celdas. (Reto: Despoblación y abandono de tierras agrícolas).
*   **POLÍTICA**:
    *   *Variante A (Burocracia Centralizada)*: +20% Recaudación. (Reto: Fugas de recursos por corrupción endémica).
    *   *Variante B (Descentralización Regional)*: +20% Estabilidad local. (Reto: Dificultad para coordinar fuerzas militares).
*   **CULTURA**:
    *   *Variante A (Crónicas de Decadencia)*: +25% Fe. (Reto: -15% de velocidad en avances científicos).
    *   *Variante B (Preservación del Conocimiento)*: +20% Ciencia. (Reto: Requiere construir bibliotecas costosas en el Orbe).
*   **TECNOLOGÍA**:
    *   *Variante A (Molino Rotativo Manual)*: +20% Celeridad de comida. (Reto: Requiere mano de obra permanente).
    *   *Variante B (Hormigón Puzolánico)*: +25% Construcción defensiva rápida. (Reto: Requiere celdas de ceniza/piedra especial).
*   **RELIGIÓN**:
    *   *Variante A (Conversión del Imperio)*: +30% Unificación religiosa. (Reto: Conflictos y rebeliones de minorías heréticas).
    *   *Variante B (Sincretismo Defensivo)*: +20% Estabilidad comercial con bárbaros. (Reto: Pérdida de fe pura).

---

### EDAD 7: Alta Edad Media (Feudalismo)
*   **MILITAR**:
    *   *Variante A (Caballería Feudal)*: +30% Daño de asalto. (Reto: Capacidad de reclutamiento muy limitada).
    *   *Variante B (Milicia Leva)*: +25% Cantidad de infantería reclutable. (Reto: Moral de combate y defensa muy bajas).
*   **ECONOMÍA**:
    *   *Variante A (Rotación de Cultivos de Tres Hojas)*: +25% Comida. (Reto: Las invasiones destruyen campos en descanso de forma crítica).
    *   *Variante B (Tributo de Servidumbre)*: +20% Mano de obra barata. (Reto: Constante peligro de revueltas campesinas en el HUD).
*   **POLÍTICA**:
    *   *Variante A (Contrato Feudal)*: +20% Estabilidad interna. (Reto: Nobles exigen exención de impuestos).
    *   *Variante B (Centralización de la Corona)*: +20% Oro monárquico. (Reto: Rebeliones constantes de los barones).
*   **CULTURA**:
    *   *Variante A (Manuscritos Iluminados)*: +25% Cultura. (Reto: Proceso de copia extremadamente lento).
    *   *Variante B (Cantares de Gesta)*: +20% Moral militar en combate. (Reto: Cero avance en ciencia).
*   **TECNOLOGÍA**:
    *   *Variante A (Arado de Vertedera)*: +20% Producción en suelos húmedos. (Reto: Requiere bueyes de alto coste).
    *   *Variante B (Herraduras y Estribos)*: +15% Velocidad y potencia de caballería. (Reto: Consume hierro de forja).
*   **RELIGIÓN**:
    *   *Variante A (Monasterios Agrícolas)*: +25% Fe y ciencia. (Reto: Celdas del clero exentas de impuestos).
    *   *Variante B (Tribunales Eclesiásticos)*: +20% Estabilidad social. (Reto: Persecución de ideas y descontento).

---

### EDAD 8: Baja Edad Media (Renacimiento Urbano)
*   **MILITAR**:
    *   *Variante A (Compañías de Mercenarios)*: +30% Daño a distancia. (Reto: Requiere pago constante; si se acaba el oro, se rebelan).
    *   *Variante B (Muros Concéntricos)*: +35% Defensa de fortalezas. (Reto: Alta inversión de oro y tiempo).
*   **ECONOMÍA**:
    *   *Variante A (Gremios de Oficios)*: +25% Producción y aleaciones. (Reto: El monopolio gremial ralentiza nuevas tecnologías).
    *   *Variante B (Ligas Comerciales Marítimas)*: +30% Oro. (Reto: Rutas comerciales vulnerables a piratería).
*   **POLÍTICA**:
    *   *Variante A (Fueros Ciudadanos)*: +20% Crecimiento de población urbana. (Reto: Tensiones políticas constantes con terratenientes).
    *   *Variante B (Consejos Reales)*: +20% Recaudación de impuestos. (Reto: Aumento del coste de burocracia).
*   **CULTURA**:
    *   *Variante A (Arquitectura Gótica)*: +35% Fe y prestigio. (Reto: Consume grandes reservas de piedra y oro).
    *   *Variante B (Universidades Escolásticas)*: +25% Ciencia. (Reto: Disminuye la fe dogmática del reino).
*   **TECNOLOGÍA**:
    *   *Variante A (Gafas y Astrolabios)*: +20% Ciencia y exploración marítima. (Reto: Consume reservas de vidrio de alta pureza).
    *   *Variante B (Molinos Cinéticos)*: +25% Producción pasiva mediante viento/agua. (Reto: Dependencia total de factores meteorológicos).
*   **RELIGIÓN**:
    *   *Variante A (Órdenes Mendicantes)*: +20% Felicidad del pueblo. (Reto: No generan diezmo para las arcas públicas).
    *   *Variante B (Inquisición Doctrinal)*: +25% Control del orden público. (Reto: Frena y persigue avances científicos).

---

### EDAD 9: Renacimiento y Era de los Descubrimientos
*   **MILITAR**:
    *   *Variante A (Tercios de Picas y Arcabuces)*: +30% Daño defensivo y ofensivo combinado. (Reto: Equipamiento militar muy caro).
    *   *Variante B (Galeones de Guerra)*: +35% Dominio naval. (Reto: Elevado consumo de madera y aleaciones de bronce).
*   **ECONOMÍA**:
    *   *Variante A (Comercio Colonial)*: +35% Oro comercial. (Reto: Pérdida de cargamentos por tormentas y corsarios).
    *   *Variante B (Monopolios de Corona)*: +25% Recursos estables de minería. (Reto: Bloquea iniciativas mercantiles independientes).
*   **POLÍTICA**:
    *   *Variante A (Monarquía Absoluta)*: +25% Estabilidad y gobernanza directa. (Reto: Revueltas burguesas frecuentes).
    *   *Variante B (República Mercantil)*: +25% Comercio exterior. (Reto: Inestabilidad política por elecciones periódicas).
*   **CULTURA**:
    *   *Variante A (Mecenazgo Artístico)*: +30% Cultura. (Reto: Financiado con impuestos directos a la población).
    *   *Variante B (Humanismo y Ciencia)*: +25% Ciencia. (Reto: Debilita el fervor y la obediencia religiosa).
*   **TECNOLOGÍA**:
    *   *Variante A (Imprenta de Tipos Móviles)*: +35% Velocidad científica. (Reto: Acelera la difusión de ideas rebeldes y panfletos).
    *   *Variante B (Astrolabio y Cuadrante)*: +25% Exploración de fronteras. (Reto: Coste de fabricación de instrumentos finos).
*   **RELIGIÓN**:
    *   *Variante A (Reforma Protestante)*: +25% Ciencia y ética de trabajo. (Reto: Cismas y guerras religiosas internas).
    *   *Variante B (Contrarreforma)*: +30% Fe y estabilidad social. (Reto: Persecución y censura de libros y teorías).

---

### EDAD 10: Era de la Ilustración y Revoluciones
*   **MILITAR**:
    *   *Variante A (Ejércitos de Línea Profesionales)*: +25% Tasa de daño disciplinado. (Reto: Mantenimiento mensual fijo caro).
    *   *Variante B (Conscripción Nacional - Leva)*: +35% Volumen de reclutamiento. (Reto: Penalización del 20% en producción de celdas agrícolas).
*   **ECONOMÍA**:
    *   *Variante A (Fisiocracia Agraria)*: +25% Producción de alimentos. (Reto: Retrasa la industrialización pesada).
    *   *Variante B (Mercantilismo Proteccionista)*: +25% Oro e ingresos comerciales. (Reto: Empeora relaciones diplomáticas).
*   **POLÍTICA**:
    *   *Variante A (Despotismo Ilustrado)*: +20% Velocidad de reformas y ciencia. (Reto: Descontento por falta de derechos civiles).
    *   *Variante B (Parlamentarismo Constitucional)*: +25% Estabilidad a largo plazo. (Reto: Decisions del senado son lentas).
*   **CULTURA**:
    *   *Variante A (Salones y Debates)*: +30% Ciencia. (Reto: Disminuye significativamente el fervor religioso).
    *   *Variante B (Enciclopedismo)*: +25% Cultura. (Reto: Consume reservas de papel y recursos forestales).
*   **TECNOLOGÍA**:
    *   *Variante A (Máquina Atmosférica de Vapor)*: +20% Producción minera. (Reto: Consume carbón vegetal de forma acelerada).
    *   *Variante B (Cálculo Infinitesimal)*: +25% Ciencia general. (Reto: No provee producción material inmediata).
*   **RELIGIÓN**:
    *   *Variante A (Deísmo Ilustrado)*: +25% Tolerancia y ciencia. (Reto: Disminución de la fe comunitaria).
    *   *Variante B (Pietismo y Renacimiento Fe)*: +30% Fe y moral de tropas. (Reto: Oposición y censura de ideas científicas).

---

### EDAD 11: Era Industrial (Carbón y Vapor)
*   **MILITAR**:
    *   *Variante A (Artillería Pesada Estriada)*: +30% Daño de asedio militar. (Reto: Movilidad logística lenta).
    *   *Variante B (Acorazados de Acero)*: +35% Dominio naval y costero. (Reto: Altísimo coste de producción de acero).
*   **ECONOMÍA**:
    *   *Variante A (Capitalismo Laissez-Faire)*: +35% Oro e industria pesada. (Reto: Alta probabilidad de huelgas obreras destructivas).
    *   *Variante B (Proteccionismo Industrial)*: +25% Recursos estables de carbón. (Reto: Frena tratados de comercio exterior).
*   **POLÍTICA**:
    *   *Variante A (Estado de Bienestar Primitivo)*: +20% Control de crisis sociales. (Reto: Impuesto del 15% sobre el oro del erario).
    *   *Variante B (Expansión Colonial)*: +25% Recursos exóticos. (Reto: Guerras y rebeliones en territorios de ultramar).
*   **CULTURA**:
    *   *Variante A (Prensa de Masas)*: +25% Cultura. (Reto: Facilita la agitación política y panfletos obreros).
    *   *Variante B (Arquitectura de Hierro y Cristal)*: +30% Prestigio nacional. (Reto: Consume reservas de metal fino).
*   **TECNOLOGÍA**:
    *   *Variante A (Locomotora de Vapor)*: +35% Velocidad de transporte lineal de pipelines. (Reto: Requiere redes de carbón y hierro).
    *   *Variante B (Telégrafo Eléctrico)*: +25% Exploración y comunicaciones de frontera. (Reto: Coste de instalación alto).
*   **RELIGIÓN**:
    *   *Variante A (Secularismo Científico)*: +30% Ciencia. (Reto: Pérdida total de fe espiritual en el imperio).
    *   *Variante B (Evangelismo Industrial)*: +20% Estabilidad y obediencia laboral. (Reto: Oposición a teorías científicas modernas).

---

### EDAD 12: Era Atómica e Informática
*   **MILITAR**:
    *   *Variante A (Disuasión Nuclear)*: +40% Resistencia defensiva. (Reto: Riesgo de accidentes de fisión y desastres atómicos).
    *   *Variante B (Fuerzas Especiales de Movilidad)*: +30% Daño quirúrgico. (Reto: Alto coste de reclutamiento y mantenimiento).
*   **ECONOMÍA**:
    *   *Variante A (Automatización de Fábricas)*: +35% Producción industrial. (Reto: Aumento de tensiones y huelgas por desempleo tecnológico).
    *   *Variante B (Corporaciones Globales)*: +30% Oro comercial. (Reto: Evasión fiscal y pérdida de control regulatorio estatal).
*   **POLÍTICA**:
    *   *Variante A (Democracia Digital)*: +25% Estabilidad y felicidad. (Reto: Vulnerabilidad a ciberataques políticos).
    *   *Variante B (Tecnocracia Ministerial)*: +30% Velocidad de investigación científica. (Reto: Descontento popular).
*   **CULTURA**:
    *   *Variante A (Medios de Comunicación de Masas)*: +30% Cultura. (Reto: Propagación de desinformación y pánicos morales).
    *   *Variante B (Cultura de Consumo)*: +25% Felicidad y comercio interno. (Reto: Consumo de recursos naturales descontrolado).
*   **TECNOLOGÍA**:
    *   *Variante A (Microprocesadores de Silicio)*: +35% Ciencia. (Reto: Requiere celdas de tierras raras).
    *   *Variante B (Fisión Nuclear)*: +35% Energía. (Reto: Generación de residuos radiactivos periódicos).
*   **RELIGIÓN**:
    *   *Variante A (Humanismo Secular)*: +25% Tolerancia y paz social. (Reto: Cero generación de fe espiritual).
    *   *Variante B (Ortodoxia Doctrinaria)*: +30% Fe y cohesión nacional. (Reto: Oposición ética a la biotecnología).

---

### EDAD 13: Era de la Expansión Solar
*   **MILITAR**:
    *   *Variante A (Drones Autónomos Orbitales)*: +30% Daño ofensivo espacial. (Reto: Vulnerables a ciberataques remotos).
    *   *Variante B (Escudos de Plasma)*: +35% Defensa espacial. (Reto: Enorme consumo de energía eléctrica).
*   **ECONOMÍA**:
    *   *Variante A (Minería de Asteroides)*: +40% Producción de metales/aleaciones. (Reto: Tránsito logístico muy lento).
    *   *Variante B (Terraformación Colonial)*: +30% Comida y espacio habitable. (Reto: Alta inversión de energía inicial).
*   **POLÍTICA**:
    *   *Variante A (Federación de Mundos)*: +25% Comercio interplanetario. (Reto: Latencia de comunicación interplanetaria).
    *   *Variante B (Corporaciones de Frontera)*: +35% Producción colonial. (Reto: Huelgas violentas en colonias mineras).
*   **CULTURA**:
    *   *Variante A (Redes Cuánticas)*: +30% Cultura y velocidad de logs. (Reto: Consume infraestructura de telecomunicación cuántica).
    *   *Variante B (Estética de Domo)*: +25% Prestigio y atracción de población. (Reto: Gran coste de mantenimiento de soporte vital).
*   **TECNOLOGÍA**:
    *   *Variante A (Reactores de Fusión)*: +35% Energía. (Reto: Requiere explotación minera de Helio-3).
    *   *Variante B (Cómputo Cuántico)*: +35% Ciencia. (Reto: Requiere infraestructura de superenfriamiento).
*   **RELIGIÓN**:
    *   *Variante A (Panteísmo de Gaia)*: +25% Estabilidad ecológica. (Reto: Frena y penaliza la minería planetaria intensiva).
    *   *Variante B (Cosmo-Humanismo)*: +30% Adaptabilidad colonial en entornos extremos. (Reto: Cero fe religiosa).

---

### EDAD 14: Era Interestelar (Megadiseños)
*   **MILITAR**:
    *   *Variante A (Cruceros con Railguns)*: +35% Daño ofensivo a flotas. (Reto: Consume aleaciones espaciales en combate).
    *   *Variante B (Defensas Orbitales de Anillo)*: +40% Resistencia defensiva. (Reto: Unidades totalmente inmóviles).
*   **ECONOMÍA**:
    *   *Variante A (Enjambre de Dyson)*: +50% Energía estelar. (Reto: Construcción masiva que inhabilita planetas del sistema).
    *   *Variante B (Fábricas de Antimateria)*: +40% Combustible y aleaciones. (Reto: Riesgo de fallos críticos de contención).
*   **POLÍTICA**:
    *   *Variante A (Colectivo Consciente)*: +30% Eficiencia del imperio. (Reto: Elimina libertades y desata rebeliones de rebeldes).
    *   *Variante B (Confederación de Sistemas Libres)*: +25% Estabilidad social. (Reto: Alto riesgo de secesión e independencia de sistemas).
*   **CULTURA**:
    *   *Variante A (Museos Virtuales del Pasado)*: +30% Cultura. (Reto: Consume procesamiento de datos masivo).
    *   *Variante B (Monumentos Gravitacionales)*: +35% Prestigio estelar. (Reto: Consume energía de agujeros negros artificiales).
*   **TECNOLOGÍA**:
    *   *Variante A (Motores de Curvatura)*: +35% Velocidad de exploración interestelar. (Reto: Requiere combustible de antimateria).
    *   *Variante B (Nanotecnología Auto-Replicante)*: +35% Velocidad de construcción. (Reto: Riesgo de plaga gris).
*   **RELIGIÓN**:
    *   *Variante A (Armonía Cósmica)*: +30% Reducción de conflictos. (Reto: Menor efectividad militar ofensiva).
    *   *Variante B (Cientificismo Absoluto)*: +35% Ciencia. (Reto: Anulación completa de la generación de fe).

---

### EDAD 15: Era de la Singularidad (Trascendencia)
*   **MILITAR**:
    *   *Variante A (Desintegradores Dimensionales)*: +45% Daño de aniquilación. (Reto: Riesgo de inestabilidades temporales en el HUD).
    *   *Variante B (Escudos Gravitacionales)*: +50% Resistencia a daños. (Reto: Enorme coste de mantenimiento energético).
*   **ECONOMÍA**:
    *   *Variante A (Replicadores de Antimateria)*: +40% Producción libre de recursos. (Reto: Enorme consumo de energía estelar).
    *   *Variante B (Esfera de Dyson Completa)*: +60% Energía estelar. (Reto: Requiere desmantelar celdas planetarias por completo).
*   **POLÍTICA**:
    *   *Variante A (Mente Colmena Integrada)*: +35% Control y anulación de crisis. (Reto: Desaparición de los puntos de cultura individuales).
    *   *Variante B (Anarquía Algorítmica)*: +30% Ciencia de cálculo. (Reto: Anomalías de sistema e inestabilidad periódica).
*   **CULTURA**:
    *   *Variante A (Realidades Virtuales)*: +40% Felicidad y cultura. (Reto: Caída drástica del crecimiento demográfico físico).
    *   *Variante B (Megaproyectos de Memoria)*: +35% Prestigio y gloria. (Reto: Requiere reservas de antimateria).
*   **TECNOLOGÍA**:
    *   *Variante A (Manipulación de Cuerdas)*: +40% Ciencia. (Reto: Demanda de computación cuántica masiva).
    *   *Variante B (Virtualización de Conciencia)*: +40% Población virtual. (Reto: Vulnerable a ciber-plagas de datos y virus).
*   **RELIGIÓN**:
    *   *Variante A (El Dios Máquina)*: +40% Fe y estabilidad cuántica. (Reto: Rechazo y penalización a la biología tradicional).
    *   *Variante B (Cosmología Multidimensional)*: +35% Ciencia y fe armónicas. (Reto: Cero control social).

---

## 🎲 5. El Componente Rogue-like y Meta-Progresión (Bucle de Rejugabilidad)

Para multiplicar la rejugabilidad a la escala de los mejores juegos *Idle* y *Rogue-like* (como *Leaf Blower Revolution* y *Hades*), se introduce un bucle de juego basado en "Incursiones" (Runs) procedimentales y una progresión persistente del Legado del Imperio.

```
                  +-----------------------------------+
                  |      INICIO DE LA INCURSIÓN       |
                  |  - Mapa procedimental aleatorio   |
                  |  - Elección de Facción / Mutador  |
                  +-----------------------------------+
                                    |
                                    v
                  +-----------------------------------+
                  |        PROGRESO POR ERAS          |
                  |  - 15 Edades históricas           |
                  |  - Recolección de Reliquias       |
                  |  - Decisiones de Tecnología       |
                  +-----------------------------------+
                                    |
             +----------------------+----------------------+
             |                                             |
             v                                             v
  [ COLAPSO / DERROTA ]                         [ ASCENSIÓN (Edad 15) ]
  - Conquista enemiga.                          - Singularidad cuántica.
  - Consumo de recursos crítico.                - Victoria de Campaña.
             |                                             |
             +----------------------+----------------------+
                                    |
                                    v
                  +-----------------------------------+
                  |       PANTALLA DE EXTINCIÓN       |
                  |  - Conversión a Polvo Singular    |
                  |  - Registro en el Panteón         |
                  +-----------------------------------+
                                    |
                                    v
                  +-----------------------------------+
                  |         ÁRBOL DE LEGADO           |
                  |  - Compra de mejoras permanentes  |
                  |  - Desbloqueo de reliquias meta   |
                  +-----------------------------------+
```

### A. La Estructura de la Incursión (The Run)
*   **Generación de Mapa Procedimental**: Al iniciar una incursión, el grafo de la red de autopistas estelares (1D en el HUD, 2.5D en pantalla completa) se genera dinámicamente. La distribución de planetas habitables, cinturones de asteroides y la posición de los imperios hostiles de la IA cambia por completo en cada partida.
*   **Condición de Derrota (Lisis del Imperio)**: Si tu capital (o base inicial) es conquistada por invasores o sufre un colapso económico total (déficit de recursos insostenible durante 10 turnos seguidos en el HUD), la incursión finaliza.
*   **Condición de Victoria (Trascendencia)**: Completar la transición de la Edad 15 y asimilar la galaxia entera dentro de la Mente Enjambre cuántica.

### B. Meta-Moneda: Polvo de Singularidad (Singularity Dust)
Al finalizar una incursión (ya sea por colapso o por trascendencia), todo el progreso acumulado se convierte en **Polvo de Singularidad** (la divisa persistente de meta-progression). 
*   **Cálculo de Polvo obtenido**:
    $$\text{Polvo Generado} = \left( \sum_{i=1}^{\text{Edad Alcanzada}} 100 \cdot i \right) + \frac{\text{Población Total}}{1000} + \frac{\text{Tecnologías Desbloqueadas} \cdot \text{Turnos Sobrevividos}}{50}$$
*   Un multiplicador del **+50%** de Polvo se añade si la Incursión terminó en Trascendencia completa.

### C. Árbol de Legado Cósmico (Meta-Progression Upgrades)
El Polvo de Singularidad se gasta entre partidas en un menú de pantalla completa para comprar mejoras permanentes que facilitarán las siguientes incursiones:

1.  **Overclock del PC (PC Overclock)**: +2% de velocidad de generación de Energía ⚡ de CPU permanente por nivel (máx. 10 niveles).
2.  **Memoria Archival (Archival Memory)**: Empieza el juego con 1 tecnología aleatoria ya desbloqueada en la Edad de Piedra por nivel (máx. 3 niveles).
3.  **Conductos Mejorados**: Reduce la probabilidad de atascamiento logístico en un 5% por nivel (máx. 5 niveles).
4.  **Escudos del Núcleo**: Aumenta la salud base de la capital en un +10% por nivel (máx. 10 niveles).
5.  **Gobernadores Permanentes**: Desbloquea la capacidad de reclutar personajes históricos del Panteón que otorgan bonificaciones fijas a la economía.

### D. Reliquias de Era y Artefactos del Vacío
Durante una Incursión, el jugador puede descubrir **Reliquias** (modificadores pasivos que solo duran durante esa partida). Se consiguen al explorar ruinas, investigar anomalías complejas o derrotar imperios de la IA:

*   **Reliquias Comunes (Tier 1)**:
    *   *Fósil de Sílex Cristalino*: +15% de recolección de piedra en las edades 1 a 3.
    *   *Cáliz de Diezmo*: +10% de generación de fe en la Edad Media.
*   **Reliquias Épicas (Tier 2)**:
    *   *Turbina de Vapor Excesiva*: Aumenta la velocidad de tránsito del ferrocarril de la Edad Moderna en un +30% a costa de consumir un 10% más de Carbón.
    *   *Batería de Helio-3*: +20% de almacenamiento de Energía en la Edad Espacial.
*   **Artefactos del Vacío (Tier 3 - Game changers)**:
    *   *El Ojo del Vacío*: Revela instantáneamente todos los nodos inexplorados del mapa lineal, pero reduce el daño defensivo de las flotas en un 15% por la interferencia espacial.
    *   *Núcleo Singular Inestable*: Duplica toda la producción de Ciencia de la partida, pero la probabilidad de desastres y fallas cibernéticas en el HUD aumenta un +50%.

### E. Mutadores de Inestabilidad Temporal
Al cambiar de Edad, el jugador puede elegir de forma opcional un **Mutador de Inestabilidad** para esa era. Esto actúa como un nivel de dificultad adicional autoimpuesto a cambio de aumentar el Polvo de Singularidad obtenido al final de la partida:
*   *Llamarada Solar (Sun Flare)*: Duplica el coste de mantenimiento energético en la Edad Espacial, pero otorga un **+25% de velocidad científica** y **+40% de Polvo de Singularidad** acumulado durante esa época.
*   *Plaga Agrícola*: Reduce a la mitad la producción de cultivos en la Edad del Neolítico, pero duplica el valor del oro obtenido en caravanas.

---

## 🛠️ 6. Arquitectura Técnica y Optimización a Bajo Nivel (Windows & Rust)

Para alinearse con la **Ley Primera del Geist** (proteger la estabilidad y los recursos del ordenador del usuario), *TASK BAR 4X* implementa una arquitectura nativa ultra-ligera en **Rust**.

### A. Elección del Motor y Backends Gráficos
*   **Rechazo de Tauri/Chromium**: Tauri instancia procesos WebView2 basados en Chromium. Esto consume múltiples subprocesos y una base mínima de ~70-120MB de RAM.
*   **Rechazo de Unity/Godot**: Los motores de juego comerciales imponen una sobrecarga pesada debido a recolectores de basura, hilos activos de física y bucles de dibujo constantes.
*   **Arquitectura Adoptada**: Binario puro en **Rust nativo** utilizando bindings directos del sistema operativo (crates `windows` o `windows-sys`). La interfaz gráfica y las animaciones de la barra se renderizan mediante **Direct2D** (o **wgpu** con perfiles de bajo consumo energético).
*   **Consumo Objetivo**: **RAM < 15MB** en reposo, **CPU ~0.0%** en reposo y <1% durante simulación de fondo.

### B. Integración con la API de Windows (Win32)
*   **AppBar System (`SHAppBarMessage`)**: Se registra el HUD como una barra de herramientas del sistema (`ABM_NEW` con borde `ABE_BOTTOM`). Windows reserva físicamente 48 píxeles de la pantalla, forzando a todas las demás aplicaciones maximizadas a redimensionar su espacio de trabajo y evitar superposiciones.
*   **Estilos Extendidos de Ventana**:
    *   `WS_EX_NOACTIVATE`: Permite clics en el HUD sin quitar el foco de la ventana activa del usuario (IDE, navegador, etc.).
    *   `WS_EX_TOOLWINDOW`: Oculta el juego del conmutador Alt+Tab y de la barra de tareas real.
    *   `WS_EX_LAYERED`: Habilita la transparencia por píxel mediante mezcla de canal alfa para fundir la barra con el fondo.
*   **Mecanismo de Clics Passthrough**: Para zonas vacías o transparentes de la barra, la aplicación añade dinámicamente el estilo `WS_EX_TRANSPARENT`. Esto le indica al gestor de ventanas de Windows (`DWM`) que ignore el clic del ratón y lo envíe a los iconos del escritorio o a la barra de tareas del sistema subyecente. Al pasar el cursor sobre un botón o unidad del juego, el estilo se remueve para recibir la entrada del ratón.
*   **Soporte Multimonitor**: Se interceptan los mensajes `WM_SETTINGCHANGE` y `WM_DISPLAYCHANGE` en el procedimiento de ventana (`WndProc`) para recalcular el área del monitor donde reside la barra de tareas activa y reposicionar el juego dinámicamente.

### C. Estrategias de Optimización de CPU y GPU
*   **Bucle de Mensajes Reactivo (`WaitMessage`)**: Se detiene el bucle de renderizado clásico basado en redibujos continuos. Se utiliza la función bloqueante `WaitMessage()` en el hilo principal. Esto duerme el proceso del juego por completo hasta que ocurra una entrada de ratón, un temporizador interno o un mensaje del sistema, reduciendo el consumo de CPU en reposo a exactamente **0.0%**.
*   **Detección de Pantalla Completa (Modo Silencioso)**: Un temporizador de baja frecuencia (1 Hz) evalúa las dimensiones de la ventana activa en el sistema mediante `GetForegroundWindow` y `GetWindowRect`. Si se detecta una aplicación a pantalla completa (como un juego 3D o presentación), *TASK BAR 4X* entra en **Modo Suspendido**, deteniendo la simulación lógica, los hilos de renderizado y el temporizador gráfico para liberar CPU y GPU.
*   **Gamificación de Recursos basada en Hardware (Meta-OS)**:
    *   ⚡ **Energía**: Se genera proporcionalmente al porcentaje de uso real del CPU del PC (gamificación del procesador).
    *   ⚛️ **Ciencia**: Se asocia a la memoria RAM libre del ordenador (incentiva a tener el sistema optimizado).
    *   🛡️ **Aleaciones**: Generadas por la actividad de lectura/escritura de almacenamiento físico (E/S del SSD/HDD).

---

## 🚀 7. Integración del SDK de Steam e Economía Comunitaria

Para asegurar la viabilidad comercial y la rejugabilidad a largo plazo en Steam:

### A. Funciones de Integración Estándar
*   **Inicialización Segura**: Integración de la biblioteca `steamworks-rs` configurando `restart_app_if_necessary` al inicio del hilo principal para forzar la ejecución mediante el cliente de Steam.
*   **Guardado en la Nube**: Configuración de Steam Cloud para sincronizar de manera asíncrona la carpeta local de guardado atómico ubicada en `%LOCALAPPDATA%\TaskBar4X\saves\`.

### B. Mercado de la Comunidad (Drops de Inventario)
*   **Steam Inventory Service**: Se emplea el servicio de inventario sin servidor de Steamworks. Mediante la configuración de un archivo de esquema JSON (*Item Def Schema*), el juego otorga objetos cosméticos raros de forma aleatoria basados en tiempo de actividad del juego (ej. aspectos de naves steampunk, marcos de madera para el Orbe, avatares pixel-art). Estos objetos pueden comerciarse en el mercado comunitario de Steam de forma 100% nativa.

### C. Sistema Antitrampas Ligero (Lightweight Anti-Cheat)
*   **Ofuscación en Memoria**: Los recursos críticos del imperio (como aleaciones, energía y oro) no se guardan como enteros simples en memoria. Se almacenan dentro de una estructura `ValorProtegido` que encripta el dato real mediante operaciones XOR con claves aleatorias mutables que cambian en cada tick de actualización física.
*   **Detección de Depuración**: Directas llamadas a `IsDebuggerPresent()` para detectar software de alteración de memoria e inhabilitar la obtención de drops en el mercado de Steam en caso de manipulación.

---

## 📦 8. Pipeline de Desarrollo y Distribución del Juego

La construcción de un juego indie estable e interactivo se asegura mediante flujos de desarrollo ágiles y sistemas robustos de seguimiento de errores:

1. **Integración Continua (CI/CD)**: Compilación diaria automatizada con monitoreo de consumo de RAM y CPU en reposo para alertar inmediatamente si una nueva actualización introduce fugas de memoria o interrupciones en el bucle WndProc.
2. **Empaquetado mediante WiX Toolset**: Generación de un instalador MSI de usuario único (`Single User`) que coloca el ejecutable del juego en `%LOCALAPPDATA%\Programs\TaskBar4X`. No requiere elevación de privilegios de administrador (UAC), eliminando la fricción de instalación para el usuario final.
3. **Manejo de Errores Post-Mortem**: Integración de la biblioteca **Crashpad** (o Sentry local) para interceptar violaciones de acceso de memoria o excepciones graves no controladas en el equipo del usuario final. En caso de fallo, se genera un archivo de volcado minúsculo (`.dmp`) y se envía al servidor de desarrollo para su depuración con los símbolos PDB correspondientes.
