# ⚡ RustFetch: Hyper-Fast System Information Engine 🦀

[🇺🇸 English Version](#-english-version) | [🇷🇺 Русская версия](#-русская-версия)

---

## 🇺🇸 English Version

**RustFetch** is a performance-first, zero-runtime-dependency system information engine written from scratch in Rust. It eliminates the fatal architectural flaws of legacy fetch utilities (like `neofetch` and `fastfetch`): heavy runtime disk I/O, split configuration assets, and unpredictable text-column drifting.

The project is built to deliver maximum execution velocity while freezing all layout parameters into a single, fully autonomous binary.

### 🏎️ Technical Matrix


| Technical Matrix | Original Fastfetch (C) | **RustFetch (Rust)** |
| :--- | :---: | :---: |
| **I/O Overhead** | Reads external ASCII files from disk on every execution | **Absolute Zero.** Asset states are kept directly inside RAM. |
| **Portability** | Requires auxiliary directories, themes, and configs | **Monolithic.** 100% portable single-file execution. |
| **Layout Stability** | Multi-byte glyphs and escape codes cause column sliding | **Anti-Drift Engine.** Perfect mathematical alignment. |
| **Code Maintenance** | Monolithic C codebase with complex macro trees | **Modular architecture.** Isolated logic via `config.rs`. |

### 🚀 Low-Level Engine Architecture

1. **Compile-Time Memory Mapping (`include_str!`)**
   RustFetch avoids filesystem reads during execution. Using static compiler constants, all raw distribution graphics (`ascii/*.txt`) are embedded straight into the machine code payload as immutable byte blocks. Moving or deleting files post-compilation cannot break the rendering engine.

2. **Embedded Fastfetch Palette Binder (`builtin.c`)**
   The compiler injects the official upstream `fastfetch` specifications file (`src/builtin.c`) directly into the parser block. A native token processing loop extracts C-macro identifiers (such as `FF_COLOR_FG_CYAN` or `FF_COLOR_FG_256`) and binds them directly to active console escape sequences. The engine recursively handles nested sequence signatures like `$1`, `${1}`, and dynamic reset bounds (`$%`, `$%{}`).

3. **True-Width Counter Loop**
   Standard text processing functions like `strlen` or `.len()` calculate raw byte weight instead of physical screen layout footprints. When multi-byte symbols or font glyphs are introduced, the layout shifts sideways. RustFetch passes all text lines through an isolated token-stripping loop that completely filters out ANSI escape styling codes, vector indices, and pipeline control gates. The trailing space padding calculates only what is physically drawn inside the active terminal cell matrix, anchoring the right data grid down to the exact pixel.

---

## 🇷🇺 Русская версия

**RustFetch** — высокопроизводительная утилита вывода системных данных, спроектированная на Rust с полным отказом от внешних зависимостей среды выполнения. Проект полностью устраняет архитектурные недостатки классических утилит (таких как `neofetch` и `fastfetch`): постоянные дисковые операции, разбросанные конфигурационные файлы и хаотичный сдвиг правого текстового блока.

Цель разработки — достижение максимальной скорости выполнения при жесткой фиксации параметров интерфейса внутри одного автономного бинарного файла.

### 🏎️ Сравнительный технический анализ


| Параметр сравнения | Оригинальный Fastfetch (C) | **RustFetch (Rust)** |
| :--- | :---: | :---: |
| **Нагрузка на I/O** | Считывает ASCII-шаблоны с диска при каждом вызове | **Полный ноль.** Данные логотипов извлекаются из ОЗУ. |
| **Автономность** | Требует наличия внешних папок, тем и схем | **Монолит.** Работает без внешнего окружения. |
| **Стабильность сетки** | Мультибайтовые глифы ломают ширину и уводят текст | **Движок Anti-Drift.** Математически ровное выравнивание. |
| **Поддержка кода** | Массивный репозиторий на Си со сложной логикой макросов | **Модульная структура.** Изолированная логика в `config.rs`. |

### 🚀 Компоненты внутренней архитектуры

1. **Статическое вшивание ресурсов (`include_str!`)**
   В RustFetch полностью ликвидировано чтение файловой системы во время выполнения. С помощью макросов компилятора графические макеты дистрибутивов (`ascii/*.txt`) преобразуются в неизменяемые блоки байтов в сегменте данных бинарного файла. Изменение структуры папок после сборки никак не влияет на работу отрисовщика.

2. **Парсер цветовых спецификаций (`builtin.c`)**
   Программа компилирует оригинальный файл конфигурации `src/builtin.c` от Fastfetch напрямую в собственный модуль обработки. Специальный цикл считывает структуры Си-макросов (`FF_COLOR_FG_CYAN`, `FF_COLOR_FG_256` и т.д.) и трансформирует их в прямые управляющие ANSI-коды терминала. Парсер корректно отрабатывает вложенные токены `$1`, `${1}` и маркеры сброса цвета (`$%`, `$%{}`).

3. **Алгоритм расчета видимого отступа Anti-Drift**
   Стандартные методы подсчета вроде `strlen` или `.len()` вычисляют объем занимаемой памяти в байтах, а не фактическое пространство на экране. Появление многобайтовых иконок шрифта неизбежно уводит разметку вбок. RustFetch решает эту проблему путем пропуска строки через фильтрующий цикл, который вырезает управляющие ANSI-коды окрашивания, переменные разметки и служебные ключи очистки слоя. Вычисление финального отступа пробелами происходит на основе физически отрисованных символов. Это фиксирует правый столбик спецификаций строго по вертикальной оси.
