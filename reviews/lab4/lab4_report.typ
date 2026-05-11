#set page(
  paper: "a4",
  margin: (top: 2cm, bottom: 2cm, left: 3cm, right: 1.5cm),
)

#set text(
  size: 14pt,
  lang: "ru",
  font: "Times New Roman",
)

#set par(justify: true)

#show heading.where(level: 4): set text(weight: "regular", size: 1em)
#show heading.where(level: 4): set block(sticky: false)
#set heading(supplement: none)

#show link: set text(fill: blue)
#show link: underline

// --- 1. Титульный лист ---
#align(center)[
  #set text(size: 11pt)
  Министерство науки и высшего образования Российской Федерации \
  Федеральное государственное автономное образовательное учреждение высшего образования \
  #strong[«Национальный исследовательский университет ИТМО»] \
  #v(0.5em)
  Факультет Программной Инженерии и Компьютерной Техники
]

#v(1fr)

#align(center)[
  #set text(size: 16pt)
  #strong[Лабораторная работа №4] \
  #v(0.5em)
  #set text(size: 14pt)
  по дисциплине «Вычислительная математика» \
  Тема: «Аппроксимация функции методом наименьших квадратов»
]

#v(1em)

#align(center)[
  Вариант: #strong[6]
]

#v(1.2fr)

#align(right)[
  #block(width: 50%)[
    #align(right)[
      #strong[Преподаватель:] \
      Бострикова Дарья Константиновна \
      #v(1em)
      #strong[Выполнил:] \
      Пшеничников Артём Дмитриевич \
      #strong[Группа:] Р3207
    ]
  ]
]

#v(1fr)

#align(center + bottom)[
  Санкт-Петербург, 2026
]

#pagebreak()

#outline(depth: 3)

#set page(numbering: "1")

#pagebreak()

= Цель работы
Найти функцию, являющуюся наилучшим приближением заданной табличной функции по методу наименьших квадратов.

= Порядок выполнения работы
1. Изучение теоретических основ аппроксимации функций методом наименьших квадратов.
2. Программная реализация алгоритмов аппроксимации для различных типов функций (линейная, полиномиальная, экспоненциальная, логарифмическая, степенная).
3. Реализация графического интерфейса пользователя (UI) на базе библиотеки egui для визуализации результатов.
4. Проведение численных экспериментов и расчет статистических показателей качества аппроксимации ($S, epsilon, R^2$).
5. Анализ полученных результатов и выбор наилучшей модели.

= Рабочие формулы метода
Задача аппроксимации методом наименьших квадратов сводится к минимизации меры отклонения:
$ S = sum_(i=1)^n (phi(x_i) - y_i)^2 $

== Линейная аппроксимация ($phi(x) = a x + b$)
Система нормальных уравнений:
$
  cases(
    a sum x_i^2 + b sum x_i = sum x_i y_i,
    a sum x_i + b n = sum y_i
  )
$

== Полиномиальная аппроксимация 2-й степени ($phi(x) = a x^2 + b x + c$)
$
  cases(
    a sum x_i^4 + b sum x_i^3 + c sum x_i^2 = sum x_i^2 y_i,
    a sum x_i^3 + b sum x_i^2 + c sum x_i = sum x_i y_i,
    a sum x_i^2 + b sum x_i + c n = sum y_i
  )
$

#pagebreak()

= Вычисление заданного интеграла (табулированной функции)
Вариант 6: $y = (12x) / (x^4 + 6)$ на $[0, 2]$, $h = 0.2$.

== Табулирование функции
#align(center)[
  #table(
    columns: (auto, auto, auto, auto, auto, auto, auto, auto, auto, auto, auto),
    inset: 5pt,
    align: center + horizon,
    [$x$], [0.0], [0.2], [0.4], [0.6], [0.8], [1.0], [1.2], [1.4], [1.6], [1.8],
    [2.0], [$y$], [0.000], [0.400], [0.797], [1.175], [1.498], [1.714], [1.784], [1.707], [1.529],
    [1.309], [1.091],
  )
]

== Линейная аппроксимация ($y = a x + b$)
Вычислим суммы для $n = 11$:
$ sum x_i = 11.0, quad sum y_i = 13.013, quad sum x_i^2 = 15.4, quad sum x_i y_i = 14.659 $

Система:
$
  cases(
    15.4 a + 11.0 b = 14.659,
    11.0 a + 11.0 b = 13.013
  )
$

Решение: $a approx 0.374, b approx 0.809$. Уравнение: $y = 0.374 x + 0.809$.
СКО $epsilon approx 0.324$.

== Квадратичная аппроксимация ($y = a x^2 + b x + c$)
Решая систему нормальных уравнений (3х3), получаем коэффициенты:
$ a approx -0.835, b approx 2.044, c approx 0.141 $
Уравнение: $y = -0.835 x^2 + 2.044 x + 0.141$. СКО $epsilon approx 0.086$.

#pagebreak()

= Листинг программы
Исходный код лабораторной работы доступен в репозитории GitHub:
#link("https://github.com/q-artem/CompMath/tree/main/src/lab4")[github.com/q-artem/CompMath/src/lab4]

#show raw: it => align(center, block(fill: rgb("#EEEEEE"), inset: 5pt, radius: 8pt, text(it, size: 9pt)))

Реализация метода наименьших квадратов (линейная модель):
#raw(
  "fn solve_linear(points: &[Point]) -> Option<ApproximationResult> {
    let n = points.len() as f64;
    let mut sx = 0.0; let mut sy = 0.0; let mut sxx = 0.0; let mut sxy = 0.0;
    for p in points {
        sx += p.x; sy += p.y; sxx += p.x * p.x; sxy += p.x * p.y;
    }
    let det = sxx * n - sx * sx;
    if det.abs() < 1e-9 { return None; }
    let a = (sxy * n - sx * sy) / det;
    let b = (sxx * sy - sx * sxy) / det;

    Some(build_result(ModelType::Linear, vec![a, b], points, None))
}",
  lang: "rust",
  block: true,
)

= Результаты выполнения программы
== Графики аппроксимирующих функций

#figure(
  image("lab4_plot.png", width: 100%),
  caption: [График аппроксимирующих функций для данных из задания],
)

#figure(
  image("lab4_plot_2.png", width: 100%),
  caption: [Пример загрузки данных из файла],
)

#figure(
  image("lab4_plot_3.png", width: 100%),
  caption: [Еще один пример точек],
)

#pagebreak()

= Выводы
В ходе лабораторной работы был реализован метод наименьших квадратов для аппроксимации функций. Были исследованы линейная, полиномиальные (2-й и 3-й степени), экспоненциальная, логарифмическая и степенная модели. Для исходной функции варианта 6 наилучшее приближение показал полином 3-й степени, что подтверждается минимальным значением среднеквадратичного отклонения.
