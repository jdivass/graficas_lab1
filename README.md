# Laboratorio 1 - Dibujado de Polígonos

## Descripción

Este proyecto implementa un framebuffer en Rust utilizando **raylib**, con el objetivo de dibujar y rellenar polígonos mediante algoritmos clásicos de gráficos por computadora.

Se implementaron las siguientes funcionalidades:

* Framebuffer personalizado.
* Dibujo de píxeles.
* Dibujo de líneas utilizando el algoritmo de Bresenham.
* Dibujo de polígonos conectando sus vértices.
* Relleno de polígonos mediante el algoritmo **Scanline Fill**.
* Exportación del resultado como imagen (`out.png`).

## Requisitos

* Rust
* Cargo
* Raylib 6.0.0

## Instalación

Clonar el repositorio:

```bash
git clone https://github.com/jdivass/graficas_lab1.git
cd <graficas_lab1>
```

Compilar el proyecto:

```bash
cargo build
```

Ejecutar:

```bash
cargo run
```

Al finalizar la ejecución se generará el archivo:

```
out.png
out.bmp
```

en la carpeta raíz del proyecto.

## Estructura del proyecto

```
src/
├── framebuffer.rs      # Implementación del framebuffer
├── line.rs             # Algoritmo de Bresenham
├── polygon.rs          # Dibujo y relleno de polígonos
└── main.rs             # Definición y renderizado de los polígonos
```

## Algoritmos implementados

### Bresenham

Se utiliza para rasterizar líneas de forma eficiente utilizando únicamente operaciones con enteros.

### Scanline Fill

El relleno de polígonos se realiza recorriendo cada línea horizontal de la imagen, calculando las intersecciones con los lados del polígono y pintando los segmentos comprendidos entre pares de intersecciones.

## Salida

La ejecución del programa genera una imagen (`out.png` y `out.bmp`) con los cinco polígonos en la rama main y con los poligonos individuales en sus respectivas ramas

## Autor

**Julián André Divas Silvestre**
