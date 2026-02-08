#import "lib.typ": card
#show: card.with(color: sys.inputs.at("color", default: none))

#import "@preview/cetz:0.4.2": canvas
#import "@preview/cetz-plot:0.1.3": chart


#let languages = sys.inputs.at(
  "languages",
  default: (
    (name: "Rust", color: "#dea584", bytes: 121264),
    (name: "Typst", color: "#239dad", bytes: 463173),
    (name: "HTML", color: "#e34c26", bytes: 321322),
    (name: "Others", color: "#444", bytes: 321322),
  ),
)
#let total_language_bytes = (
  languages.fold(0, (acc, v) => acc + v.bytes)
)

= Most used languages

#grid(
  columns: 2,
  grid(
    columns: 3,
    align: (x, y) => horizon + if x == 2 { right } else { left },
    ..languages
      .map(lang => (
        block(
          height: .8em,
          width: .8em,
          fill: rgb(lang.color),
        ),
        lang.name,
        [#(calc.round(100 * lang.bytes / total_language_bytes, digits: 1)) %],
      ))
      .flatten(),
  ),

  canvas({
    let colors = languages.map(lang => rgb(lang.color))

    chart.piechart(
      languages.map(lang => (lang.name, lang.bytes)),
      label-key: none,
      value-key: 1,
      radius: 2,
      stroke: none,
      slice-style: colors,
    )
  }),
)
